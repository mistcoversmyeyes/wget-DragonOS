mod cli;           // 命令行接口：解析命令行参数，调度使用下层模块完成任务
mod log;           // 日志模块 ✓
mod web;           // 网络模块 ✓  
mod file_writer;   // 文件I/O模块 ✓ 
// 未来可能添加：
// mod recursive;  // 递归模块 (HTML解析、URL队列等)

use cli::CLI;

fn main() {
    // 创建CLI实例并运行
    let cli = CLI::new();
    
    if let Err(e) = cli.run() {
        eprintln!("程序执行失败: {}", e);
        std::process::exit(1);
    }
}
