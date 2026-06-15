# Lesson 05：模块、错误处理与边界设计

## 学习目标

- 用 `mod` 拆分文件，用 `pub` 控制可见性。
- 用 `Result<T, E>` 和 `?` 传播可恢复错误。
- 定义自定义错误枚举，让调用方知道失败原因。
- 为错误实现 `Display` 和 `Error`。
- 把“外部输入解析”放到明确边界里。

## 运行 demo

```bash
cargo run -p lesson_05_errors_modules
```

## 代码导读

- `src/main.rs`：应用入口，只关心成功/失败结果。
- `src/config.rs`：解析模块，隐藏内部细节。
- `ConfigError`：将缺字段、坏行、坏端口、坏布尔值分开表达。
- `load_from_str`：核心 API，返回 `Result<AppConfig, ConfigError>`。

## 前端架构提示

前端架构里常见边界包括 URL query、localStorage、接口响应、环境变量。Rust 鼓励你在边界处立即从“弱类型字符串”转换成“强类型领域对象”，错误也作为类型的一部分暴露。

## 练习

1. 增加 `log_level = info` 配置，并限制可选值。
2. 为 `load_from_str` 增加单元测试。
3. 把未知 key 的错误从 `InvalidLine` 拆成 `UnknownKey(String)`。
