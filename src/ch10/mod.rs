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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch10_01() {
        assert_eq!(_ch10_01_concept_lifetime(), ());
    }
}
