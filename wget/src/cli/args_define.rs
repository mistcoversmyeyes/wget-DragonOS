use clap::Parser;

#[derive(Parser, Debug, Clone)]         
#[command(name = "wget-DragonOS")]
#[command(author = "Yuming")]
#[command(version = "0.1.0")]
#[command(about = "")]  
pub struct WgetArgs {
    
    // ================= 位置参数 (Positional Arguments) =================================
    
    // 位置参数0：URL (必选)
    pub url: String,           

    // ================= 选项 (Options) =================================

    // 选项0：输出debug 日志
    #[arg(short = 'd', long = "debug", default_value = "true")]
    pub debug: bool,

    // 选项1：输出文件的名称
    #[arg(short = 'O', long = "output-file")]
    pub output_file_name: Option<String>,

    // 选项2：输出文件的目录
    #[arg(short = 'P',long = "prefix",)]
    pub directory_prefix: Option<String>,

    // 选项x：示例选项
    // #[arg(short = 'x',long = "xxxx", default_value = "xxxx",)]
    // pub arg_name: Option<T>

    // TODO:添加更多的选项
}

impl WgetArgs {
    // 解析命令行参数
    pub fn parse_args() -> Self {
        Self::parse()
    }
    
    // 获取URL
    pub fn get_url(&self) -> &str {
        &self.url
    }
    
    // 获取输出文件名
    pub fn get_output_file_name(&self) -> Option<&str> {
        self.output_file_name.as_deref()
    }
}
