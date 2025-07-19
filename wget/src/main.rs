mod parameter_process;
use parameter_process::WgetArgs;


fn main() {
    let args = WgetArgs::parse_args();

    let is_debug_mode = args.debug;
    let url : String =  args.url;

    match is_debug_mode {
        true => {
            println!("调试模式开启");
        }
        false =>{
            println!("调试模式关闭");
        }
    }

    println!("输入的 URL 是 :{}", url);
}
