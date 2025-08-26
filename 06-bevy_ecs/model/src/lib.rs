use bevy_ecs::prelude::*;
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::{
    sync::{LazyLock, Mutex, RwLock},
    thread::JoinHandle,
};

pub trait IModelEvent: Sync + Send + 'static {
    fn handle(&self, world: &mut World) -> BasicEvent;
}

#[derive(Clone, Copy)]
pub enum BasicEvent {
    Stop,
    Next,
    Nothing,
}

impl BasicEvent {
    pub fn heap(self) -> Box<Self> {
        Box::new(self)
    }
    fn stop(&self) {
        MODEL.deinit();
    }
    fn next(&self, world: &mut World) -> Option<()> {
        let systems = MODEL.systems.lock().ok()?;
        let mut schedule = systems.schedule();
        schedule.run(world);
        Some(())
    }
}

impl IModelEvent for BasicEvent {
    fn handle(&self, world: &mut World) -> BasicEvent {
        match self {
            BasicEvent::Stop => self.stop(),
            BasicEvent::Next => {
                self.next(world);
            }
            BasicEvent::Nothing => (),
        }
        *self
    }
}

pub struct SystemStore(Vec<Box<dyn Fn(&mut Schedule) + Sync + Send + 'static>>);

impl SystemStore {
    fn schedule(&self) -> Schedule {
        let mut schedule = Schedule::default();
        for add in self.0.iter() {
            add(&mut schedule);
        }
        schedule
    }

    fn add<T>(&mut self, system: T)
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static,
    {
        self.0.push(Box::new(system));
    }
}

impl SystemStore {
    fn clear(&mut self) {
        self.0.clear();
    }
}

pub struct Model {
    world: RwLock<Option<World>>,
    sender: Mutex<Option<Sender<Box<dyn IModelEvent>>>>,
    systems: Mutex<SystemStore>,
    reciver_handle: Mutex<Option<JoinHandle<()>>>,
}

impl Model {
    fn new() -> Self {
        let world = RwLock::new(None);
        let sender = Mutex::new(None);
        let reciver_handle = Mutex::new(None);
        let systems = Mutex::new(SystemStore(vec![]));
        Self {
            world,
            sender,
            systems,
            reciver_handle,
        }
    }

    pub fn add_system<T>(&self, system: T) -> Option<()>
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static,
    {
        self.systems.lock().ok()?.add(system);
        Some(())
    }

    pub fn clear_systems(&self) -> Option<()> {
        self.systems.lock().ok()?.clear();
        Some(())
    }

    pub fn init(&self) -> Option<()> {
        self.deinit()?;
        // Create World
        *self.world.write().ok()? = Some(World::default());
        // Create Sender
        let (tx, rx) = unbounded::<Box<dyn IModelEvent>>();
        *self.sender.lock().ok()? = Some(tx);
        // Create Reciver Thread
        *self.reciver_handle.lock().ok()? = Some(std::thread::spawn(move || {
            Model::reciver_thread(rx);
        }));
        Some(())
    }
    pub fn deinit(&self) -> Option<()> {
        // Destory World
        *self.world.write().ok()? = None;
        // Destory Sender
        {
            let mut inner = self.sender.lock().ok()?;
            if let Some(sender) = inner.as_mut() {
                let _ = sender.send(BasicEvent::Stop.heap());
                *inner = None;
            }
        }
        // Clear Systems
        self.systems.lock().ok()?.clear();
        // Stop Reciver Thread
        let mut inner = self.reciver_handle.lock().ok()?;
        if let Some(handle) = inner.take() {
            let _ = handle.join();
        }
        Some(())
    }

    pub fn get_tx(&self) -> Option<Sender<Box<dyn IModelEvent>>> {
        self.sender.lock().ok()?.clone()
    }

    fn reciver_thread(rx: Receiver<Box<dyn IModelEvent + 'static>>) -> Option<()> {
        let mut world = MODEL.world.write().ok()?;
        let world = world.as_mut()?;
        while let Ok(event) = rx.recv() {
            if let BasicEvent::Stop = event.handle(world) {
                break;
            }
        }
        Some(())
    }
}

pub static MODEL: LazyLock<Model> = LazyLock::new(Model::new);
