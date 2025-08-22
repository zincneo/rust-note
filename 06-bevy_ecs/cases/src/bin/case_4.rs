/*!
## Query/Res
# Query
用于加入计划表的系统函数的第一个参数query(本身Query类型实现了迭代器特征)
1. Query的泛型参数第一个指定要查询的组件类型(以引用形式取到，可以是包含多个组件的元组)
2. Query的第二个泛型参数用来指定过滤方式
    1. With类型表示对应实体必须包含指定类型的组件才会被选中
    2. Changed类型表示当该组件值变动的实体才会被选中(对比同一计划表上一次run)
    3. Without类型表示对应实体不包含指定类型的组件才会被选中
    4. Added类型表示添加了该组件的实体才会被选中(对比同一计划表上一次run)
# Res
用于加入计划表的系统函数的第一个参数res(这里就很简单直接指定对应的资源类型作为泛型参数获取唯一的资源)
*/

#![allow(dead_code)]
#![allow(unused)]
use bevy_ecs::prelude::*;

#[derive(Debug, Component)]
struct Position(f32, f32);

#[derive(Debug, Component)]
struct Velocity(f32, f32);

// 只会查询到同时包含Poisition和Velocity组件的实体中的对应组件
fn print_pos_vel(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        println!("{:?}", position);
        println!("{:?}", velocity);
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Alive;
// 找到所有包含Player组件且不包含Alive组件的实体中的Position组件
fn system(query: Query<&Position, (With<Player>, Without<Alive>)>) {
    for position in &query {}
}

// 找到Velocity组件变更的实体中的Position组件(变更指同一个计划表上一次run)
fn system_changed(query: Query<&Position, Changed<Velocity>>) {
    for position in &query {}
}

// 找到添加了Velocity组件的实体中的Position组件(对比同一个计划表上一次run)
fn system_added(query: Query<&Position, Added<Velocity>>) {
    for position in &query {}
}

fn main() {
    let mut world = World::default();
    world.spawn((Position(0., 0.), Velocity(0., 0.)));
    world.spawn((Velocity(0., 0.)));
    world.spawn((Position(0., 0.)));

    let mut schedule = Schedule::default();
    schedule.add_systems(print_pos_vel);
    schedule.run(&mut world);
}
