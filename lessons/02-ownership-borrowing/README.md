# Lesson 02：所有权、借用与可变引用

## 学习目标

- 知道 `String`、`Vec<T>` 这类堆数据为什么会发生所有权转移。
- 能区分 `T`、`&T`、`&mut T` 三种函数参数设计。
- 理解 `clone` 的安全性与性能成本。
- 能从前端状态管理视角理解“同一时间只有一个可变引用”。

## 核心概念

- **所有权（ownership）**：每个值有且只有一个 owner。
- **移动（move）**：把 owner 转交给另一个变量或函数。
- **借用（borrow）**：用 `&T` 只读访问，不接管生命周期。
- **可变借用（mutable borrow）**：用 `&mut T` 修改数据，调用期间必须独占。

## 运行 demo

```bash
cargo run -p lesson_02_ownership_borrowing
```

## 代码导读

- `preview_title(&title)`：借用字符串，不拿走所有权。
- `normalize_title(copied_title)`：函数接管 `String`，返回新值。
- `add_tag(&mut tags, "rust")`：通过可变引用集中修改。
- `consume_tags(tags)`：演示 move 后原变量不可再用。

## 前端架构提示

把所有权看成“谁负责释放资源”的设计约束。大型前端里你会通过 store、query cache、组件边界管理数据来源；Rust 把这种边界推到编译期检查。

## 练习

1. 删除 `title.clone()`，直接把 `title` 传给 `normalize_title`，观察编译器报错。
2. 在 `add_tag` 调用之间增加一次只读打印，理解可变借用的作用域。
3. 把 `preview_title` 的最大字符数改成函数参数。
