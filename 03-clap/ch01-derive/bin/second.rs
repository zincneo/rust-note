use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // 1. 必要的参数<name>
    // 2. 通过arg宏可以通过short或者long将参数转换为option参数，并且配置option的名称
    // 3. 参数默认是必选的
    #[arg(short = 'n', long)]
    name: String,

    // 4. 默认参数是必选的，通过Option枚举包裹可以让参数为可选的
    age: Option<i32>,

    // 5. arg可以设置参数默认值
    #[arg(default_value_t = 2020)]
    port: u16,

    strs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
