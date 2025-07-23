mod ch01_hello;
mod ch02_widgets;
mod ch03_custom_widget;
mod ch04_layout;

use std::env;
fn main() {
    let mut args = env::args().into_iter();
    let Some(_arg_0) = args.next() else {
        return;
    };
    let Some(arg_1) = args.next() else {
        return;
    };
    match arg_1.as_str() {
        "1" => {
            ch01_hello::run();
        }
        "2" => {
            ch02_widgets::run();
        }
        "3" => {
            ch03_custom_widget::run();
        }
        "4" => {
            ch04_layout::run();
        }
        _ => {
            println!("不存在对应的示例，请输入存在的章节编号，如:1、2、3...");
        }
    }
}
