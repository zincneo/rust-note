use std::env;

use egui_examples::{counter, fonts, layout, painter, widgets, window_frame};

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let case = args.next();
    match case {
        Some(case_name) => run_case(case_name),
        None => println!("No case name has been entered"),
    }
}

fn run_case(case_name: String) {
    match case_name.as_str() {
        "counter" => counter::run(),
        "widgets" => widgets::run(),
        "window_frame" => window_frame::run(),
        "layout" => layout::run(),
        "fonts" => fonts::run(),
        "painter" => painter::run(),
        _ => println!("Undefined case"),
    };
    println!("{case_name} end");
}
