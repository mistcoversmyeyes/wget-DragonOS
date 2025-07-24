mod cli;           // 命令行接口：解析命令行参数，调度使用下层模块完成任务
mod log;           // 日志模块 ✓
mod web;           // 网络模块 ✓  
mod file_writer;   // 文件I/O模块 ✓ 
// 未来可能添加：
// mod recursive;  // 递归模块 (HTML解析、URL队列等)

use cli::WgetArgs;

use crate::{
    log::events::http_events::HttpEvents, file_writer::FileDownloader,
    log::{debuglog::productor::DebugLogProductor, on_event::OnEventHttp},
    web::http::http_client::HttpClient
};


fn main() {
    let args = WgetArgs::parse_args();

    let mut http_client = HttpClient::from_url(&args.url).expect("http客户端创建失败");

    let content_length = http_client.get_file_length().unwrap();
    let mut content_stream = http_client.get_content_stream().unwrap();

    let mut file_dowloader : FileDownloader = FileDownloader::default(&mut content_stream, args, content_length);
    if let Ok(downloaded_size) = file_dowloader.download() {
        ()
    } 
    else {
        println!("下载失败");
        ()
    }
    // TODO: 修复下载完成后无法自动退出的问题
}
