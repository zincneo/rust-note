/*!
# 06 Rust模式匹配

## 01 match和if let

1. [match关键字](./ch06_01_match/fn.f01_match.html)
2. [if let](./ch06_01_match/fn.02_if_let.html)
3. [matches!](./ch06_01_match/fn.f03_matches.html)
4. [解构Option枚举](./ch06_01_match/fn.f04_option.html)

## 02 模式

1. [模式内容和使用](./ch06_02_01_pattern/fn.pattern.html)
2. [模式使用场景](./ch06_02_02_scence/fn.scence.html)
3. [解构和忽略](./ch06_02_03_deconstruction/fn.deconstruction.html)
*/

pub mod ch06_01_match;
pub mod ch06_02_01_pattern;
pub mod ch06_02_02_scence;
pub mod ch06_02_03_deconstruction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(ch06_01_match::f01_match(), ());
        assert_eq!(ch06_01_match::f02_if_let(), ());
        assert_eq!(ch06_01_match::f03_matches(), ());
        assert_eq!(ch06_01_match::f04_option(), ());
    }

    #[test]
    fn ch06_02() {
        assert_eq!(ch06_02_01_pattern::pattern(), ());
        assert_eq!(ch06_02_02_scence::scence(), ());
        assert_eq!(ch06_02_03_deconstruction::deconstruction(), ());
    }
}
