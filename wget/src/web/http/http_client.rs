use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io::{self, Result, Read, Write};
use std::time::Duration;
use crate::log;
use crate::web::http::http_events::HttpEvents;
use crate::log::debuglog::productor::{DebugLogProductor, OnEventDebug};

pub struct HttpClient {
    pub stream: TcpStream,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub log_productor : DebugLogProductor,
}

impl HttpClient {
    pub fn from_url(url: &str) -> Result<Self> {
        let log_productor = DebugLogProductor::new();

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
            log_productor,
        })
    }

    
    pub fn send_http_head_request(&mut self ) {
        let host_header = format!("Host:{}", self.host);
        let http_head_request : HttpRequest = HttpRequest::HEAD {   
                                                                    path: &self.path,
                                                                    protocol_version: HttpProtocolVersion::Http11,
                                                                    request_head: &host_header 
                                                                };
        let request_content : String = http_head_request.to_string();

        self.log_productor.on_event(&HttpEvents::HTTPRequestSend(request_content.clone()));
        self.stream.write_all(&request_content.as_bytes());
    }
    pub fn send_http_get_request(&mut self) {
        let host_header = format!("Host:{}", self.host);
        let http_get_request: HttpRequest = HttpRequest::GET {
            path: &self.path,
            protocol_version: HttpProtocolVersion::Http11,
            request_head: &host_header,
        };
        let request_content: String = http_get_request.to_string();

        self.log_productor.on_event(&HttpEvents::HTTPRequestSend(request_content.clone()));
        self.stream.write_all(&request_content.as_bytes());
    }

}

