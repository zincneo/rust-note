#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;
/**
# 哈希表HashMap
- 创建哈希表
    1. 关联方法`HashMap::new`
    2. 通过迭代器的collect方法直接将值手机未集合类型
- 插入值的方法`insert`，如果值已经存在则会更新值
- 查询HashMap
    - 使用get方法，返回值是包裹原始类型引用的Option
- 更新HashMap
    1. 使用`insert`方法，返回的是包裹旧值的Option
    2. 使用`entry`方法和`or_inser`方法链式调用，可以返回索引对应元素的可变引用
*/
pub fn f02_hashmap() {
    // 1. 通过关联方法创建
    {
        let mut my_gems = HashMap::new();

        // 将宝石类型和对应的数量写入表中
        my_gems.insert("红宝石", 1);
        my_gems.insert("蓝宝石", 2);
        my_gems.insert("河边捡的误以为是宝石的破石头", 18);
    }

    // 2. 使用其它可以迭代的类型收集
    {
        let teams_list = vec![
            ("teamA".to_string(), 100),
            ("teamB".to_string(), 10),
            ("teamC".to_string(), 50),
        ];

        let mut teams_map = teams_list.into_iter().collect::<HashMap<_, _>>();
        println!("{:?}", teams_map);
    }

    // 3. 常用方法
    {
        let mut scores = HashMap::new();
        // 1. inser方法
        // 返回值是包裹旧值的Option
        let old = scores.insert("Blue", 10);
        assert_eq!(old, None);
        let old = scores.insert("Blue", 20);
        assert_eq!(old, Some(10));
        // 查询新插入的值
        let new = scores.get("Blue");
        assert_eq!(new, Some(&20));
        let new = scores.get("Yellow");
        assert_eq!(new, None);

        // 2. entry方法，查询对应的值，需要结合or_inser方法链式调用
        // 查询Yellow对应的值，若不存在则插入新值
        let v = scores.entry("Yellow").or_insert(5);
        assert_eq!(*v, 5); // 不存在，插入5
        // 查询Yellow对应的值，若不存在则插入新值
        let v = scores.entry("Yellow").or_insert(50);
        assert_eq!(*v, 5); // 已经存在，因此50没有插入
    }

    // 4. 迭代所有元素
    {
        let teams_list = vec![
            ("teamA".to_string(), 100),
            ("teamB".to_string(), 10),
            ("teamC".to_string(), 50),
        ];

        let teams_map = teams_list.into_iter().collect::<HashMap<_, _>>();
        for (key, value) in &teams_map {
            println!("{key} : {value}");
        }
    }
}
