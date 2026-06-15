// Lesson 06: Trait、泛型与生命周期 demo
// 运行：cargo run -p lesson_06_traits_generics_lifetimes
//
// 前端类比：
// - trait 像“组件必须实现的接口”。
// - 泛型像可复用组件/Hook，但 Rust 会在编译期生成高效代码。
// - 生命周期标注不是手动释放内存，而是在说明引用之间的有效期关系。

trait Renderable {
    fn render(&self) -> String;
}

struct Metric {
    name: String,
    value: u32,
    unit: String,
}

impl Renderable for Metric {
    fn render(&self) -> String {
        format!("{}: {}{}", self.name, self.value, self.unit)
    }
}

struct Alert<'a> {
    level: &'a str,
    message: &'a str,
}

impl Renderable for Alert<'_> {
    fn render(&self) -> String {
        format!("[{}] {}", self.level, self.message)
    }
}

struct Panel<T: Renderable> {
    title: String,
    widgets: Vec<T>,
}

impl<T: Renderable> Panel<T> {
    fn new(title: &str, widgets: Vec<T>) -> Self {
        Self {
            title: title.to_string(),
            widgets,
        }
    }

    fn render(&self) -> String {
        let title = &self.title;
        let body = self
            .widgets
            .iter()
            .map(Renderable::render)
            .collect::<Vec<_>>()
            .join("\n");
        format!("== {title} ==\n{body}")
    }
}

fn main() {
    let metric_panel = Panel::new(
        "核心指标",
        vec![
            Metric {
                name: "首屏耗时".to_string(),
                value: 1_230,
                unit: "ms".to_string(),
            },
            Metric {
                name: "转化率".to_string(),
                value: 18,
                unit: "%".to_string(),
            },
        ],
    );

    let alert_panel = Panel::new(
        "架构告警",
        vec![
            Alert {
                level: "warn",
                message: "bundle size 超过预算",
            },
            Alert {
                level: "info",
                message: "Rust demo 已通过 cargo check",
            },
        ],
    );

    println!("{}", metric_panel.render());
    println!("\n{}", alert_panel.render());

    let primary = choose_primary_label("转化率", "关键转化率指标");
    println!("\n更适合作为看板标题：{primary}");
}

fn choose_primary_label<'a>(left: &'a str, right: &'a str) -> &'a str {
    // 返回值可能来自 left，也可能来自 right，因此需要说明返回引用不会比二者活得更久。
    if left.chars().count() >= right.chars().count() {
        left
    } else {
        right
    }
}
