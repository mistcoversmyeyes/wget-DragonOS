use std::{env, fs::File, path::PathBuf, time::{Duration, Instant}};

use crate::{
    log::{
        debuglog::productor::DebugLogProductor,
        on_event::OnEventHttp,
        events::HttpEvents,
    },
    cli::WgetArgs};
use std::io::{Read, Seek, SeekFrom, Write};


pub struct FileDownloader<'a> {
    source : &'a mut dyn Read,
    destination : File,
    file_length: usize,
    offset : usize,            //为断点续传预留的

}


impl<'a> FileDownloader<'a> {
    /// 全新下载构造函数，用于从头开始的下载
    /// 给定一个已经去除了响应头内容的 网络流 `stream`, 命令行参数解析结果 `para` 和 获取到的文件内容大小 `file_length`
    /// 创建一个包含源网络流，目的文件，文件大小，已经下载的大小(offset)的执行下载工作的结构体。
    /// 默认从命令行参数的 <URL> 获取文件名，文件名为 <URL>字段的最后一个 '/' 后面的内容
    /// ``` bash
    /// # pwd = /home/username/tools
    /// wget http://google.com/index.html
    /// ```
    /// 对于以上示例，保存文件的路径为 "/home/username/tools/index.html"
    /// 
    pub fn new(stream: &'a mut dyn Read, para: WgetArgs, file_length: usize) -> Self {
        // 尝试获取命令行选项 '-P' 传入的文件保存目录，支持相对路径与绝对路径，支持win/linux两大平台的路径
        // 如果没有传入的路径，默认为运行 wget 的时候所在的工作目录
        let directory_prefix: PathBuf = match para.directory_prefix {
            Some(prefix) => PathBuf::from(prefix),
            None => env::current_dir().unwrap(),
        };
        // 尝试获取 命令行选项 '-O' 传入的文件名，如果没有传入的文件名，那么从 url 中获取
        let file_name = match para.output_file_name {
            Some(name) => name ,
            None => {
                para.url
                .rsplit('/')
                .next()
                .expect("自动获取文件名失败")
                .to_string()
            },
        };
        
        // 拼接目录前缀和文件名为完整文件路径
        let file_path: PathBuf = directory_prefix.join(file_name);

        // 创建文件
        let destination: File = File::create_new(file_path)
                                    .expect("创建本地文件失败，请检查指定的路径是否合法。\n若路径合法，请检查是否已经存在文件，不允许覆盖下载。");
        
        FileDownloader {
            source: stream,
            destination,
            file_length,
            offset: 0,
        }
    }
    
    /// 智能构造函数，根据命令行参数自动选择全新下载或断点续传
    /// 如果设置了 `-c` 或 `--continue` 参数且本地文件存在，则进行断点续传
    /// 否则进行全新下载
    pub fn smart_new(stream: &'a mut dyn Read, para: WgetArgs, file_length: usize) -> std::io::Result<Self> {
        // 获取文件路径
        let directory_prefix: PathBuf = match para.directory_prefix.as_ref() {
            Some(prefix) => PathBuf::from(prefix),
            None => env::current_dir().unwrap(),
        };
        
        let file_name = match para.output_file_name.as_ref() {
            Some(name) => name.clone(),
            None => {
                para.url
                .rsplit('/')
                .next()
                .expect("自动获取文件名失败")
                .to_string()
            },
        };
        
        let file_path: PathBuf = directory_prefix.join(file_name);
        
        // 预检查：确保目标目录存在且可写
        if let Some(parent_dir) = file_path.parent() {
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir)?;
            }
        }
        
        // 预检查：如果目标路径存在但是目录，返回错误
        if file_path.exists() && file_path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::IsADirectory,
                format!("目标路径是目录: {}", file_path.display())
            ));
        }
        
        // 根据是否启用断点续传和文件是否存在来决定行为
        // 使用 match statement 清晰地处理所有情形
        let (destination, offset) = match (para.continue_download, file_path.exists()) {
            // 情形1: 启用断点续传 + 文件存在 → 断点续传
            (true, true) => {
                let file = std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&file_path)?;
                
                let metadata = file.metadata()?;
                let current_size = metadata.len() as usize;
                
                // 检查文件完整性
                if current_size >= file_length {
                    DebugLogProductor::get_instance().on_event(&HttpEvents::FileAlreadyComplete { file_size: current_size });
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "本地文件已完整，无需继续下载"
                    ));
                }
                
                DebugLogProductor::get_instance().on_event(&HttpEvents::ResumeDownloadDetected { 
                    local_size: current_size, 
                    total_size: file_length 
                });
                (file, current_size)
            },
            
            // 情形2: 启用断点续传 + 文件不存在 → 全新下载（断点续传退化为普通下载）
            (true, false) => {
                let file = File::create(&file_path)?;
                DebugLogProductor::get_instance().on_event(&HttpEvents::NewDownloadStarted);
                (file, 0)
            },
            
            // 情形3: 未启用断点续传 + 文件不存在 → 全新下载
            (false, false) => {
                let file = File::create(&file_path)?;
                DebugLogProductor::get_instance().on_event(&HttpEvents::NewDownloadStarted);
                (file, 0)
            },
            
            // 情形4: 未启用断点续传 + 文件存在 → 报错（避免意外覆盖）
            (false, true) => {
                DebugLogProductor::get_instance().on_event(&HttpEvents::FileExistsWithoutResume { 
                    file_path: file_path.to_string_lossy().to_string() 
                });
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "文件已存在，使用 -c 选项进行断点续传，或删除现有文件"
                ));
            },
        };
        
        Ok(FileDownloader {
            source: stream,
            destination,
            file_length,
            offset,
        })
    }

    pub fn download(&mut self) -> std::io::Result<usize> {
        // 定位到断点
        self.destination.seek(SeekFrom::Start(self.offset as u64))?;
        let mut total_written: usize = 0usize;
        let mut buf = [0u8; 8192];

        // 开始下载内容
        DebugLogProductor::get_instance().on_event(&HttpEvents::StartDownload);
        
        // 时间和速度跟踪变量
        let start_time = Instant::now();
        let mut last_update_time = start_time;
        let mut last_downloaded = self.offset;
        let update_interval = Duration::from_secs(1); // 1秒更新间隔
        
        loop {
            let n = self.source.read(&mut buf)?;
            if n == 0 {
                break;
            }
            self.destination.write_all(&buf[..n])?;
            
            total_written += n as usize;
            let current_downloaded = self.offset + total_written;
            let now = Instant::now();
            
            // 检查是否需要更新进度（每秒一次）
            if now.duration_since(last_update_time) >= update_interval {
                let time_elapsed = now.duration_since(last_update_time).as_secs_f64();
                let bytes_since_last_update = current_downloaded - last_downloaded;
                let speed_bps = bytes_since_last_update as f64 / time_elapsed;
                
                // 发送进度更新事件
                DebugLogProductor::get_instance().on_event(&HttpEvents::DownloadProgress {
                    downloaded: current_downloaded,
                    total: self.file_length,
                    speed_bps,
                });
                
                last_update_time = now;
                last_downloaded = current_downloaded;
            }
        }
        
        // 下载完成后立即发送最终进度（确保显示100%）
        let final_downloaded = self.offset + total_written;
        let final_time_elapsed = Instant::now().duration_since(start_time).as_secs_f64();
        let average_speed = if final_time_elapsed > 0.0 {
            total_written as f64 / final_time_elapsed
        } else {
            0.0
        };
        
        DebugLogProductor::get_instance().on_event(&HttpEvents::DownloadProgress {
            downloaded: self.offset + total_written,
            total: self.file_length,
            speed_bps: average_speed,
        });
        
        Ok(total_written)
    }
}