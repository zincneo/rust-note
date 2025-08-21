/*!
# Resource

应用中通常会使用一些独一无二的资源
*/

use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
struct Time {
    seconds: i32,
}

fn main() {
    let mut world = World::default();
    // 1. 插入资源
    world.insert_resource(Time::default());
    // 2. 获取资源
    let time = world.get_resource::<Time>();
    println!("{}", time.unwrap().seconds);
}
