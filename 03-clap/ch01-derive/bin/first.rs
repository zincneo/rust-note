// clap库用来解析命令行参数
// clap::Parser特征 -> fn parse() -> Self 内部通过std::env::args() API 来获取命令行参数
use clap::Parser;

/// 通过derive宏来为结构体自动实现Parser特征
/// command宏用于设置命令的相关信息，[所有可以设置的属性](https://docs.rs/clap/latest/clap/_derive/index.html#command-attributes)
#[derive(Parser)]
#[command(name = "first")]
#[command(version = "v0.0.1")]
#[command(about = "Does awsome things", long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    two: String,

    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
