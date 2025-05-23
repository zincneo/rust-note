/*!
# <span style="color: #ed8796;">ch06 Rust模式匹配</span>

## 01 match和if let

1. [match](./ch06_01_match/fn.f01_01_match.html)
2. [if let](./ch06_01_match/fn.f01_02_if_let.html)
3. [matches!](./fn.f01_03_matches.html)
4. [解构Option枚举](./fn.f01_04_option.html)

## 02 模式

1. [模式内容和使用](./ch06_02_01_pattern/fn.f02_01_pattern.html)
2. [模式使用场景](./ch06_02_02_scence/fn.f02_02_scence.html)
3. [解构和忽略](./ch06_02_03_deconstruction/fn.f02_03_deconstruction.html)
*/

pub mod ch06_01_match;
pub use ch06_01_match::{f01_01_match, f01_02_if_let, f01_03_matches, f01_04_option};

pub mod ch06_02_01_pattern;
pub use ch06_02_01_pattern::f02_01_pattern;

pub mod ch06_02_02_scence;
pub use ch06_02_02_scence::f02_02_scence;

pub mod ch06_02_03_deconstruction;
pub use ch06_02_03_deconstruction::f02_03_deconstruction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch06_01() {
        assert_eq!(f01_01_match(), ());
        assert_eq!(f01_02_if_let(), ());
        assert_eq!(f01_03_matches(), ());
        assert_eq!(f01_04_option(), ());
    }

    #[test]
    fn ch06_02() {
        assert_eq!(f02_01_pattern(), ());
        assert_eq!(f02_02_scence(), ());
        assert_eq!(f02_03_deconstruction(), ());
    }
}
