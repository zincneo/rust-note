use bevy_ecs::prelude::*;
use std::sync::RwLockReadGuard;
use std::thread::spawn;
use std::{
    sync::{
        Arc, LazyLock, Mutex, RwLock,
        mpsc::{Receiver, Sender, channel},
    },
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

impl Model {
    fn new() -> Self {
        let (tx, rx) = channel::<ModelEvent>();
        let reciver_handle = Mutex::new(Some(spawn(move || {
            reciver_thread(rx);
        })));
        Self {
            world: Arc::new(RwLock::new(World::default())),
            sender: tx,
            reciver_handle,
        }
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
        // #TODO 通过资源获取将要执行的系统
        let mut schedule = Schedule::default();
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
            }
            _ => println!("Invalid String"),
        }
    }
}

fn mock_output_thread() {}
