/*!
# MVVM架构
- ViewModel:暴露数据状态和命令，自动同步Model和View

对比MVC的改进: 增加了数据绑定机制，替代手动更新，View自动响应ViewModel的变化
*/
use std::cell::RefCell;
use std::rc::Rc;

struct Model {
    count: i32,
}

impl Model {
    fn new() -> Self {
        Model { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

struct ViewModel {
    count: Rc<RefCell<i32>>,
    on_count_changed: Option<Box<dyn Fn(i32)>>,
}

impl ViewModel {
    fn new(model: Rc<RefCell<Model>>) -> Self {
        let count = Rc::new(RefCell::new(model.borrow().count));
        ViewModel {
            count,
            on_count_changed: None,
        }
    }

    fn increment(&self, model: Rc<RefCell<Model>>) {
        model.borrow_mut().increment();
        *self.count.borrow_mut() = model.borrow().count;
        // 自动触发 View 更新
        if let Some(callback) = &self.on_count_changed {
            callback(*self.count.borrow());
        }
    }

    fn bind(&mut self, callback: impl Fn(i32) + 'static) {
        self.on_count_changed = Some(Box::new(callback));
    }
}

struct View {
    view_model: Rc<RefCell<ViewModel>>,
}

impl View {
    fn new(view_model: Rc<RefCell<ViewModel>>) -> Self {
        View { view_model }
    }

    fn run(&self, model: Rc<RefCell<Model>>) {
        // 绑定数据变化回调
        self.view_model
            .borrow_mut()
            .bind(|count| println!("Count: {}", count));

        loop {
            println!("Enter 'inc' to increment, 'exit' to quit:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "inc" => {
                    // 直接调用 ViewModel
                    self.view_model.borrow().increment(model.clone());
                }
                "exit" => break,
                _ => println!("Invalid command"),
            }
        }
    }
}

fn main() {
    let model = Rc::new(RefCell::new(Model::new()));
    let view_model = Rc::new(RefCell::new(ViewModel::new(model.clone())));
    let view = View::new(view_model);
    view.run(model);
}
