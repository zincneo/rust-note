use crate::{MODEL, modules::system::ISystemQueue};
use bevy_ecs::{entity::Entity, world::World};
use colored::Colorize;
use crossbeam::channel::Sender;
use log::{error, info};
use std::fmt::Debug;

pub trait ModelEvent: Debug + Sync + Send + 'static {
    fn handle(&self, world: &mut bevy_ecs::world::World) -> BasicEvent;
    fn info(&self) {
        #[cfg(debug_assertions)]
        info!("[ECS Thread Handling: {:?}]", self);
    }
}

#[derive(Debug)]
pub(crate) struct Stop;

pub type SpawnFn = Box<dyn Fn(&mut World) -> Entity + Send + Sync + 'static>;

#[allow(private_interfaces)]
pub enum BasicEvent {
    Stop(Stop),
    Next,
    Nothing,
    Spawn(SpawnFn, Sender<Entity>),
    DeSpawn(Entity),
}

impl Debug for BasicEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let event_str = match self {
            Self::Stop(Stop) => "--   BasicEvent(Stop)   --".red(),
            Self::Next => "--   BasicEvent(Next)   --".bright_blue(),
            Self::Nothing => "--  BasicEvent(Nothing) --".green(),
            Self::Spawn(_, _) => "--   BasicEvent(Spawn)  --".bright_green(),
            Self::DeSpawn(_) => "--  BasicEvent(DeSpan)  --".bright_red(),
        };
        write!(
            f,
            "{} {} {}",
            "[".yellow().bold(),
            event_str,
            "]".yellow().bold()
        )
    }
}

impl ModelEvent for BasicEvent {
    fn handle(&self, world: &mut World) -> BasicEvent {
        self.info();
        match self {
            BasicEvent::Next => {
                self.handle_next(world);
            }
            BasicEvent::Spawn(bundle, sender) => {
                self.handle_spawn(bundle, sender, world);
            }
            BasicEvent::DeSpawn(entity) => {
                self.handle_despawn(entity, world);
            }
            _ => (),
        }
        BasicEvent::Nothing
    }
}

impl BasicEvent {
    pub(crate) fn stop() -> Box<Self> {
        Box::new(Self::Stop(Stop))
    }

    pub fn next() -> Box<Self> {
        Box::new(Self::Next)
    }

    pub fn nothing() -> Box<Self> {
        Box::new(Self::Nothing)
    }

    pub fn spawn(spawn_fn: SpawnFn, sender: Sender<Entity>) -> Box<Self> {
        Box::new(Self::Spawn(spawn_fn, sender))
    }

    pub fn despawn(entity: Entity) -> Box<Self> {
        Box::new(Self::DeSpawn(entity))
    }
}

#[allow(dead_code)]
impl BasicEvent {
    fn handle_next(&self, world: &mut World) {
        let mut schedule = MODEL.system_queue.schedule();
        schedule.run(world);
    }

    fn handle_spawn(&self, spawn_fn: &SpawnFn, sender: &Sender<Entity>, world: &mut World) {
        let entity = spawn_fn(world);
        if let Err(e) = sender.send(entity) {
            error!("{:?}", e);
        }
    }

    fn handle_despawn(&self, entity: &Entity, world: &mut World) {
        world.despawn(*entity);
        crate::ui::remove(*entity);
    }
}
