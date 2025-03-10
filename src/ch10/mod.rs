/// # 生命周期
/// - 变量在内存中有创建到销毁的过程
/// - 变量的引用应该在变量被销毁时会之前不可以再被使用
/// - 生命周期的概念就是在程序中变量的生老病死
fn _ch10_01_concept_lifetime() {
    {
        let r;
        {
            let x = 5;
            r = &x;
            println!("r: {r}");
            // 在离开这个代码块的时候，变量x被drop
            // 因此离开之后r不应该引用一个生命周期已经被结束的变量
        }
        // 被引用的生命周期不正确，编译器不会允许编译
        // println!("r: {r}");
    }
}

/// # 函数中使用生命周期
/// - 以'开头在泛型参数列表中来定义生命周期标识符
/// - 生命周期标识符并非凭空变出来生命周期而是为了将生命周期往更短的变量约束以通过编译
fn _ch10_02_function_lifetime() {
    // 定义一个函数用来返回两个字符串切片中更长的一个
    // 这里就涉及到一个生命周期的问题，因为切片是一个引用，编译器不知道返回变量谁会活的更久
    // 因此为了通过生命周期标识符可以约束在不清楚生命周期谁最长的时候使用时对齐到短的那一个
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let x = "hello";
    let rstr;
    {
        let y = "world!";
        rstr = longest(x, y);
        println!("{rstr}");
        // 生命周期标识符将rstr的生命周期约束为和y相同，在离开代码块之后不能再被使用
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch10_01() {
        assert_eq!(_ch10_01_concept_lifetime(), ());
    }

    #[test]
    fn ch10_02() {
        assert_eq!(_ch10_02_function_lifetime(), ());
    }
}
