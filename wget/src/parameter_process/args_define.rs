use clap::Parser;

#[derive(Parser, Debug)]         // 使用 derive 宏自动为结构体 WgetArgs实现 Parser 和 Debug Trait 
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
    #[arg(short = 'd', long = "debug", default_value = true)]
    pub debug: bool,

    // 选项1：输出文件名称
    #[arg(short = 'O', long = "output-file")]
    pub output_file_name: Option<String>,

    // 选项x：..........
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
