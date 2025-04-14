#![allow(dead_code)]
/**
# Bevy_ECS

1. 组件 Component
    - 通过`derive(Component)`宏定义的结构体或者枚举
    - 作为数据存储在World结构体的实例中
    - 一个组件一定只属于一个实体
2. 世界 World
    - 实体、组件、资源被存储在这个结构体中
    - 该结构体类似标准库提供的集合类型HashMap还有Vec
    - 该结构体提供了增删改查的API
3. 实体 Entity
    - 在World中作为标识符用来关联>=0个组件
4. 系统 System
    - 普通的Rust方法
    - 通过Rust的类型系统可以通过参数类型来确定需要向系统发送哪些参数
    - 而且bevy_ecs可以根据参数类型将函数并行运行
5. 执行计划Schedules
    - 通过add_system方法加入要执行的系统函数
    - 通过run方法传入一个可变的world引用使得所有的系统函数对world执行一次
    - bevy_ecs本身有做平行化优化
    ```rust
    // 定义组件
    #[derive(Component, Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    // 定义组件
    #[derive(Component)]
    enum Type {
        A,
        B,
    }

    #[derive(Component, Debug)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    // 创建一个system函数，通过参数类型Query可以让函数获取到数据
    fn movement(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut query {
            (position.x, position.y) = (position.x + velocity.x, position.y + velocity.y);
            println!("{:#?}", position);
        }
    }

    let mut world = World::new();
    // 在world中添加一个实体
    world.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Type::A,
    ));
    let mut schedule = Schedule::default();
    // 给计划表中添加要执行的系统
    schedule.add_systems(movement);
    loop {
        thread::sleep(Duration::from_secs(1));
        // 计划表中的所有系统对指定的world执行一次
        schedule.run(&mut world);
    }
    ```
*/
#[test]
pub fn basic() {
    use bevy_ecs::prelude::*;
    use std::thread;
    use std::time::Duration;
    #[derive(Component, Debug)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Component)]
    enum Type {
        A,
        B,
    }

    #[derive(Component, Debug)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    // This system moves each entity with a Position and Velocity component
    fn movement(mut query: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut query {
            (position.x, position.y) = (position.x + velocity.x, position.y + velocity.y);
            println!("{:#?}", position);
        }
    }

    let mut world = World::new();
    world.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Type::A,
    ));
    let mut schedule = Schedule::default();
    schedule.add_systems(movement);
    loop {
        thread::sleep(Duration::from_secs(1));
        schedule.run(&mut world);
    }
}
