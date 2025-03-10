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
/// - 函数的返回值如果是一个引用类型，那么其生命周期不可能无中生有，只会有两种来源
///   1. 跟随某一个实参
///   2. 跟随函数体内新建引用的生命周期 -> 这种时候必然是报错，因为试图返回一个临时变量的引用，要用到这种需求就直接返回对应数据的所有权就可以了
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

/// ## 结构体中使用生命周期
/// - 在泛型参数列表中加上了生命周期标识符
/// - 在引用&符号后使用生命周期标识符
/// - 该字段的生命周期会跟随被赋予的引用
/// - 被引用的数据生命周期要大于等于该结构体的生命周期
fn _ch10_03_struct_lifetime() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let mut i;
    let novel = String::from("Call me Ishmael. Some years ago...");
    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        i.part = &novel;
    }
    // 下一行可以通过编译，因为i在离开代码块之前引用了生命周期更长的novel变量
    println!("{:?}", i);
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

    #[test]
    fn ch10_03() {
        assert_eq!(_ch10_03_struct_lifetime(), ());
    }
}
