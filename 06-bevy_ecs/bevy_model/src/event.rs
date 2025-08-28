use crate::{MODEL, system::ISystemQueue};
use bevy_ecs::world::World;
use colored::Colorize;
use log::info;
use std::fmt::Debug;

pub trait ModelEvent: Debug + Sync + Send + 'static {
    fn handle(&self, world: &mut bevy_ecs::world::World) -> BasicEvent;
}

#[derive(Debug)]
pub(crate) struct Stop;

#[allow(private_interfaces)]
pub enum BasicEvent {
    Stop(Stop),
    Next,
    Nothing,
}

impl Debug for BasicEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let event_str = match self {
            Self::Stop(Stop) => "--   BasicEvent(Stop)   --".red(),
            Self::Next => "--   BasicEvent(Next)   --".bright_blue(),
            Self::Nothing => "--  BasicEvent(Nothing) --".bright_green(),
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
        #[cfg(debug_assertions)]
        info!("[ECS Thread Handling: {:?}]", self);

        match self {
            BasicEvent::Next => {
                self.handle_next(world);
            }
            _ => (),
        }
        BasicEvent::Nothing
    }
}

#[allow(dead_code)]
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

    fn handle_next(&self, world: &mut World) {
        let mut schedule = MODEL.system_queue.schedule();
        schedule.run(world);
    }
}
