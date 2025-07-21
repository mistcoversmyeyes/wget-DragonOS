pub enum DebugLogEvents {
    DebugModeSet,   
    URLAnalysing(String),
    HostAnalysing(String),
    IPAnalysed(String),
    ConnectionEstablishing {host : String, ip: String , port : u16},
    ConnectionEstablished,
    HTTPRequestSend(String),
    HTTPRequestReceived(String),
    StartDownload,
    Downloading(usize),
    DownloadFinished(String),
}