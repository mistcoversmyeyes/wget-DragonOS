use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::io::{self, Result, Read, Write};
use std::time::Duration;
use crate::log;
use crate::web::http_events::HttpEvents;
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

    
    pub fn send_http_request(&mut self) -> Result<()> {
        let http_get_request = format!("GET {} HTTP/1.1\r\n\
                                                Host:{}\r\n"
                                                , self.path
                                                , self.host);

        self.log_productor.on_event(&HttpEvents::HTTPRequestSend(http_get_request.clone()));

        self.stream.write_all(http_get_request.as_bytes())?;

        self.stream.write_all(b"\r\n")?;
        Ok(())
    }



}

