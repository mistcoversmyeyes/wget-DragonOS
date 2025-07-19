use std::process::Command;

#[test]
fn test_debug_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--debug", "http://example.com"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("调试模式开启"));
    assert!(stdout.contains("输入的 URL 是 :http://example.com"));
}