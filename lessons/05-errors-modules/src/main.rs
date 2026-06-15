// Lesson 05: 模块、错误处理与边界设计 demo
// 运行：cargo run -p lesson_05_errors_modules

mod config;

use config::load_from_str;

fn main() {
    let raw_config = r#"
        # Rust 课程 demo 配置
        app_name = rust-learning
        port = 8080
        enable_cache = true
    "#;

    match load_from_str(raw_config) {
        Ok(config) => {
            println!("应用名称：{}", config.app_name);
            println!("监听端口：{}", config.port);
            println!("缓存开关：{}", config.enable_cache);
        }
        Err(error) => println!("配置解析失败：{error}"),
    }

    // 故意传入坏配置，演示错误不会 panic，而是作为 Result 返回。
    let broken_config = "app_name = broken\nport = eighty\nenable_cache = yes";
    if let Err(error) = load_from_str(broken_config) {
        println!("\n预期中的坏配置错误：{error}");
    }
}
