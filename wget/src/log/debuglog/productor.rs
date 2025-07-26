pub use crate::log::events::HttpEvents;
pub use crate::log::on_event::{OnEventHttp};
use std::sync::OnceLock;

pub struct DebugLogProductor {
    // 移除自引用字段，单例模式不需要实例持有自己的引用
}

impl DebugLogProductor {
    // 私有构造函数，防止外部直接创建实例
    fn new() -> Self {
        DebugLogProductor {}
    }

    // 线程安全的单例获取方法
    pub fn get_instance() -> &'static DebugLogProductor {
        static INSTANCE: OnceLock<DebugLogProductor> = OnceLock::new();
        INSTANCE.get_or_init(|| DebugLogProductor::new())
    }
}

impl OnEventHttp for DebugLogProductor {
    fn on_event(&self, event : &HttpEvents){
        match event {
            HttpEvents::DebugModeSet => {
                println!("DEBUG: 设置为调试模式");
            }
            HttpEvents::StartToProcessDownload(url) => {
                println!("DEBUG: 开始处理下载任务: {}", url);
            }
            HttpEvents::GetLocalContentlength(size) => {
                println!("DEBUG: 本地文件大小: {}B", size);
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
            HttpEvents::HeadRequestSent => {
                println!("DEBUG: 发送 HEAD 请求获取文件元信息");
            }
            HttpEvents::ContentLengthAnalysed(file_size) => {
                println!("DEBUG: 文件大小为: {}B", file_size);
            }
            HttpEvents::ContentStreamGeted => {
                println!("DEBUG: 已解析要下载的文件流");
            }
            HttpEvents::StartDownload => {
                println!("DEBUG: 开始下载数据...");
            }
            HttpEvents::Downloading(bytes) => {
                println!("DEBUG: 已下载 {} 字节...", bytes);
            }
            HttpEvents::DownloadProgress { downloaded, total, speed_bps } => {
                let progress_percent = if *total > 0 {
                    (*downloaded as f64 / *total as f64) * 100.0
                } else {
                    0.0
                };
                println!("下载进度: {}/{} 字节 ({:.1}%) - 速度: {:.0} B/s", 
                         downloaded, total, progress_percent, speed_bps);
            }
            HttpEvents::DownloadFinished(filename) => {
                println!("DEBUG: 下载完成，保存为 {}", filename);
            }
            HttpEvents::ResumeDownloadDetected { local_size, total_size } => {
                println!("DEBUG: 检测到本地文件，从 {} 字节处继续下载 (总大小: {}B)", local_size, total_size);
            }
            HttpEvents::NewDownloadStarted => {
                println!("DEBUG: 开始全新下载");
            }
            HttpEvents::FileAlreadyComplete { file_size } => {
                println!("DEBUG: 本地文件已完整 ({}B)，无需继续下载", file_size);
            }
            HttpEvents::FileExistsWithoutResume { file_path } => {
                println!("DEBUG: 文件已存在但未启用断点续传: {}", file_path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_on_debug_mode_set_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::DebugModeSet);
    }
    
    #[test]
    fn test_on_url_analysing_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::URLAnalysing("http://example.com".to_string()));
    }
    
    #[test]
    fn test_on_host_analysing_event() {
        let productor = DebugLogProductor::get_instance();   
        productor.on_event(&HttpEvents::HostAnalysing("example.com".to_string()));
        productor.on_event(&HttpEvents::IPAnalysed("192.168.1.1".to_string()));
    }
    
    #[test]
    fn test_on_connection_establishing_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::ConnectionEstablishing {
            host: "example.com".to_string(),
            ip: "192.168.1.1".to_string(),
            port: 80,
        }); 
    }
    
    #[test]
    fn test_on_connection_established_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::ConnectionEstablished);
    }

    #[test]
    fn test_on_http_request_send_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::HTTPRequestSend("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".to_string()));
    }   

    #[test]
    fn test_on_content_lenght_analysed() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&&HttpEvents::ContentLengthAnalysed(1234));     
    }

    #[test]
    fn test_on_start_download_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::StartDownload);
    }
    
    #[test]
    fn test_on_downloading_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::Downloading(1024));
    }
    
    #[test]
    fn test_on_download_finished_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::DownloadFinished("output.txt".to_string()));
    }

    #[test]
    fn test_on_start_to_process_download_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::StartToProcessDownload("http://example.com/file.txt".to_string()));
    }

    #[test]
    fn test_on_get_local_contentlength_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::GetLocalContentlength(1024));
    }

    #[test]
    fn test_on_head_request_sent_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::HeadRequestSent);
    }

    #[test]
    fn test_on_resume_download_detected_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::ResumeDownloadDetected { 
            local_size: 512, 
            total_size: 1024 
        });
    }

    #[test]
    fn test_on_new_download_started_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::NewDownloadStarted);
    }

    #[test]
    fn test_on_file_already_complete_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::FileAlreadyComplete { file_size: 1024 });
    }

    #[test]
    fn test_on_file_exists_without_resume_event() {
        let productor = DebugLogProductor::get_instance();
        productor.on_event(&HttpEvents::FileExistsWithoutResume { 
            file_path: "/path/to/file.txt".to_string() 
        });
    }
}