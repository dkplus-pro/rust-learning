# Lesson 04：集合、迭代器与数据处理管道

## 学习目标

- 使用 `Vec<T>` 存放顺序数据。
- 使用 `HashMap<K, V>` 做聚合索引。
- 用 `iter()` 借用集合，避免不必要 move。
- 用 `map`、`filter`、`fold` 组合数据处理管道。
- 理解切片参数 `&[T]` 比 `&Vec<T>` 更通用。

## 运行 demo

```bash
cargo run -p lesson_04_collections_iterators
```

## 代码导读

本课 demo 模拟前端性能分析里的 page view 日志：

- `sample_page_views`：构造演示数据。
- `unique_active_users`：从访问日志提取去重用户。
- `total_time_by_route`：按路由聚合停留时间。
- `top_route`：找出累计停留时间最高的路由。

## 前端架构提示

Rust 的迭代器很像 `Array.prototype.map/filter/reduce`，但它通常是惰性的，且借用关系清晰。你会更早意识到“这一步是在复制数据，还是只是在读视图”。

## 练习

1. 增加 `device` 字段，统计 mobile/desktop 的平均耗时。
2. 把慢访问阈值 `900` 提取成函数参数。
3. 让 `top_route` 在耗时相同时按路由名排序。
