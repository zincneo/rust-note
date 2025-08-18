use bevy_ecs::prelude::*;
use godot::global::godot_print_rich;

pub struct BevyApp {
    world: World,
    schedule: Schedule,
}

impl BevyApp {
    pub fn new() -> Self {
        let mut app = Self {
            world: World::new(),
            schedule: Schedule::default(),
        };
        app.schedule.run(&mut app.world);
        app
    }
}

impl Drop for BevyApp {
    fn drop(&mut self) {
        godot_print_rich!("[BevyApp]: drop() called");
    }
}
