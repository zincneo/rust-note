use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    // 1. 通过command宏配置子命令
    #[command(subcommand)]
    command: Commands,
}

// 2. 通过derive宏实现Subcommand特征
#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
    // 3. 子命令枚举中可以包裹实现了Args特征的结构体
    Sub(SubArgs),
}

#[derive(Args)]
struct SubArgs {
    value: String,
}

fn main() {
    let cli = Cli::parse();

    // 检测子命令
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}");
        }
        Commands::Sub(SubArgs { value }) => {
            println!("'myapp sub' was used, name is: {value:?}");
        }
    }
}
