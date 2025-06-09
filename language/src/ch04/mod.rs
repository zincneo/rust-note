/*!
# ch04 Rust结构体和枚举

## 01 结构体

1. [结构体语法](./ch04_01_struct/fn.f01_struct.html)
2. [结构体内存排布](./ch04_01_struct/fn.f02_memory.html)
3. [元组结构体](./ch04_01_struct/fn.f03_tuple_struct.html)

## 02 枚举

1. [枚举语法](./ch04_02_enum/fn.01_enum.html)
2. [Option枚举](./ch04_02_enum/fn.f02_option.html)
*/

pub mod ch04_01_struct;
pub mod ch04_02_enum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch04_01() {
        assert_eq!(ch04_01_struct::f01_struct(), ());
        assert_eq!(ch04_01_struct::f02_memory(), ());
        assert_eq!(ch04_01_struct::f03_tuple_struct(), ());
    }

    #[test]
    fn ch04_02() {
        assert_eq!(ch04_02_enum::f01_enum(), ());
        assert_eq!(ch04_02_enum::f02_option(), ());
    }
}
