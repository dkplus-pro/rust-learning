// Lesson 02: 所有权、借用与可变引用 demo
// 运行：cargo run -p lesson_02_ownership_borrowing
//
// 前端类比：
// - move：像把一个对象交给新的模块负责，原模块不再使用它。
// - borrow：像只读 selector，不拿走数据所有权。
// - &mut：像 reducer 中唯一允许修改 draft 的地方，必须独占。

fn main() {
    let title = String::from("从 TypeScript 到 Rust");

    // 只读借用：preview_title 只需要看标题，不拥有标题。
    let preview = preview_title(&title);
    println!("预览标题：{preview}");

    // clone 会复制堆上的字符串。它安全但有成本，架构代码里要有意识地使用。
    let copied_title = title.clone();
    let normalized = normalize_title(copied_title);
    println!("规范化标题：{normalized}");

    // title 仍然可用，因为上面传出去的是 clone 后的新 String。
    println!("原始标题仍可使用：{title}");

    let mut tags = Vec::from([String::from("frontend"), String::from("architecture")]);

    // 可变借用：add_tag 在调用期间独占 tags 的修改权。
    add_tag(&mut tags, "rust");
    add_tag(&mut tags, "systems-thinking");

    println!("课程标签：{}", tags.join(", "));

    // move：consume_tags 接管 tags 的所有权，调用后 main 不能再使用 tags。
    let tag_count = consume_tags(tags);
    println!("标签总数：{tag_count}");
}

fn preview_title(title: &str) -> String {
    // &str 是借用视图，适合只读 API；这里没有复制完整字符串。
    let max_chars = 10;
    let preview: String = title.chars().take(max_chars).collect();
    format!("{preview}...")
}

fn normalize_title(title: String) -> String {
    // 这个函数拥有 title，因此可以自由转换并返回新的 String。
    title.trim().to_lowercase().replace(' ', "-")
}

fn add_tag(tags: &mut Vec<String>, tag: &str) {
    // &mut Vec<String> 允许 push；同一时间不能再有其它借用访问 tags。
    if !tags.iter().any(|existing| existing == tag) {
        tags.push(tag.to_string());
    }
}

fn consume_tags(tags: Vec<String>) -> usize {
    // tags 被 move 到这里。函数结束后，Vec 会自动释放内存。
    for tag in &tags {
        println!("  已登记标签：{tag}");
    }
    tags.len()
}
