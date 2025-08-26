use bevy_ecs::prelude::*;
use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::HashMap;
use std::sync::RwLockReadGuard;
use std::thread::spawn;
use std::{
    sync::{Arc, LazyLock, Mutex, RwLock},
    thread::JoinHandle,
};

enum ModelEvent {
    Stop,
    Next,
}

struct Model {
    world: Arc<RwLock<World>>,
    sender: Sender<ModelEvent>,
    reciver_handle: Mutex<Option<JoinHandle<()>>>,
}

#[derive(Resource, Default)]
struct ModelSystems(HashMap<String, Box<dyn Fn(&mut Schedule) + Send + Sync>>);

#[allow(dead_code)]
impl Model {
    fn new() -> Self {
        let (tx, rx) = unbounded::<ModelEvent>();
        let reciver_handle = Mutex::new(Some(spawn(move || {
            reciver_thread(rx);
        })));
        let mut world = World::default();
        world.insert_resource(ModelSystems::default());
        let model = Self {
            world: Arc::new(RwLock::new(world)),
            sender: tx,
            reciver_handle,
        };
        model
    }

    fn stop(&self) {
        let Ok(mut handle) = self.reciver_handle.lock() else {
            return;
        };
        let Some(handle) = handle.take() else {
            return;
        };
        let _ = handle.join();
    }

    pub fn read() -> Option<RwLockReadGuard<'static, World>> {
        if let Ok(read) = MODEL.world.read() {
            Some(read)
        } else {
            None
        }
    }

    pub fn next(&self) {
        let Ok(mut world) = self.world.write() else {
            return;
        };
        let mut schedule = Schedule::default();
        let Some(systems) = world.get_resource::<ModelSystems>() else {
            return;
        };
        for (_, add) in systems.0.iter() {
            add(&mut schedule);
        }
        schedule.run(&mut world);
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        self.stop();
    }
}

static MODEL: LazyLock<Model> = LazyLock::new(Model::new);

fn main() {
    spawn(|| {
        mock_ui_thread();
    });
    spawn(|| {
        mock_output_thread();
    });
    MODEL.stop();
}

fn reciver_thread(rx: Receiver<ModelEvent>) {
    use ModelEvent::*;
    while let Ok(event) = rx.recv() {
        match event {
            Stop => break,
            Next => MODEL.next(),
        }
    }
}

fn mock_ui_thread() {
    let stdin = std::io::stdin();
    let tx = MODEL.sender.clone();
    loop {
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            continue;
        }
        match input.trim() {
            "stop" => {
                let _ = tx.send(ModelEvent::Stop);
                break;
            }
            "next" => {
                let _ = tx.send(ModelEvent::Next);
            }
            _ => println!("无效输入，请输入 'next' 或 'stop'"),
        }
    }
}

fn mock_output_thread() {}
