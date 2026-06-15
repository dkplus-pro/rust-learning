# Rust Learning for Frontend Architects

这是一套面向 **7 年前端架构师** 的 Rust 入门课程。课程从零开始，但不会停留在语法罗列；每一课都会把 Rust 概念映射到你熟悉的前端架构经验，例如包管理、类型系统、状态管理、数据流、构建任务和响应式 store。

## 课程原则

- **循序渐进**：从 Cargo、变量、类型开始，再进入所有权、错误处理、泛型、并发和架构实战。
- **每课独立 demo**：每个课程目录都是一个独立 Cargo package，可以单独 `cargo run -p <package>`。
- **文档 + 代码 + 注释**：每课有 README，代码内有中文注释，便于复盘和二次练习。
- **少依赖**：当前课程只使用 Rust 标准库，避免初学阶段被第三方生态分散注意力。

## 环境准备

推荐使用 rustup 安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
cargo --version
```

本仓库是 Cargo workspace。常用命令：

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo run -p lesson_01_foundation
```

如果系统缺少 C linker，`cargo run` 可能无法链接可执行文件；可以先用 `cargo check` 完成语义验证，然后安装系统构建工具链。

## 课程路线

| 课程 | 主题 | 你会得到什么 | 状态 |
| --- | --- | --- | --- |
| [Lesson 01](lessons/01-foundation/README.md) | 从零开始：环境、Cargo、变量、类型与控制流 | 把 Rust 当作一门强类型系统语言重新开始：理解 Cargo、不可变默认、基础类型、函数、if/match/for。 | ✅ 已完成 |
| Lesson 02 | 所有权、借用与可变引用 | 用前端数据流类比理解 move、borrow、clone、&mut，避免把 Rust 当成带类型的 JS。 | 📝 规划中 |
| Lesson 03 | 结构体、枚举、Option/Result 与模式匹配 | 用任务状态机学习 Rust 建模方式：struct 表达数据，enum 表达状态和命令，Result 表达可恢复错误。 | 📝 规划中 |
| Lesson 04 | 集合、迭代器与数据处理管道 | 用页面访问日志分析学习 Vec、HashMap、iter/filter/map/fold，建立 Rust 版数据流思维。 | 📝 规划中 |
| Lesson 05 | 模块、错误处理与边界设计 | 用配置解析器学习 mod/pub、Result、?、自定义错误类型，把工程边界从一开始写清楚。 | 📝 规划中 |
| Lesson 06 | Trait、泛型与生命周期 | 用 dashboard 组件模型理解 trait 抽象、泛型复用、生命周期标注，连接到前端组件和 hooks 设计经验。 | 📝 规划中 |
| Lesson 07 | 并发、线程与消息传递 | 用前端构建任务模拟学习 thread、mpsc channel、move closure，理解 Rust 的无数据竞争并发。 | 📝 规划中 |
| Lesson 08 | 架构实战：迷你响应式 Store | 综合所有权、枚举、trait object、状态机和迭代器，实现一个类似 Redux/Pinia 的极简 store。 | 📝 规划中 |

## 已完成课程运行命令

```bash
cargo run -p lesson_01_foundation  # Lesson 01
```

也可以使用脚本：

```bash
./scripts/run-lesson.sh lesson_01_foundation
./scripts/check-all.sh
```

## 建议学习方式

1. 先读每课 README 的“前端架构提示”。
2. 再读 `src/main.rs`，重点看注释和函数签名。
3. 运行 demo，并完成 README 里的练习。
4. 用自己的业务经验改写 demo，例如把任务状态机改成审批流、把日志分析改成真实埋点数据。

## 目录结构

```text
.
├── Cargo.toml              # workspace 配置
├── README.md               # 总课程路线
├── lessons/                # 每课独立 package
└── scripts/                # 运行和检查脚本
```
