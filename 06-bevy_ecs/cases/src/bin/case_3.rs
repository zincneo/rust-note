/*!
# System / Schedule
## System
是函数，用来对World中的数据做操作
## Schedule
计划表，计划表中可以添加System，然后指定对哪个World进行执行
*/

#![allow(dead_code)]

use bevy_ecs::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn main() {
    // 1. 创建世界
    let mut world = World::default();
    // 2. 创建实体
    world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));
    // 3. 创建计划表
    let mut schedule = Schedule::default();
    // 4. 计划表中加入要执行的系统
    schedule.add_systems(movement);
    // 5. 对指定的世界执行一次计划表中的所有系统
    schedule.run(&mut world);
}
