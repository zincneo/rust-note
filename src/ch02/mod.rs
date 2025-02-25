fn _ch02_01_variable_mutability() {
    /// ## rust变量
    /// 1. 关键字let
    /// 2. rust变量默认不可以在定义后改变值
    /// 3. 编译器会警告未使用的变量
    /// 4. 变量名使用_开头可以关闭对该变量未使用的警告
    /// 5. 如果希望变量可变需要加上mut关键字
    fn variabel_mutability() {
        let _v = 10;
        let mut _v = 10;
        _v += 2;
    }
    variabel_mutability();

    /// ## 变量遮蔽
    /// 1. rust支持同名变量在同一作用域重复定义
    /// 2. 后定义的会遮蔽前面定义的
    /// 3. 对于后定义但变量生命周期先结束的，结束后可以再使用之前定义的同名变量
    fn shadowing() {
        let _v = 10;
        println!("{_v}");
        let _v = 20;
        println!("{_v}");
        {
            let _v = 30;
            println!("{_v}");
        }
        println!("{_v}");
    }
    shadowing();
}

fn _ch02_02_data_type() {
    /// ## rust标量类型
    /// 1. integer:
    ///     - u/i 8/16/32/64/128/size
    ///     - 编译器默认推导整数字面量为i32
    ///     - 字面量支持二进制(0b)、八进制(0o)、十六进制(0x)、u8字符(b'')
    /// 2. float
    ///     - f32/f64
    ///     - 编译器默认推导浮点数字面量为f64
    /// 3. boolean
    /// 4. charactor
    ///     - rust的字符是unicode字符，长度是4byte -> 32bit
    fn scalar() {
        let _num = 10;
        let _num = 0b0011_1100;
        println!("{_num}");
        let _num = b'A';
        println!("{_num}");
        let _num = 10.0;
        let _bool = true;
        let _c = '😻';
    }
    scalar();

    /// ## rust组合类型
    /// 1. tuple
    ///     - 使用()包裹不确定数量的其他类型
    ///     - ()不包裹类型的时候是一种特殊的值，类型和值都为()，主函数就返回空元组
    ///     - 可以在变量绑定运算符=左边使用()包裹和元组对应数量的变量名进行解包赋值
    /// 2. array
    ///     - 编译期就确定长度和数据类型
    ///     - 类型写法 [type;length]
    ///     - 初始化所有值相同的数组 [value;length]
    fn compound() {
        let _t = (1, 1.1, 'c');
        let _void = ();
        let (_x, _y, _z) = _t;
        println!("{:#?}", _t);

        let _a = [10; 10];
        let _a: [u8; 3] = [b'3'; 3];
        println!("{:#?}", _a);
    }
    compound();
}

fn _ch02_03_function() {
    /// ## rust函数
    /// 1. 使用fn关键字定义
    /// 2. 编译器会警告未使用的函数
    /// 3. 参数列表变量需要指明类型
    /// 4. 函数返回值类型在参数列表后使用-> type 指明
    /// 5. 函数返回值可以使用return关键字也可以使用函数体内的最后一个表达式
    /// 6. 函数不写返回值默认返回空元组
    /// 7. 函数返回!表示该函数永远没法正常结束，对应数学上发散的概念，称之为发散函数
    fn functions() {
        fn add(i: i32, j: i32) -> i32 {
            i + j
        }
        add(3, 4);
        fn _loop() -> ! {
            loop {
                println!("死循环的发散函数");
            }
        }
    }
    functions();
}

fn _ch02_04_statement_expression() {
    /// ## rust语句
    /// 1. rust中没有返回值的就是语句
    /// 2. 变量定义是语句必须要使用;结尾
    /// 3. 其他所有表达式;都是语句
    fn statement() {
        let _i = 10;
    }
    statement();

    /// ## rust表达式
    /// 1. rust中任何有返回值的都是表达式
    /// 2. {}代码块将最后一个表达式作为返回值
    /// 3. loop代码块的返回值特殊，详细见流程控制
    fn expression() {
        let _y = {};
        let _y = {
            let x = 100;
            println!("{x}");
            x * 2
        };
    }
    expression();
}

fn _ch02_05_comment() {
    /// ## rust注释
    /// rust中使用//表示行注释
    fn comment() {
        // 这是行注释
    }
    comment();
}

fn _ch02_06_control_flow() {
    /// ## if
    /// 1. rust支持if else else if关键字
    /// 2. 满足if expression表达式则会进入{}代码块
    fn if_else_elseif() {
        let condition = 100;
        let num = if condition % 2 == 1 && condition > 10 {
            10
        } else if condition % 2 == 0 && condition < 50 {
            20
        } else {
            0
        };
        println!("{num}");
    }
    if_else_elseif();

    /// ## while
    /// 1. while expression 不满足表达式时跳出循环
    /// 2. while 作为表达式只返回空元组()
    fn _while() {
        let mut condition = 100;
        let _n = while condition != 0 {
            condition /= 5;
        };
        println!("_while");
    }
    _while();

    /// ## loop
    /// 1. loop返回值需要使用break关键字指定
    /// 2. loop可以使用标签语法'label_name: loop
    /// 3. break关键字可以关联到标签跳出指定标签的loop
    fn _loop() {
        let mut condition = 100;
        let num = 'out: loop {
            let mut count = 10;
            loop {
                if count > 100 && count % 2 != 1 {
                    condition /= 2;
                    break 'out condition ^ 0b0011_1111_0110_0111_1100_1010;
                } else if count <= 100 && count % 2 == 0 {
                    condition += 1;
                    count += 1;
                } else {
                    count += 3;
                }
            }
        };
        println!("{num}");
    }
    _loop();

    /// ## for
    /// - for 元素 in 集合
    /// - 需要注意所有权问题
    /// - range语法start..=end创建序列
    fn _for() {
        let a = [3; 5];
        // [i32; 5]在栈上，发生copy行为
        for v in a {
            println!("{v}");
        }
        println!("{:#?}", a);
        for v in 0..=10 {
            println!("{v}");
        }
    }
    _for();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch02_01() {
        assert_eq!(_ch02_01_variable_mutability(), ());
    }

    #[test]
    fn ch02_02() {
        assert_eq!(_ch02_02_data_type(), ());
    }

    #[test]
    fn ch02_03() {
        assert_eq!(_ch02_03_function(), ());
    }

    #[test]
    fn ch02_04() {
        assert_eq!(_ch02_04_statement_expression(), ());
    }

    #[test]
    fn ch02_05() {
        assert_eq!(_ch02_05_comment(), ());
    }

    #[test]
    fn ch02_06() {
        assert_eq!(_ch02_06_control_flow(), ());
    }
}
