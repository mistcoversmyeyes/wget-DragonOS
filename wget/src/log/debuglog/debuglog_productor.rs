use super::debuglog_events::DebugLogEvents;


trait DebugOnEvent {
    fn on_event(&self, event : &DebugLogEvents);
}
pub struct DebugLogProductor {

}

impl DebugOnEvent for DebugLogProductor {
    fn on_event(&self, event : &DebugLogEvents){
        match event {
            DebugLogEvents::DebugModeSet => {
                println!("DEBUG: 设置为调试模式");
            }
            DebugLogEvents::URLAnalysing(url) =>{
                println!("DEBUG: 解析URL: {}",url);
            }
            DebugLogEvents::HostAnalysing(host_name) =>{
                println!("DEBUG: 正在解析主机 {}", host_name);
            }
            DebugLogEvents::IPAnalysed(ip) => {
                println!("DEBUG: IP地址为 {}", ip);
            }
            DebugLogEvents::ConnectionEstablishing { host, ip, port } => {
                println!("DEBUG: 正在连接 {}|{}|:{}", host, ip, port);
            }
            DebugLogEvents::ConnectionEstablished => {
                println!("DEBUG: 已建立连接");
            }
            DebugLogEvents::HTTPRequestSend(req) => {
                println!("DEBUG: 发送 HTTP 请求:\n{}", req);
            }
            DebugLogEvents::HTTPRequestReceived(resp) => {
                println!("DEBUG: 收到响应:\n{}", resp);
            }
            DebugLogEvents::StartDownload => {
                println!("DEBUG: 开始下载数据...");
            }
            DebugLogEvents::Downloading(bytes) => {
                println!("DEBUG: 已下载 {} 字节...", bytes);
            }
            DebugLogEvents::DownloadFinished(filename) => {
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
        productor.on_event(&DebugLogEvents::DebugModeSet);
    }
    #[test]
    fn test_on_url_analysing_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::URLAnalysing("http://example.com".to_string()));
    }
    #[test]
    fn test_on_host_analysing_event() {
        let productor = DebugLogProductor {};   
        productor.on_event(&DebugLogEvents::HostAnalysing("example.com".to_string()));
        productor.on_event(&DebugLogEvents::IPAnalysed("192.168.1.1".to_string()));
    }
    #[test]
    fn test_on_connection_establishing_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::ConnectionEstablishing {
            host: "example.com".to_string(),
            ip: "192.168.1.1".to_string(),
            port: 80,
        }); 

    }
    #[test]
    fn test_on_connection_established_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::ConnectionEstablished);
    }

    #[test]
    fn test_on_http_request_send_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::HTTPRequestSend("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".to_string()));
    }   

    #[test]
    fn test_on_http_request_received_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::HTTPRequestReceived("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html>...</html>".to_string()));     
    }

    #[test]
    fn test_on_start_download_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::StartDownload);
    }
    #[test]
    fn test_on_downloading_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::Downloading(1024));
    }
    #[test]
    fn test_on_download_finished_event() {
        let productor = DebugLogProductor {};
        productor.on_event(&DebugLogEvents::DownloadFinished("output.txt".to_string()));
    }
}