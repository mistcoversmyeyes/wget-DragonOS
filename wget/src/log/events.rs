

pub enum HttpEvents {
    DebugModeSet,   
    URLAnalysing(String),
    HostAnalysing(String),
    IPAnalysed(String),
    ConnectionEstablishing {host : String, ip: String , port : u16},
    ConnectionEstablished,
    HTTPRequestSend(String),
    ContentLengthAnalysed(usize),
    ContentStreamGeted,
    StartDownload,
    Downloading(usize),
    DownloadFinished(String),
}

pub enum FTPEvents {
    // TODO: 定义 FTP 下载过程中的所有事件
}