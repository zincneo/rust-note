mod modules {
    pub mod event;
    pub mod system;
    pub mod ui;
}

use std::{
    sync::{Arc, LazyLock},
    thread::{JoinHandle, spawn},
};

use arc_swap::ArcSwapOption;
use bevy_ecs::{entity::Entity, schedule::Schedule, world::World};
use crossbeam::channel::{Receiver, Sender, unbounded};
use log::{error, info};
pub use modules::event::*;
use modules::system::*;
pub use modules::ui::*;

#[cfg(test)]
#[path = "tests/tests.rs"]
mod tests;

struct Model {
    sender: ArcSwapOption<Sender<Box<dyn ModelEvent>>>,
    handle: ArcSwapOption<JoinHandle<()>>,
    system_queue: SystemQueue,
    ui_store: UIStore,
}

impl Model {
    fn new() -> Self {
        #[cfg(debug_assertions)]
        let _ = env_logger::builder().is_test(true).try_init();

        Self {
            sender: ArcSwapOption::from(None),
            handle: ArcSwapOption::from(None),
            system_queue: SystemQueue::get_queue(),
            ui_store: UIStore::new(),
        }
    }
}

fn ecs_thread(rx: Receiver<Box<dyn ModelEvent>>) {
    let mut world = World::default();
    while let Ok(event) = rx.recv() {
        let event = event.handle(&mut world);
        if let BasicEvent::Stop(Stop) = event {
            break;
        } else {
            event.handle(&mut world);
        }
    }
}

pub mod model {
    use super::*;
    pub fn init() {
        deinit();
        let (tx, rx) = unbounded::<Box<dyn ModelEvent>>();
        let tx = Some(Arc::new(tx));
        MODEL.sender.store(tx);
        let handle = Some(Arc::new(spawn(move || {
            ecs_thread(rx);
        })));
        MODEL.handle.store(handle);
    }

    pub fn deinit() -> Option<()> {
        MODEL.system_queue.clear();
        MODEL.ui_store.clear();
        MODEL.sender.swap(None)?.send(BasicEvent::stop()).ok()?;
        let handle = MODEL.handle.swap(None)?;
        if let Ok(handle) = std::sync::Arc::try_unwrap(handle) {
            #[cfg(debug_assertions)]
            info!("wait ecs thread stop");
            Some(handle.join().ok()?)
        } else {
            #[cfg(debug_assertions)]
            error!("Arc<JoinHandle<()>> is held by multiple owners.");
            panic!("Arc<JoinHandle<()>> is held by multiple owners.");
        }
    }

    pub fn push_system<T>(system: T)
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static,
    {
        MODEL.system_queue.push(Box::new(system));
    }
}

pub mod event {
    use super::*;
    pub fn send(event: Box<dyn ModelEvent>) -> Option<()> {
        MODEL.sender.load_full()?.send(event).ok()?;
        Some(())
    }
}

pub mod ui {
    use super::*;
    pub fn insert<T: UITriat>(entity: Entity, data: T) -> Option<()> {
        MODEL.ui_store.insert::<T>(entity, data)
    }

    pub fn query<T: UITriat>(entity: Entity) -> Option<T> {
        MODEL.ui_store.query::<T>(entity)
    }

    pub fn remove(entity: Entity) -> Option<()> {
        MODEL.ui_store.remove(entity)
    }
}

static MODEL: LazyLock<Model> = LazyLock::new(Model::new);
