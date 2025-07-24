pub mod args_define;                    // 表示当前目录模块具有公开的子模块 args_define.rs

pub use args_define::WgetArgs;          // 表示当前目录模块公开了子模块中的 WgetArgs 结构体

use crate::log::debuglog::productor::DebugLogProductor;
use crate::log::on_event::OnEventHttp;
use crate::log::events::HttpEvents;
use crate::web::http::http_client::HttpClient;         
use crate::file_writer::FileDownloader; 

pub struct CLI {
    args: WgetArgs,
}

impl CLI {
    /// 创建CLI实例，解析命令行参数
    pub fn new() -> Self {
        let args = WgetArgs::parse_args();
        CLI { args }
    }

    /// 执行下载任务
    /// 这个函数按照 命令行参数解析 --> http连接建立并发送请求获取文件流 --> 创建下载器将网络流（实现了 Read trait）拷贝到本地文件中。
    pub fn run(&self) -> Result<usize, Box<dyn std::error::Error>> {
        // 显示开始下载的信息
        println!("开始下载: {}", self.args.url);

        let mut http_client = HttpClient::from_url(&self.args.url)?;

        let content_length = http_client.get_file_length()
            .ok_or("无法获取文件大小")?;
            
        // 检查是否需要断点续传，如果需要则先检查本地文件大小 
        // TODO: (improvable):get_local_file_size() 发放本身就考虑到了启用了断点续传，但是本地没有文件或者有文件但是文件为空的情况吧
        // 也就是这里可以直接优化为下面一行的代码
        // let resume_from = self.get_local_file_size()?;
        let resume_from = if self.args.continue_download {
            // 这里需要实现获取本地文件大小的逻辑
            self.get_local_file_size()?
        } else {
            0
        };

        
        // 根据是否需要断点续传发送不同的HTTP请求
        let mut content_stream = if resume_from > 0 {
            println!("从第 {} 字节处开始断点续传", resume_from);
            // TODO(bugfix):似乎这里请求多了，因为 get_content_stream() 里面还会再调用一次 send_http_get_request()方法。
            http_client.send_http_get_request_with_range(resume_from); 
            http_client.get_content_stream()?
        } else {
            http_client.get_content_stream()?
        };

        // 使用智能构造函数创建下载器
        let mut file_downloader = match FileDownloader::smart_new(&mut content_stream, self.args.clone(), content_length) {
            Ok(downloader) => downloader,
            Err(e) => {
                println!("创建下载器失败: {}", e);
                return Err(Box::new(e));
            }
        };
        
        match file_downloader.download() {
            Ok(downloaded_size) => {
                DebugLogProductor::get_instance().on_event(&HttpEvents::DownloadFinished(format!("下载完成，共下载 {downloaded_size}B")));
                Ok(downloaded_size)
            }
            Err(e) => {
                println!("下载失败: {}", e);
                Err(Box::new(e))
            }
        }
    }
    
    /// 获取本地文件的当前大小（用于断点续传）
    fn get_local_file_size(&self) -> Result<usize, Box<dyn std::error::Error>> {
        use std::{env, path::PathBuf};
        
        // 获取文件路径（与FileDownloader中相同的逻辑）
        let directory_prefix: PathBuf = match self.args.directory_prefix.as_ref() {
            Some(prefix) => PathBuf::from(prefix),
            None => env::current_dir()?,
        };
        
        let file_name = match self.args.output_file_name.as_ref() {
            Some(name) => name.clone(),
            None => {
                self.args.url
                .rsplit('/')
                .next()
                .ok_or("自动获取文件名失败")?
                .to_string()
            },
        };
        
        let file_path = directory_prefix.join(file_name);
        
        if file_path.exists() {
            let metadata = std::fs::metadata(file_path)?;
            Ok(metadata.len() as usize)
        } else {
            Ok(0)
        }
    }

    /// 获取命令行参数
    pub fn get_args(&self) -> &WgetArgs {
        &self.args
    }
}
