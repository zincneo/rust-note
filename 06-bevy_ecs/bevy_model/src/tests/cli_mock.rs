use crate::{
    Model,
    event::{BasicEvent, ModelEvent},
    ui_store::UIInstanceId,
};
use bevy_ecs::prelude::*;
use log::info;

pub(super) fn auto_mock() {
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    let mut handles = Vec::new();
    let ui_pos = Position(0., 0.);
    let ui_insance_id = Model::add_ui_instance(ui_pos);
    let sender = Model::sender();

    // send event thread
    handles.push(spawn(move || {
        let _ = sender.send(Box::new(Event::Spawn(50, 50, ui_insance_id)));
        sleep(Duration::from_secs(1));
        for _ in 0..5 {
            sleep(Duration::from_secs(2));
            Model::push_system(|s| {
                s.add_systems(move_pos);
            });
            let _ = sender.send(Box::new(BasicEvent::Next));
        }

        let _ = sender.send(Box::new(Event::Despawn));
    }));
    // ui thread
    handles.push(spawn(move || {
        for _ in 0..25 {
            sleep(Duration::from_millis(500));
            let pos = Model::get_ui_instance::<Position>(ui_insance_id);
            info!("[UI Thread: {:?}]", pos);
        }
    }));

    for handle in handles {
        let _ = handle.join();
    }
}

#[derive(Component, Clone, PartialEq, Debug)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[allow(dead_code)]
#[derive(Component)]
struct Size(u16, u16);

fn move_pos(query: Query<(&mut Position, &Velocity, &UIInstanceId)>) {
    for (mut pos, vel, id) in query {
        pos.0 += vel.0;
        pos.1 += vel.1;
        Model::set_instance::<Position>(pos.clone(), *id);
    }
}

#[derive(Bundle)]
struct Rectangle {
    pos: Position,
    vel: Velocity,
    size: Size,
    id: UIInstanceId,
}

impl Rectangle {
    fn new(size: Size, id: UIInstanceId) -> Self {
        Self {
            pos: Position(0., 0.),
            vel: Velocity(10., 10.),
            size,
            id,
        }
    }
}

#[derive(Debug)]
enum Event {
    Spawn(u16, u16, UIInstanceId),
    Despawn,
}

impl ModelEvent for Event {
    fn handle(&self, world: &mut World) -> crate::event::BasicEvent {
        info!("{:?}", self);
        match self {
            Self::Spawn(w, h, id) => Self::spawn(*w, *h, *id, world),
            Self::Despawn => Self::despawn(world),
        }
        crate::event::BasicEvent::Nothing
    }
}

impl Event {
    fn spawn(w: u16, h: u16, id: UIInstanceId, world: &mut World) {
        world.spawn(Rectangle::new(Size(w, h), id));
    }
    fn despawn(world: &mut World) {
        let mut query = world.query_filtered::<Entity, (With<Velocity>, With<Position>)>();
        let entites = query.iter(&world).collect::<Vec<Entity>>();
        for entity in entites {
            world.despawn(entity);
        }
    }
}
