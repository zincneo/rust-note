use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use bevy_ecs::{component::Mutable, prelude::*};
use crossbeam::channel::{Receiver, Sender, unbounded};
use log::info;

use crate::{BasicEvent, ModelEvent, SpawnFn, event, model, ui};

#[derive(Debug, Component, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, Component)]
struct Velocity(f32);

#[derive(Debug, Component, Clone, Copy)]
enum HDirection {
    Left,
    Right,
    None,
}

#[derive(Debug, Component, Clone, Copy)]
enum VDirection {
    Top,
    Down,
    None,
}

#[derive(Bundle)]
struct Point {
    pos: Position,
    vel: Velocity,
    hd: HDirection,
    vd: VDirection,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos: Position { x: 0., y: 0. },
            vel: Velocity(0.),
            hd: HDirection::None,
            vd: VDirection::None,
        }
    }
}

#[derive(Debug)]
enum PointEvent {
    Create(Sender<Entity>),
    Destory(Entity),
    Vel(Velocity, Entity),
    HD(HDirection, Entity),
    VD(VDirection, Entity),
    Move(Entity),
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Priority {
    Move,
    Update,
}

impl PointEvent {
    fn create(sender: Sender<Entity>) -> BasicEvent {
        BasicEvent::Spawn(Box::new(|world| world.spawn(Point::default()).id()), sender)
    }

    fn destory(ent: Entity) -> BasicEvent {
        BasicEvent::DeSpawn(ent)
    }

    fn set_component<T: Component<Mutability = Mutable>>(
        value: T,
        entity: Entity,
        world: &mut World,
    ) -> BasicEvent {
        if let Some(mut component) = world.get_mut::<T>(entity) {
            *component = value;
        }
        BasicEvent::Nothing
    }

    fn emit_move(entity: Entity, world: &mut World) -> BasicEvent {
        let event = BasicEvent::Nothing;
        let (Some(vel), Some(hd), Some(vd)) = (
            world.get::<Velocity>(entity),
            world.get::<HDirection>(entity),
            world.get::<VDirection>(entity),
        ) else {
            return event;
        };
        let (vel, hd, vd) = (*vel, *hd, *vd);
        let degree;
        match (hd, vd) {
            (HDirection::None, VDirection::None) => {
                return event;
            }
            (HDirection::Left, VDirection::None) => {
                degree = 0.;
            }
            (HDirection::Right, VDirection::None) => {
                degree = 180.;
            }
            (HDirection::None, VDirection::Top) => {
                degree = 90.;
            }
            (HDirection::None, VDirection::Down) => {
                degree = 270.;
            }
            (HDirection::Left, VDirection::Top) => {
                degree = 45.;
            }
            (HDirection::Left, VDirection::Down) => {
                degree = 315.;
            }
            (HDirection::Right, VDirection::Top) => {
                degree = 135.;
            }
            (HDirection::Right, VDirection::Down) => {
                degree = 225.;
            }
        };
        let Some(mut pos) = world.get_mut::<Position>(entity) else {
            return event;
        };
        let distance = (vel.0 * sin_dp(degree, 2), vel.0 * cos_dp(degree, 2));
        pos.x += distance.0;
        pos.y += distance.1;
        ui::insert(entity, *pos);
        event
    }
}

impl ModelEvent for PointEvent {
    fn handle(&self, world: &mut bevy_ecs::world::World) -> crate::BasicEvent {
        match self {
            Self::Create(sender) => Self::create(sender.clone()),
            Self::Destory(ent) => Self::destory(*ent),
            Self::Vel(vel, entity) => Self::set_component(*vel, *entity, world),
            Self::HD(d, entity) => Self::set_component(*d, *entity, world),
            Self::VD(d, entity) => Self::set_component(*d, *entity, world),
            Self::Move(entity) => Self::emit_move(*entity, world),
        }
    }
}

struct UIStop;

pub(super) fn cli_task() {
    let (tx, rx) = unbounded::<Entity>();
    event::send(Box::new(PointEvent::Create(tx)));
    let point = rx.recv().unwrap();
    ui::insert(point, Position { x: 0., y: 0. });
    let (tx, rx) = unbounded::<UIStop>();
    let mut handles = [
        spawn(move || {
            stdin_thread(point, tx);
        }),
        spawn(move || {
            ui_thread(point, rx);
        }),
    ];

    for handle in handles {
        let _ = handle.join();
    }
}

fn stdin_thread(point: Entity, tx: Sender<UIStop>) {
    let stdin = std::io::stdin();
    let mut vel = Velocity(0.);
    loop {
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            continue;
        }
        match input.trim() {
            "move" => {
                event::send(Box::new(PointEvent::Move(point)));
            }
            "add_vel" => {
                vel.0 += 5.0;
                event::send(Box::new(PointEvent::Vel(vel, point)));
            }
            "left" => {
                event::send(Box::new(PointEvent::HD(HDirection::Left, point)));
            }
            "rigth" => {
                event::send(Box::new(PointEvent::HD(HDirection::Right, point)));
            }
            "top" => {
                event::send(Box::new(PointEvent::VD(VDirection::Top, point)));
            }
            "down" => {
                event::send(Box::new(PointEvent::VD(VDirection::Down, point)));
            }
            "none" => {
                event::send(Box::new(PointEvent::HD(HDirection::None, point)));
                event::send(Box::new(PointEvent::VD(VDirection::None, point)));
            }
            "stop" => {
                break;
            }
            _ => {
                info!("unknown input");
            }
        }
    }
}

fn ui_thread(point: Entity, rx: Receiver<UIStop>) {
    loop {
        if let Some(pos) = ui::query::<Position>(point) {
            info!("get new pos: {:?}", pos);
        }
        sleep(Duration::from_millis(250));
        if let Ok(_) = rx.try_recv() {
            break;
        }
    }
}

fn sin_dp(degree: f32, remain: u32) -> f32 {
    let remain = 10_i32.checked_pow(remain).unwrap() as f32;
    (degree.to_radians().sin() * remain).round() / remain
}
fn cos_dp(degree: f32, remain: u32) -> f32 {
    let remain = 10_i32.checked_pow(remain).unwrap() as f32;
    (degree.to_radians().cos() * remain).round() / remain
}
