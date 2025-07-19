# 参数处理模块

这个模块负责处理 wget-DragonOS 的命令行参数解析。

## 结构

- `mod.rs`: 模块入口文件，公开 `WgetArgs` 结构体
- `args.rs`: 包含 `WgetArgs` 结构体定义和相关方法

## 使用方法

```rust
use parameter_process::WgetArgs;

fn main() {
    // 解析命令行参数
    let args = WgetArgs::parse_args();
    
    // 处理调试模式
    args.handle_debug();
    
    // 获取URL
    println!("URL: {}", args.get_url());
    
    // 获取输出文件名（如果提供）
    if let Some(filename) = args.get_output_file_name() {
        println!("输出文件: {}", filename);
    }
}
```

## 当前支持的参数

- `url`: 位置参数，要下载的URL
- `--debug, -d`: 启用调试模式
- `--output-file, -O`: 指定输出文件名

## 扩展

要添加新的命令行参数，请在 `args.rs` 中的 `WgetArgs` 结构体中添加新字段，并在相应的 impl 块中添加相关方法。
