pub mod args_define;                    // 表示当前目录模块具有公开的子模块 args_define.rs

pub use args_define::WgetArgs;          // 表示当前目录模块公开了子模块中的 WgetArgs 结构体

use crate::log::debuglog::productor::DebugLogProductor;
use crate::log::on_event::OnEventHttp;
use crate::log::events::http_events::HttpEvents;
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
        let mut content_stream = http_client.get_content_stream()?;

        let mut file_downloader = FileDownloader::default(&mut content_stream, self.args.clone(), content_length);
        
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

    /// 获取命令行参数
    pub fn get_args(&self) -> &WgetArgs {
        &self.args
    }
}
