use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io::{self, Result, Read, Write, Cursor};
use std::time::Duration;
use crate::log::on_event::OnEventHttp;
use crate::events::http_events::HttpEvents;
use crate::log::debuglog::productor::{DebugLogProductor};
use super::http_requests::{HttpProtocolVersion,HttpRequest};

pub struct HttpClient {
    pub stream: TcpStream,  // 打开的tcp连接
    pub host: String,       // 使用域名标识的主机名
    pub port: u16,          // 连接的端口号
    pub path: String,       // 资源路径
}

impl HttpClient {
    pub fn from_url(url: &str) -> Result<Self> {
        let log_productor = DebugLogProductor::get_instance();

        log_productor.on_event(&HttpEvents::DebugModeSet);


        log_productor.on_event(&HttpEvents::URLAnalysing(url.to_string()));
        // 假设 url 形如 "http://example.com/path"
        // 去除 "http://" 前缀
        let url = url.trim_start_matches("http://");

        // 分离 主机名:端口号 和 资源路径
        let (host_port, path) = match url.split_once('/') {
            Some((h, p)) => (h, format!("/{}", p)),
            None => (url, "/".to_string()),
        };

        // 分离 主机名 和 端口号
        let (domain, port) = match host_port.split_once(':') {
            Some((d, p)) => (d.to_string(), p.parse::<u16>().unwrap_or(80)),
            None => (host_port.to_string(), 80),
        };


        log_productor.on_event(&HttpEvents::HostAnalysing(domain.clone()));

        let addr : SocketAddr = format!("{}:{}", domain, port)
        .to_socket_addrs()?
        .next()
        .expect(&format!("无法解析域名 {}:{}", domain, port));

        log_productor.on_event(&HttpEvents::IPAnalysed(addr.ip().to_string()));

        log_productor.on_event(&HttpEvents::ConnectionEstablishing {
            host: domain.clone(),
            ip: addr.ip().to_string(),
            port: addr.port(),
        });

        let stream = TcpStream::connect(addr)?;
        log_productor.on_event(&HttpEvents::ConnectionEstablished);

        Ok(HttpClient {
            stream,
            host: domain,
            port,
            path,
        })
    }

    
    pub fn send_http_head_request(&mut self ) {
        let host_header = format!("Host: {}", self.host);
        let http_head_request : HttpRequest = HttpRequest::HEAD {   
                                                                    path: &self.path,
                                                                    protocol_version: HttpProtocolVersion::Http11,
                                                                    request_head: &host_header 
                                                                };
        let request_content : String = http_head_request.to_string();

        DebugLogProductor::get_instance().on_event(&HttpEvents::HTTPRequestSend(request_content.clone()));
        self.stream.write_all(&request_content.as_bytes());
    }
    pub fn send_http_get_request(&mut self) {
        let host_header = format!("Host: {}", self.host);
        let http_get_request: HttpRequest = HttpRequest::GET {
            path: &self.path,
            protocol_version: HttpProtocolVersion::Http11,
            request_head: &host_header,
        };
        let request_content: String = http_get_request.to_string();

        DebugLogProductor::get_instance().on_event(&HttpEvents::HTTPRequestSend(request_content.clone()));
        self.stream.write_all(&request_content.as_bytes());
    }

    /// 解析 Http 响应报文以获取要下载的文件的元信息,包括 Content-type, charset
    /// 假设Http的 响应格式如下
    /// ```
    /// HTTP/1.1 200 OK
    /// Content-Type: image/png
    /// Content-Length: 12345
    /// Last-Modified: Wed, 21 Oct 2015 07:28:00 GMT
    /// Connection: keep-alive
    /// // etc.
    /// // 以上 Headers 的各个字段顺序仅供参考，解析的时候需要使用正则表达式匹配
    /// ```
    pub fn get_file_length (&mut self) -> Option<usize> {
        // 发送获取 响应头部 的请求信息
        self.send_http_head_request();

        // 创建接收缓冲区
        let mut buf: [u8; 4096] = [0u8; 4096];

        // 创建需要将接收到的响应暂存的位置
        let mut response: String = String::new();
        loop {
            match self.stream.read(&mut buf) {
                Ok(n) => {
                    // 检查是否读到了末尾，http 响应行 + 响应头 以 \r\n\r\n 结束
                    let cur_str = &*String::from_utf8_lossy(&buf[..n]);

                    if cur_str.contains("\r\n\r\n") {
                        // 将读取到的数据追加到 response 字符串中,并结束读取
                        response.push_str(cur_str);
                        break;
                    }
                    else {
                        // 将读取到的数据追加到 response 字符串中
                        response.push_str(cur_str);
                    }
                }
                Err(_e) => {
                    break;
                }
            }
        }

        // 解析 Content-Length
        for line in response.lines() {
            if line.to_ascii_lowercase().starts_with("content-length:") {
                if let Some(len_str) = line.split(':').nth(1) {
                    if let Ok(len) = len_str.trim().parse::<usize>() {
                        return Some(len);
                    }
                }
            }
        }
        None
    }
    /// 返回一个实现了 Read trait 的类型，从响应体开始读取 HTTP 响应内容
    /// 注意：调用前应先发送 GET 请求
    pub fn get_content_stream(&mut self) -> io::Result<impl Read + '_> {
        // 发送获取 文件 的请求信息
        self.send_http_get_request();


        // 读取响应头，找到 \r\n\r\n 的分界点
        let mut buf = Vec::with_capacity(8192);
        let mut header_end = None;
        let mut tmp = [0u8; 1024];

        while header_end.is_none() {
            let n = self.stream.read(&mut tmp)?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n]);
            if let Some(pos) = twoway::find_bytes(&buf, b"\r\n\r\n") {
                header_end = Some(pos + 4);
            }
        }

        let body_start = header_end.unwrap_or(buf.len());
        // 剩余数据为响应体的开头部分
        let mut body = buf.split_off(body_start);

        // 创建一个组合流，先读 body，再读 self.stream
        let cursor = Cursor::new(body);
        Ok(cursor.chain(&mut self.stream))
    }

}


#[cfg(test)]
mod tests {
    
use std::io::{Read, Write};
use std::net::{TcpListener};
use std::thread;
use std::io::{Cursor, Chain};

    use super::*;

    // Helper function to start a simple TCP server for testing
    fn start_test_server(response: &'static str) -> std::net::SocketAddr {

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf);
                let _ = stream.write_all(response.as_bytes());
            }
        });

        addr
    }

    #[test]
    fn test_send_http_get_request() {
        let response: &'static str = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        let addr: SocketAddr = start_test_server(response);

        let url: String = format!("http://{}", addr);
        let mut client: HttpClient = HttpClient::from_url(&url).expect("Failed to create HttpClient");
        client.send_http_get_request();

        let mut buf: [u8; 1024] = [0u8; 1024];
        let n: usize = client.stream.read(&mut buf).unwrap();
        let resp_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf[..n]);
        assert!(resp_str.contains("HTTP/1.1 200 OK"));
    }

    #[test]
    fn test_send_http_head_request() {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        let addr = start_test_server(response);

        let url = format!("http://{}", addr);
        let mut client = HttpClient::from_url(&url).expect("Failed to create HttpClient");
        client.send_http_head_request();

        let mut buf = [0u8; 1024];
        let n = client.stream.read(&mut buf).unwrap();
        let resp_str = String::from_utf8_lossy(&buf[..n]);
        assert!(resp_str.contains("HTTP/1.1 200 OK"));
    }

    #[test]
    fn test_get_file_length() {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: 12345\r\n\r\n";
        let addr = start_test_server(response);

        let url = format!("http://{}", addr);
        let mut client = HttpClient::from_url(&url).expect("Failed to create HttpClient");
        client.send_http_head_request();

        let len = client.get_file_length();
        assert_eq!(len , Some(12345));
    }
}