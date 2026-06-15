# Lesson 07：并发、线程与消息传递

## 学习目标

- 使用 `std::thread::spawn` 创建线程。
- 理解 `move` closure 为什么会把数据交给 worker。
- 用 `mpsc::channel` 从多个 worker 向主线程汇报结果。
- 用 `JoinHandle::join` 等待线程结束。
- 理解 Rust 如何避免数据竞争。

## 运行 demo

```bash
cargo run -p lesson_07_concurrency
```

## 代码导读

本课 demo 模拟前端构建系统：多个资源并行构建，主线程收集报告。

- `AssetJob`：构建任务。
- `AssetReport`：worker 完成后发回的消息。
- `mpsc::channel`：多生产者、单消费者消息队列。
- `simulate_build`：用 `sleep` 模拟耗时任务。

## 前端架构提示

JS 里你会用 Web Worker、Node worker_threads 或构建器内部线程池。Rust 的并发更强调“共享可变状态要受控”，所以常用消息传递把数据所有权从一个线程交给另一个线程。

## 练习

1. 增加一个失败分支，让 worker 返回 `Result<AssetReport, String>`。
2. 统计最慢资源和最快资源的耗时差。
3. 把任务数量改成 20 个，观察输出顺序为什么不稳定。
