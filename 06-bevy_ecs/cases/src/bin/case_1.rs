/*!
# World / Entity / Component

## World

可以理解为一个全局的存储空间，数据都保存在World中

## Entity

实体，保存在World中的基本单位

## Component

组件，实体可以包含任意的组件

### Component Bundle
通过derive宏实现Bundle特征的类型字段必须都是组件，该类型的实例可以作为world.spawn的参数来创建实体
*/

#![allow(dead_code)]
use bevy_ecs::prelude::*;

#[derive(Debug, Default, Component)]
struct Position(f32, f32);

#[derive(Debug, Default, Component)]
struct Velocity(f32, f32);

#[derive(Default, Component)]
struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    position: Position,
    velocity: Velocity,
}

fn main() {
    // 1. 创建世界空间
    let mut world = World::default();

    // 2. 在世界中创建一个实体，该实体包含组件位置、速度
    let _entity = world.spawn((Position(0., 0.), Velocity(0., 0.))).id();
    let entity = world.spawn(PlayerBundle::default()).id();

    // 3. 通过实体在世界中查找到其组件
    let position = world.get::<Position>(entity);

    println!("{:?}", position);

    // 4. 在世界中查找包含指定组件的实体
    let mut query = world.query_filtered::<Entity, With<Velocity>>();
    let _entites = query.iter(&world).collect::<Vec<Entity>>();
}
