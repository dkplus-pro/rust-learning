# Lesson 06：Trait、泛型与生命周期

## 学习目标

- 用 `trait` 描述行为契约。
- 用泛型 `T` 编写可复用容器。
- 理解 `T: Renderable` 这种 trait bound。
- 能读懂简单生命周期标注 `<'a>`。
- 知道生命周期标注描述的是引用关系，不是手动内存管理。

## 运行 demo

```bash
cargo run -p lesson_06_traits_generics_lifetimes
```

## 代码导读

本课 demo 用 dashboard panel 类比前端组件系统：

- `Renderable`：任何可渲染 widget 都要实现的行为。
- `Metric` / `Alert`：两类具体 widget。
- `Panel<T: Renderable>`：泛型容器，只要求子项能 render。
- `choose_primary_label<'a>`：演示返回引用与输入引用之间的生命周期关系。

## 前端架构提示

Trait 不是“继承父类”，更接近能力接口。泛型让你在保持类型安全的同时复用逻辑。生命周期让“这个引用来自哪里、能活多久”成为 API 契约的一部分。

## 练习

1. 新增 `ProgressBar` 并实现 `Renderable`。
2. 给 `Panel` 增加 `is_empty` 方法。
3. 尝试让函数返回局部创建的 `String` 的引用，观察生命周期错误。
