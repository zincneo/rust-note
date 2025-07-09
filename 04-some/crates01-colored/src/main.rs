// colored库:让终端输出带上颜色更简单的库
// colored库提供的特征Colorize，该特征为str类型实现，提供一系列颜色方法将字符串转换为ColoredString类型
use colored::Colorize;
// colored库提供的枚举Color，除了不包含值的各种颜色名枚举值外还包含TrueColor {r: u8, g: u8, b: u8}
use colored::Color;
use colored::ColoredString;

fn main() {
    println!(
        "{} {} {}",
        "or use".red(),
        "any".italic().yellow(),
        "string type".cyan()
    );

    let catppuccin_red = Color::TrueColor {
        r: 237,
        g: 135,
        b: 150,
    };

    let mut catppuccin_red_string = ColoredString::default();
    catppuccin_red_string.input = "catppuccin_red".to_string();
    catppuccin_red_string.fgcolor = Some(catppuccin_red);

    println!("{}", catppuccin_red_string);
}
