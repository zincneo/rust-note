#![allow(dead_code)]

use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
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

#[test]
pub fn basic() {
    use std::thread;
    use std::time::Duration;
    let mut world = World::new();
    world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));
    let mut schedule = Schedule::default();
    schedule.add_systems(movement);
    loop {
        thread::sleep(Duration::from_secs(1));
        schedule.run(&mut world);
    }
}
