use std::env;
fn main() {
    let mut args = env::args();
    let _ = args.next();
    let demo = args.next();
    if let Some(demo_name) = demo {
        run_demo(demo_name);
    }
}

fn run_demo(demo_name: String) {
    match demo_name.as_str() {
        "thread" => demo::thread::run(),
        _ => println!("Undefined case"),
    }
}
