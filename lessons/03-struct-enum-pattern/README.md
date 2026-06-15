# Lesson 03：结构体、枚举、Option/Result 与模式匹配

## 学习目标

- 使用 `struct` 定义业务实体。
- 使用 `enum` 定义有限状态和命令。
- 用 `Option<T>` 表达“可能不存在”。
- 用 `Result<T, E>` 表达“可能失败且需要处理”。
- 用 `match` 写清晰的状态转移逻辑。

## 运行 demo

```bash
cargo run -p lesson_03_struct_enum_pattern
```

## 代码导读

本课 demo 是一个极小任务状态机：

- `TaskStatus`：任务状态。
- `Command`：外部命令。
- `Task::apply`：唯一允许改变状态的入口。
- `find_task`：返回 `Option<&Task>`，把“找不到”显式建模。

## 前端架构提示

TypeScript 里你可能会用 discriminated union 管理状态；Rust 的 `enum` 原生支持携带数据，并且 `match` 会强制你处理全部重要分支。这让状态机比散落的布尔字段更容易维护。

## 练习

1. 增加 `Command::Cancel` 和 `TaskStatus::Canceled`。
2. 让 `Blocked` 携带阻塞原因，而不是只打印原因。
3. 把 `String` 错误改成自定义 `enum TaskError`。
