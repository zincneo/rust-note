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

/**
# 资源、过滤器、绑定集和事件
1. 资源 Resource
    - 应用中经常需要用到唯一的资源
    - 例如: 资产集合、渲染器、音频服务以及时间...
2. 查询过滤器 Query
    - Query<QueryData, QueryFilter>
    - QueryData通过一个组件引用组成的元组让System函数可以获取到包含对应数据的实体(也只获取元组中数据的引用)
    - QueryFilter可以通过一个元组指定更多过滤条件
        - With<Component> 过滤出包含某类组件的实体
        - Without<Component> 过滤出不包含某类组件的实体
        - Changed<Component> 过滤出某类值发生了变化的实体
3. 绑定集 Bundle
    - 一个结构体通过`derive(Bundle)`宏实现
    - 字段的类型必须是结构体
    - 作为Wordl::spawn参数传入创建出来的实例会作为Entity加入
4. 事件 Event
    - 为一个或多个system之间提供通信渠道
    - 作为system的参数提供两种类型
    - `EventWriter`用于发送事件
    - `EventReader`用于读取事件
    ```rust
    use bevy_ecs::prelude::*;

    // 资源
    #[derive(Resource, Default)]
    struct Time {
        seconds: u64,
    }

    #[derive(Component, Default, Debug)]
    struct Position(f32, f32);

    #[derive(Component, Default)]
    struct Velocity(f32, f32);

    #[derive(Component, Default)]
    struct Player;

    #[derive(Bundle)]
    struct PlayerBundle(Player, Position, Velocity);

    // 定义自定义的事件，包含需要的数据
    #[derive(Event)]
    struct MyEvent {
        message: String,
    }

    impl Default for PlayerBundle {
        fn default() -> Self {
            Self(Player, Position(0.0, 0.0), Velocity(1.0, 1.0))
        }
    }

    let mut world = World::new();

    world.insert_resource(Time::default());
    world.spawn(PlayerBundle::default());
    world.spawn((Position::default(), Velocity::default()));

    // 通过Res结构体在system函数中使用资源
    fn print_time(time: Res<Time>) {
        println!("{}", time.seconds);
    }

    // 通过Query结构体在system中过滤拿到要处理的组件
    fn player_move_handler(mut player: Query<(&mut Position, &Velocity), With<Player>>) {
        let Some((mut position, velocity)) = player.iter_mut().next() else {
            return;
        };
        position.0 += velocity.0;
        position.1 += velocity.1;
        println!("{:?}", position);
    }

    // 通过EventWrite类型使得system函数可以发送数据
    fn writer(mut writer: EventWriter<MyEvent>) {
        println!("-----write-----");
        writer.send(MyEvent {
            message: "hello!".to_string(),
        });
    }

    // 通过EventReader类型使得触发对应事件的时候执行system函数
    fn reader(mut reader: EventReader<MyEvent>) {
        println!("-----read-----");
        for event in reader.read() {
            println!("---{}---", event.message);
        }
    }

    // 事件要作为资源添加到world实例中
    world.init_resource::<Events<MyEvent>>();

    let mut schedule = Schedule::default();
    schedule.add_systems(print_time);
    schedule.add_systems(player_move_handler);

    // 没有添加到world中作为资源的事件会导致panic
    schedule.add_systems(reader);
    schedule.add_systems(writer);

    println!("{:#?}", schedule.label());

    schedule.run(&mut world);
    ```
*/
#[test]
pub fn addons() {
    use bevy_ecs::prelude::*;

    // 资源
    #[derive(Resource, Default)]
    struct Time {
        seconds: u64,
    }

    #[derive(Component, Default, Debug)]
    struct Position(f32, f32);

    #[derive(Component, Default)]
    struct Velocity(f32, f32);

    #[derive(Component, Default)]
    struct Player;

    #[derive(Bundle)]
    struct PlayerBundle(Player, Position, Velocity);

    // 定义自定义的事件，包含需要的数据
    #[derive(Event)]
    struct MyEvent {
        message: String,
    }

    impl Default for PlayerBundle {
        fn default() -> Self {
            Self(Player, Position(0.0, 0.0), Velocity(1.0, 1.0))
        }
    }

    let mut world = World::new();

    world.insert_resource(Time::default());
    world.spawn(PlayerBundle::default());
    world.spawn((Position::default(), Velocity::default()));

    // 通过Res结构体在system函数中使用资源
    fn print_time(time: Res<Time>) {
        println!("{}", time.seconds);
    }

    // 通过Query结构体在system中过滤拿到要处理的组件
    fn player_move_handler(mut player: Query<(&mut Position, &Velocity), With<Player>>) {
        let Some((mut position, velocity)) = player.iter_mut().next() else {
            return;
        };
        position.0 += velocity.0;
        position.1 += velocity.1;
        println!("{:?}", position);
    }

    // 通过EventWrite类型使得system函数可以发送数据
    fn writer(mut writer: EventWriter<MyEvent>) {
        println!("-----write-----");
        writer.send(MyEvent {
            message: "hello!".to_string(),
        });
    }

    // 通过EventReader类型使得触发对应事件的时候执行system函数
    fn reader(mut reader: EventReader<MyEvent>) {
        println!("-----read-----");
        for event in reader.read() {
            println!("---{}---", event.message);
        }
    }

    // 事件要作为资源添加到world实例中
    world.init_resource::<Events<MyEvent>>();

    let mut schedule = Schedule::default();
    schedule.add_systems(print_time);
    schedule.add_systems(player_move_handler);

    // 没有添加到world中作为资源的事件会导致panic
    schedule.add_systems(reader);
    schedule.add_systems(writer);

    println!("{:#?}", schedule.label());

    schedule.run(&mut world);
}
