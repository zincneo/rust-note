mod event;
mod system;
use arc_swap::ArcSwapOption;
use bevy_ecs::{schedule::Schedule, world::World};
use crossbeam::channel::{Receiver, Sender, unbounded};
use event::*;
use std::{
    sync::{Arc, LazyLock},
    thread::{JoinHandle, spawn},
};
use system::*;

pub struct Model {
    sender: ArcSwapOption<Sender<Box<dyn ModelEvent>>>,
    handle: ArcSwapOption<JoinHandle<()>>,
    system_queue: SystemQueue,
}

impl Model {
    fn new() -> Self {
        Self {
            sender: ArcSwapOption::from(None),
            handle: ArcSwapOption::from(None),
            system_queue: SystemQueue::get_queue(),
        }
    }

    pub fn init(&self) {
        self.deinit();
        let (tx, rx) = unbounded::<Box<dyn ModelEvent>>();
        let tx = Some(Arc::new(tx));
        self.sender.store(tx);
        let handle = Some(Arc::new(spawn(move || {
            Model::world_thrad(rx);
        })));
        self.handle.store(handle);
    }

    pub fn deinit(&self) -> Option<()> {
        self.system_queue.clear();
        self.sender.swap(None)?.send(BasicEvent::stop()).ok()?;
        let handle = self.handle.swap(None)?;
        match std::sync::Arc::try_unwrap(handle) {
            Ok(handle) => Some(handle.join().ok()?),
            Err(_) => {
                println!("Arc<JoinHandle<()>> is held by multiple owners.");
                None
            }
        }
    }

    fn world_thrad(rx: Receiver<Box<dyn ModelEvent>>) {
        let mut world = World::default();
        while let Ok(event) = rx.recv() {
            if let BasicEvent::Stop(Stop) = event.handle(&mut world) {
                break;
            }
        }
    }

    pub fn sender() -> Arc<Sender<Box<dyn ModelEvent>>> {
        match MODEL.sender.load_full() {
            Some(sender) => sender,
            None => {
                panic!("Earlier than MODEL.init() or later than MODEL.deinit()");
            }
        }
    }

    pub fn push_system<T>(system: T)
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static,
    {
        MODEL.system_queue.push(Box::new(system));
    }
}

pub static MODEL: LazyLock<Model> = LazyLock::new(Model::new);
