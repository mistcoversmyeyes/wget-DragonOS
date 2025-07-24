use std::{env, fmt::DebugTuple, fs::File, net::TcpStream, path::{Path, PathBuf}};

use crate::{events::http_events::HttpEvents, log::{debuglog::productor::DebugLogProductor, on_event::{self, OnEventHttp}}, cli::WgetArgs};
use std::io::{Read, Seek, SeekFrom, Write};


pub struct FileDownloader<'a> {
    source : &'a mut dyn Read,
    destination : File,
    content_length: usize,
    offset : usize,            //为断点续传预留的

}


impl<'a> FileDownloader<'a> {
    /// 给定一个已经去除了响应头内容的 网络流 `stream`, 命令行参数解析结果 `para` 和 获取到的文件内容大小 `content_length`
    /// 创建一个包含源网络流，目的文件，文件大小，已经下载的大小(offset)的执行下载工作的结构体。
    /// 默认从命令行参数的 <URL> 获取文件名，文件名为 <URL>字段的最后一个 '/' 后面的内容
    /// ``` bash
    /// # pwd = /home/username/tools
    /// wget http://google.com/index.html
    /// ```
    /// 对于以上示例，保存文件的路径为 "/home/username/tools/index.html"
    /// 
    pub fn default(stream: &'a mut dyn Read, para: WgetArgs,content_length: usize) -> Self {
        let directory_prefix: PathBuf = env::current_dir().unwrap();
        // 尝试获取 命令行选项 '-O' 传入的文件名，如果没有传入的文件名，那么从 url 中获取
        let file_name = match para.output_file_name {
            Some(name) => name ,
            None => {
                para.url
                .rsplit('/')
                .next()
                .expect("自动获取文件名失败")
                .to_string()
            },
        };
        
        // 拼接目录前缀和文件名为完整文件路径
        let file_path: PathBuf = directory_prefix.join(file_name);

        // 创建文件
        let destination: File = File::create(file_path).unwrap();

        FileDownloader {
            source: stream,
            destination,
            content_length,
            offset: 0 ,
        }
    }

    pub fn from_name (){
        // TODO: 给定新文件的文件名初始化
    }

    pub fn from_name_and_prefix (){
        // TODO: 给定保存的文件名和指定的目录前缀
    }


    pub fn download(&mut self) -> std::io::Result<usize> {
        // 定位到断点
        self.destination.seek(SeekFrom::Start(self.offset as u64))?;
        let mut total_written: usize = 0usize;
        let mut buf = [0u8; 8192];

        // 开始下载内容
        DebugLogProductor::get_instance().on_event(&HttpEvents::StartDownload);
        loop {
            let n = self.source.read(&mut buf)?;
            if n == 0 {
                break;
            }
            self.destination.write_all(&buf[..n])?;
            
            total_written += n as usize;
            DebugLogProductor::get_instance().on_event(&HttpEvents::Downloading(total_written));
        }
        Ok(total_written)
    }
}