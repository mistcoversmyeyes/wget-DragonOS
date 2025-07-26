
// TODO: 寻找整个下载过程中所有能够出现的合法的事件，将其统一声明在这里
pub enum HttpEvents {
    /// 当前日志模式被设置为 Debug 模式
    DebugModeSet,
    StartToProcessDownload(String),
    GetLocalContentlength(usize),  
    URLAnalysing(String),
    HostAnalysing(String),
    IPAnalysed(String),
    ConnectionEstablishing {host : String, ip: String , port : u16},
    ConnectionEstablished,
    HTTPRequestSend(String),
    /// 发送HEAD请求获取文件元信息
    HeadRequestSent,
    ContentLengthAnalysed(usize),
    ContentStreamGeted,
    StartDownload,
    Downloading(usize),
    /// 周期性下载进度更新，包含当前已下载字节数、总大小和下载速度(字节/秒)
    DownloadProgress { 
        downloaded: usize, 
        total: usize, 
        speed_bps: f64 
    },
    DownloadFinished(String),
    /// 检测到本地文件，准备断点续传
    ResumeDownloadDetected { local_size: usize, total_size: usize },
    /// 本地文件不存在，开始全新下载
    NewDownloadStarted,
    /// 本地文件已完整，无需下载
    FileAlreadyComplete { file_size: usize },
    /// 未启用断点续传但文件已存在的错误情况
    FileExistsWithoutResume { file_path: String },
}

pub enum FTPEvents {
    // TODO: 定义 FTP 下载过程中的所有事件
}