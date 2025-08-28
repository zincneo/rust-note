mod event;
mod system;
mod ui_store;
use arc_swap::ArcSwapOption;
use bevy_ecs::{schedule::Schedule, world::World};
use crossbeam::channel::{Receiver, Sender, unbounded};
use dashmap::DashMap;
use event::*;
use std::{
    ops::DerefMut,
    sync::{Arc, LazyLock},
    thread::{JoinHandle, spawn},
};
use system::*;
use ui_store::*;

pub struct Model {
    sender: ArcSwapOption<Sender<Box<dyn ModelEvent>>>,
    handle: ArcSwapOption<JoinHandle<()>>,
    system_queue: SystemQueue,
    ui_store: UIStore,
}

impl Model {
    fn new() -> Self {
        Self {
            sender: ArcSwapOption::from(None),
            handle: ArcSwapOption::from(None),
            system_queue: SystemQueue::get_queue(),
            ui_store: DashMap::with_shard_amount(32),
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
        self.ui_store.clear();
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

    pub fn add_ui_instance<T>(instance: T) -> UIInstanceId
    where
        T: UITrait,
    {
        let instance_id = UIInstanceId::new_instance::<T>();
        MODEL.ui_store.insert(instance_id, UIState::new(instance));
        instance_id
    }

    pub fn remove_ui_instance(instance_id: UIInstanceId) {
        UIInstanceId::destory_instance(instance_id);
        MODEL.ui_store.remove(&instance_id);
    }

    pub fn get_ui_instance<T>(instance_id: UIInstanceId) -> Option<T>
    where
        T: UITrait,
    {
        if MODEL.ui_store.get(&instance_id)?.new_is_none() {
            return None;
        }
        MODEL
            .ui_store
            .get_mut(&instance_id)?
            .deref_mut()
            .take_new::<T>()
    }

    pub fn set_instance<T>(instance: T, instance_id: UIInstanceId) -> Option<()>
    where
        T: UITrait,
    {
        MODEL
            .ui_store
            .get_mut(&instance_id)?
            .deref_mut()
            .set_new(instance)
    }
}

pub static MODEL: LazyLock<Model> = LazyLock::new(Model::new);
