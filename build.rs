// build.rs
fn main() {
    // 在 Rust 构建前运行 CSS 构建
    if std::process::Command::new("pnpm")
        .args(&["run", "build:css"])
        .status()
        .unwrap()
        .success()
    {
        println!("CSS built successfully");
    } else {
        panic!("CSS build failed");
    }

    // 其他构建步骤...
}
