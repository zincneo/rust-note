use bevy_ecs::prelude::*;
use std::{
    io,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

#[derive(Debug, Event)]
struct Echo;

#[derive(Debug, Component)]
struct Position(f32, f32);

#[derive(Debug, Component)]
struct Velocity(f32, f32);

fn main() {
    let world = Arc::new(Mutex::new(World::new()));

    let entity = world
        .lock()
        .unwrap()
        .spawn((Position(0., 0.), Velocity(5., 5.)))
        .id();
    {
        let world = world.clone();
        spawn(move || {
            io_thread(world);
        });
    }

    // 主线程
    loop {
        sleep(Duration::from_secs(5));
        {
            let mut world = world.lock().unwrap();
            world.trigger_targets(Echo, entity);
        }
    }
}

fn io_thread(world: Arc<Mutex<World>>) {
    let stdin = io::stdin();
    let mut entity = None;
    loop {
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            continue;
        }
        println!("输入的内容 {:?}", input);
        match input.trim() {
            // 订阅
            "create" => {
                println!("订阅内容");
                if let Some(_) = entity {
                    continue;
                }
                let c_world = world.clone();
                let mut world = world.lock().unwrap();
                entity = Some(
                    world
                        .add_observer(move |trigger: Trigger<Echo>| {
                            println!("订阅者接收到消息");
                            // 这里world还处于LOCK状态，又去LOCK应该会导致死锁
                            let world = c_world.clone();
                            fn get_pos(entity: Entity, world: Arc<Mutex<World>>) {
                                // 为了避免死锁需要到另外的线程下去读取pos
                                spawn(move || {
                                    let world = world.lock().unwrap();
                                    let pos = world.get::<Position>(entity);
                                    println!("pos: {:?}", pos);
                                });
                            }
                            get_pos(trigger.target(), world);
                        })
                        .id(),
                );
                world.flush();
            }
            // 移动
            "move" => {
                let mut world = world.lock().unwrap();
                let mut schedule = Schedule::default();
                schedule.add_systems(|query: Query<(&mut Position, &Velocity)>| {
                    for (mut pos, vel) in query {
                        pos.0 += vel.0;
                        pos.1 += vel.1;
                    }
                });
                schedule.run(&mut world);
            }
            // 退订
            "destory" => {
                println!("退订内容");
                if let Some(entity) = entity {
                    let mut world = world.lock().unwrap();
                    world.despawn(entity);
                }
            }
            _ => println!("不合法的命令"),
        }
    }
}
