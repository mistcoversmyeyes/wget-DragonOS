mod parameter_process;
mod log;
mod web;
mod file_writer;
mod events;
use parameter_process::WgetArgs;

use crate::{events::http_events::HttpEvents, file_writer::FileDownloader, log::{debuglog::productor::DebugLogProductor, on_event::OnEventHttp}, web::http::http_client::HttpClient};


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
