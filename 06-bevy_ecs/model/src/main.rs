use bevy_ecs::prelude::*;
use model::{BasicEvent, IModelEvent, MODEL};

#[derive(Component, Clone, Copy, Debug)]
struct Position(f32, f32);

#[derive(Component, Clone, Copy, Debug)]
struct Velocity(f32, f32);

enum Event {
    Spawn(Position, Velocity),
}

impl IModelEvent for Event {
    fn handle(&self, world: &mut World) -> model::BasicEvent {
        use Event::*;
        match self {
            Spawn(pos, vel) => {
                world.spawn((*pos, *vel));
            }
        }
        model::BasicEvent::Nothing
    }
}

fn system_1(query: Query<&Position>) {
    for pos in query {
        println!("{:?}", pos);
    }
}

fn system_2(query: Query<&Velocity>) {
    for vel in query {
        println!("{:?}", vel);
    }
}

fn system_3(query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in query {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}

fn main() {
    MODEL.init();
    let tx = MODEL.get_tx().unwrap();
    let _ = tx.send(Box::new(Event::Spawn(
        Position(100., 500.),
        Velocity(10., 5.),
    )));
    let _ = std::thread::spawn(move || {
        let stdin = std::io::stdin();

        loop {
            let mut input = String::new();
            if let Err(_) = stdin.read_line(&mut input) {
                continue;
            }
            match input.trim() {
                "step" => {
                    MODEL.clear_systems();
                    MODEL.add_system(|sc| {
                        sc.add_systems(system_1);
                        sc.add_systems(system_2);
                        sc.add_systems(system_3);
                    });
                    let _ = tx.send(BasicEvent::Next.heap());
                }
                "quit" => break,
                _ => (),
            }
        }
    })
    .join();

    MODEL.deinit();
}
