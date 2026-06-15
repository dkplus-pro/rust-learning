# Lesson 01：从零开始：环境、Cargo、变量、类型与控制流

## 学习目标

- 能解释 `rustc`、`cargo`、package、workspace 的关系。
- 理解 Rust 默认不可变的设计，知道什么时候使用 `mut`。
- 能阅读基础类型、函数签名、`if`、`match`、`for`。
- 能独立运行本课 demo，并修改学习时间观察输出变化。

## 给 7 年前端架构师的类比

| 前端经验 | Rust 对应概念 | 关键差异 |
| --- | --- | --- |
| npm/yarn/pnpm | Cargo | Cargo 同时负责构建、测试、运行、依赖解析。 |
| TypeScript 类型标注 | Rust 函数签名 | Rust 类型参与内存安全和所有权检查。 |
| `const`/`let` 约束 | `let` 默认不可变 | Rust 要求显式 `mut` 才能改变量。 |
| `switch`/策略分支 | `match` | `match` 更强调穷尽性和类型统一。 |

## 运行 demo

```bash
cargo run -p lesson_01_foundation
```

> 如果当前环境没有 C linker，`cargo run` 可能失败；仍可先用 `cargo check -p lesson_01_foundation` 验证语义。

## 代码导读

- `TARGET_ROLE`：演示常量。
- `hours_per_week`：演示带类型注解的不可变量。
- `completed_lessons`：演示 `mut`。
- `learning_pace`：演示 `match` 范围匹配。
- `print_week_plan`：演示 `for` 与函数拆分。

## 练习

1. 把 `hours_per_week` 改成 3、12、20，观察 `match` 分支变化。
2. 给 `core_topics` 增加一个主题，确认 `for` 输出自动更新。
3. 在 `print_week_plan` 里增加第 4 周安排。
