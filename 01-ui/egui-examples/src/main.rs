use std::env;

use egui_examples::counter;

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
        _ => println!("Undefined case"),
    };
    println!("{case_name} end");
}
