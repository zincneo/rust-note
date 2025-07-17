/*
# let chains
1. let chains 是指在if和while条件中使用&&连接多个let表达式的语法特性
2. 在if和while语句中，用于判断的表达式部分`if condition` `while condition`
    - condition可以是一个简单表达式也可以是一个let表达式`let $pattern = $expr`
3. let表达式必须出现在条件的顶层，而不能嵌套在括号或其他逻辑运算符之中
*/
fn main() {
    // let在if语句中的用法
    {
        let x = 3;
        if x == 3 {}
        if let 3 = x {}
    }
    // 旧版语法let表达式的限制
    // if和while只能有一个let表达式
    {
        fn _old_way(nums: &[u8]) -> Option<u8> {
            let mut iter = nums.iter();
            if let Some(first) = iter.next() {
                if let Some(second) = iter.next() {
                    first.checked_add(*second)
                } else {
                    None
                }
            } else {
                None
            }
        }

        // 也可以使用嵌套的match
        fn _old_way_match(nums: &[u8]) -> Option<u8> {
            let mut iter = nums.iter();

            match (iter.next(), iter.next()) {
                (Some(first), Some(second)) => first.checked_add(*second),
                _ => None,
            }
        }
    }

    // 2024支持的let链语法
    {
        fn _sum_first_two(nums: &[u8]) -> Option<u8> {
            let mut iter = nums.iter();
            if let Some(first) = iter.next()
                && let Some(second) = iter.next()
            {
                first.checked_add(*second)
            } else {
                None
            }
        }
    }

    // 语法提示
    {
        let (option_1, option_2) = (Some(1), Some(2));

        // 多个let表达式使用&&连接 - 正确
        if let Some(_num_1) = option_1
            && let Some(_num_2) = option_2
        {
            println!("------");
        }

        // 多个let表达式和返回布尔值的表达式使用&&连接 - 正确
        if let Some(num_1) = option_1
            && num_1 % 2 == 0
        {
            println!("------");
        }

        // 不在顶层使用let表达式或使用其他逻辑运算符连接 - 错误
        // if let Some(num_1) = option_1 || let Some(num_2) = option_2 {}
        // if (Some(num_1) = option_1 || Some(num_2) = option_2) {}
    }

    // 实际应用
    // 解析配置文件
    {
        use std::collections::HashMap;
        #[derive(Debug)]
        #[allow(unused)]
        struct Config {
            host: String,
            port: u16,
            database: String,
        }

        fn _parse_config(settings: &HashMap<String, String>) -> Option<Config> {
            if let Some(host) = settings.get("host")
                && let host = host.clone()
                && let Some(port) = settings.get("port")
                && let Ok(port) = port.parse::<u16>()
                && let Some(database) = settings.get("database")
                && let database = database.clone()
                && !database.is_empty()
            {
                Some(Config {
                    host,
                    port,
                    database,
                })
            } else {
                None
            }
        }
    }
}
