# Lesson 08：架构实战：迷你响应式 Store

## 学习目标

- 把前 7 课知识组合成一个小型架构 demo。
- 用 `Action` 枚举集中表达状态变更入口。
- 用 `Store::dispatch` 模拟 reducer / action dispatcher。
- 用 `Subscriber` trait 和 `Box<dyn Subscriber>` 保存观察者。
- 用迭代器计算派生状态 `visible_todos`。

## 运行 demo

```bash
cargo run -p lesson_08_architecture_capstone
```

## 代码导读

这个 demo 类似极简 Redux/Pinia：

- `AppState`：全局状态。
- `Action`：状态变更命令。
- `Store`：持有状态、订阅者和 dispatch 逻辑。
- `Subscriber`：观察状态变化的接口。
- `ConsoleRenderer`：一个命令行渲染器。

## 前端架构提示

Rust 不会阻止你写“前端熟悉的架构模式”，但会迫使你更清楚地表达所有权、可变性和错误边界。这个 Store demo 不是为了替代成熟框架，而是帮助你把 Rust 类型系统映射回已有架构直觉。

## 练习

1. 增加 `RemoveTodo(u32)` action。
2. 增加第二个 subscriber：统计完成率。
3. 把 `dispatch` 改成返回 `Result<(), StoreError>`，让找不到 todo 成为显式错误。
