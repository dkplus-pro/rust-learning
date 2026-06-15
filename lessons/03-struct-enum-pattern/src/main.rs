// Lesson 03: 结构体、枚举、Option/Result 与模式匹配 demo
// 运行：cargo run -p lesson_03_struct_enum_pattern
//
// 前端类比：
// - struct：组件 props / domain model。
// - enum：联合类型，例如 TypeScript 的 discriminated union。
// - Result：显式建模“成功或失败”，避免异常在边界外乱飞。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
    Blocked,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Start,
    Complete,
    Block(&'static str),
    Reopen,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            status: TaskStatus::Todo,
        }
    }

    fn apply(&mut self, command: Command) -> Result<(), String> {
        match (self.status, command) {
            (TaskStatus::Todo, Command::Start) => {
                self.status = TaskStatus::Doing;
                Ok(())
            }
            (TaskStatus::Doing, Command::Complete) => {
                self.status = TaskStatus::Done;
                Ok(())
            }
            (TaskStatus::Doing, Command::Block(reason)) => {
                println!("任务 {} 被阻塞：{reason}", self.id);
                self.status = TaskStatus::Blocked;
                Ok(())
            }
            (TaskStatus::Blocked, Command::Reopen) => {
                self.status = TaskStatus::Doing;
                Ok(())
            }
            (current, rejected) => Err(format!("状态 {current:?} 不接受命令 {rejected:?}")),
        }
    }
}

fn main() {
    let mut tasks = vec![
        Task::new(1, "搭建 Rust workspace"),
        Task::new(2, "写所有权练习"),
        Task::new(3, "复盘前端类比"),
    ];

    let target_id = 2;
    match find_task(&tasks, target_id) {
        Some(task) => println!("找到任务：{} - {}", task.id, task.title),
        None => println!("没有找到任务 {target_id}"),
    }

    let task = tasks
        .iter_mut()
        .find(|task| task.id == target_id)
        .expect("示例数据里应该存在目标任务");

    for command in [
        Command::Start,
        Command::Block("等待评审"),
        Command::Reopen,
        Command::Complete,
    ] {
        if let Err(error) = task.apply(command) {
            println!("命令失败：{error}");
        }
        println!("当前状态：{:?}", task.status);
    }

    // 故意触发一个非法命令，观察 Result 如何显式返回错误。
    if let Err(error) = task.apply(Command::Start) {
        println!("预期中的错误：{error}");
    }
}

fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    tasks.iter().find(|task| task.id == id)
}
