// Lesson 01: Rust 基础语法与 Cargo demo
// 运行：cargo run -p lesson_01_foundation
//
// 面向前端架构师的提示：
// - `let` 默认不可变，类似团队默认启用 strict lint，先防错再开放修改。
// - `match` 可以看作更强大的 switch，编译器会帮你检查分支是否覆盖完整。
// - 函数签名里的类型是 API 契约，像 TypeScript 类型，但 Rust 会在编译期进一步保证内存安全。

const TARGET_ROLE: &str = "7 年前端架构师";

fn main() {
    println!("Rust 学习档案：{TARGET_ROLE}");

    // 默认不可变：下面的 `hours_per_week` 不能被重新赋值。
    let hours_per_week: u8 = 8;

    // 如果确实要变，必须显式写 `mut`，让“状态变化”在代码评审里更醒目。
    let mut completed_lessons = 0;
    completed_lessons += 1;

    let pace = learning_pace(hours_per_week);
    println!("每周 {hours_per_week} 小时，建议节奏：{pace}");
    println!("当前已完成课程数：{completed_lessons}");

    // 数组长度固定，适合表达“编译期已知长度”的列表。
    let core_topics = ["Cargo", "变量", "类型", "控制流"];
    println!("\n本课主题：");
    for (index, topic) in core_topics.iter().enumerate() {
        let order = index + 1;
        println!("  {order}. {topic}");
    }

    // if 是表达式：可以直接把分支结果赋给变量。
    let should_build_demo = hours_per_week >= 4;
    let next_action = if should_build_demo {
        "写一个可运行 demo"
    } else {
        "先读文档，再补 demo"
    };
    println!("\n下一步：{next_action}");

    print_week_plan(3);
}

fn learning_pace(hours_per_week: u8) -> &'static str {
    // match 支持范围匹配，所有分支必须返回同一种类型。
    match hours_per_week {
        0..=3 => "慢速：先熟悉语法，不急于项目化",
        4..=8 => "标准：每周 1 课 + 1 个 demo",
        9..=15 => "强化：每周 2 课 + 复盘笔记",
        _ => "冲刺：可以增加源码阅读与架构练习",
    }
}

fn print_week_plan(weeks: u8) {
    println!("\n前三周安排：");
    for week in 1..=weeks {
        let topic = match week {
            1 => "环境、Cargo、变量与类型",
            2 => "所有权、借用、可变引用",
            3 => "结构体、枚举、模式匹配",
            _ => "进入项目练习",
        };
        println!("  第 {week} 周：{topic}");
    }
}
