/*!
# MVC架构

1. Model:数据和业务逻辑
2. View:用户界面
3. Controller:接收用户输入，更新Model并通知View刷新

数据流:
`View->Controller->Model->View`
*/

struct Model {
    counter: i32,
}

impl Model {
    fn new() -> Self {
        Self { counter: 0 }
    }
    fn increment(&mut self) {
        self.counter += 1;
    }
    fn get_count(&self) -> i32 {
        self.counter
    }
}

struct View;

impl View {
    fn display(&self, count: i32) {
        println!("Count: {}", count);
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    fn new() -> Self {
        Controller {
            model: Model::new(),
            view: View,
        }
    }

    fn run(&mut self) {
        loop {
            // 手动更新 View
            self.view.display(self.model.get_count());
            println!("Enter 'inc' to increment, 'exit' to quit:");

            match self.view.get_input().as_str() {
                "inc" => {
                    self.model.increment(); // 更新 Model
                    // 需要手动触发 View 更新
                }
                "exit" => break,
                _ => println!("Invalid command"),
            }
        }
    }
}

fn main() {
    let mut app = Controller::new();
    app.run();
}
