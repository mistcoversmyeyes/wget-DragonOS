pub use crate::events::http_events::HttpEvents;

pub use crate::log::on_event::{OnEventHttp};
pub struct DebugLogProductor {

}

impl DebugLogProductor {
    pub fn new() -> Self {
        DebugLogProductor {  }
    }
}

impl OnEventHttp for DebugLogProductor {
    fn on_event(&self, event : &HttpEvents){
        match event {
            HttpEvents::DebugModeSet => {
                println!("DEBUG: 设置为调试模式");
            }
            HttpEvents::URLAnalysing(url) =>{
                println!("DEBUG: 解析URL: {}",url);
            }
            HttpEvents::HostAnalysing(host_name) =>{
                println!("DEBUG: 正在解析主机 {}", host_name);
            }
            HttpEvents::IPAnalysed(ip) => {
                println!("DEBUG: IP地址为 {}", ip);
            }
            HttpEvents::ConnectionEstablishing { host, ip, port } => {
                println!("DEBUG: 正在连接 {}|{}|:{}", host, ip, port);
            }
            HttpEvents::ConnectionEstablished => {
                println!("DEBUG: 已建立连接");
            }
            HttpEvents::HTTPRequestSend(req) => {
                println!("DEBUG: 发送 HTTP 请求:\n{}", req);
            }
            HttpEvents::HTTPRequestReceived(resp) => {
                println!("DEBUG: 收到响应:\n{}", resp);
            }
            HttpEvents::StartDownload => {
                println!("DEBUG: 开始下载数据...");
            }
            HttpEvents::Downloading(bytes) => {
                println!("DEBUG: 已下载 {} 字节...", bytes);
            }
            HttpEvents::DownloadFinished(filename) => {
                println!("DEBUG: 下载完成，保存为 {}", filename);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_on_debug_mode_set_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::DebugModeSet);
    }
    #[test]
    fn test_on_url_analysing_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::URLAnalysing("http://example.com".to_string()));
    }
    #[test]
    fn test_on_host_analysing_event() {
        let productor = DebugLogProductor {};   
        productor.on_event(&HttpEvents::HostAnalysing("example.com".to_string()));
        productor.on_event(&HttpEvents::IPAnalysed("192.168.1.1".to_string()));
    }
    #[test]
    fn test_on_connection_establishing_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::ConnectionEstablishing {
            host: "example.com".to_string(),
            ip: "192.168.1.1".to_string(),
            port: 80,
        }); 

    }
    #[test]
    fn test_on_connection_established_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::ConnectionEstablished);
    }

    #[test]
    fn test_on_http_request_send_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::HTTPRequestSend("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".to_string()));
    }   

    #[test]
    fn test_on_http_request_received_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::HTTPRequestReceived("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html>...</html>".to_string()));     
    }

    #[test]
    fn test_on_start_download_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::StartDownload);
    }
    #[test]
    fn test_on_downloading_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::Downloading(1024));
    }
    #[test]
    fn test_on_download_finished_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&HttpEvents::DownloadFinished("output.txt".to_string()));
    }
}