// Lesson 08: 架构实战：迷你响应式 Store demo
// 运行：cargo run -p lesson_08_architecture_capstone
//
// 本课把前面学过的知识串起来：
// - struct 建模状态
// - enum 建模 action/filter
// - trait object 保存订阅者
// - iterator 查询派生状态

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Filter {
    #[default]
    All,
    Active,
    Done,
}

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Debug, Default)]
struct AppState {
    todos: Vec<Todo>,
    filter: Filter,
    next_id: u32,
}

impl AppState {
    fn visible_todos(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.done,
                Filter::Done => todo.done,
            })
            .collect()
    }
}

enum Action {
    AddTodo(String),
    ToggleTodo(u32),
    SetFilter(Filter),
}

trait Subscriber {
    fn on_change(&self, state: &AppState);
}

struct ConsoleRenderer;

impl Subscriber for ConsoleRenderer {
    fn on_change(&self, state: &AppState) {
        let visible = state.visible_todos();
        println!(
            "\n[render] filter={:?}, visible={}",
            state.filter,
            visible.len()
        );
        for todo in visible {
            let mark = if todo.done { "x" } else { " " };
            println!("  [{mark}] #{} {}", todo.id, todo.title);
        }
    }
}

struct Store {
    state: AppState,
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl Store {
    fn new() -> Self {
        Self {
            state: AppState {
                next_id: 1,
                ..AppState::default()
            },
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    fn dispatch(&mut self, action: Action) {
        match action {
            Action::AddTodo(title) => {
                let todo = Todo {
                    id: self.state.next_id,
                    title,
                    done: false,
                };
                self.state.next_id += 1;
                self.state.todos.push(todo);
            }
            Action::ToggleTodo(id) => {
                match self.state.todos.iter_mut().find(|todo| todo.id == id) {
                    Some(todo) => todo.done = !todo.done,
                    None => println!("没有找到 todo #{id}"),
                }
            }
            Action::SetFilter(filter) => self.state.filter = filter,
        }

        self.notify();
    }

    fn notify(&self) {
        for subscriber in &self.subscribers {
            subscriber.on_change(&self.state);
        }
    }
}

fn main() {
    let mut store = Store::new();
    store.subscribe(Box::new(ConsoleRenderer));

    store.dispatch(Action::AddTodo("读完所有权章节".to_string()));
    store.dispatch(Action::AddTodo("把状态机 demo 改成自己的业务".to_string()));
    store.dispatch(Action::ToggleTodo(1));
    store.dispatch(Action::SetFilter(Filter::Active));
    store.dispatch(Action::SetFilter(Filter::Done));
    store.dispatch(Action::SetFilter(Filter::All));
}
