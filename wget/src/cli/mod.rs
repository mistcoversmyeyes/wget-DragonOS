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
        DebugLogProductor::get_instance().on_event(&HttpEvents::StartToProcessDownload(self.args.url.clone()));

        // 采用统一逻辑对待全新下载和断点续传。
        let resume_from = self.get_local_content_length()?;

        let mut http_client = HttpClient::from_url(&self.args.url)?;

        // TODO(web): 添加检测服务器是否支持断点续传的检测

        let file_length = http_client.get_file_length()
            .ok_or("无法获取文件大小")?;
        
        // 直接将决定权交给网络模块
        let mut content_stream = http_client.get_content_stream(resume_from)?;

        // 使用智能构造函数创建下载器
        // 在创建下载器的时候判断断点续传有关参数是否设置正确，断点续传共有四种可能情况
        let mut file_downloader = match FileDownloader::smart_new(&mut content_stream, self.args.clone(), file_length) {
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
    /// 如果给定路径下没有文件，那么返回 Ok(0)
    /// 如果给定路径下已经存在文件，那么返回 Ok(file_len)
    fn get_local_content_length(&self) -> Result<usize, Box<dyn std::error::Error>> {
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
            let file_size = metadata.len() as usize;
            DebugLogProductor::get_instance().on_event(&HttpEvents::GetLocalContentlength(file_size));
            Ok(file_size)
        } else {
            DebugLogProductor::get_instance().on_event(&HttpEvents::GetLocalContentlength(0));
            Ok(0)
        }
    }

    /// 获取命令行参数
    pub fn get_args(&self) -> &WgetArgs {
        &self.args
    }
}
