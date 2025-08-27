use bevy_ecs::world::World;

use crate::{MODEL, system::ISystemQueue};

pub trait ModelEvent: Sync + Send + 'static {
    fn handle(&self, world: &mut bevy_ecs::world::World) -> BasicEvent;
}

pub(crate) struct Stop;

#[allow(private_interfaces)]
pub enum BasicEvent {
    Stop(Stop),
    Next,
    Nothing,
}

impl ModelEvent for BasicEvent {
    fn handle(&self, world: &mut World) -> BasicEvent {
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
