use godot::{
    classes::{INode, Node},
    prelude::*,
};

mod bevy_app;
use bevy_app::BevyApp;

struct SingletonExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SingletonExtension {}

#[derive(GodotClass)]
#[class(tool, base=Node)]
struct SingletonPlugin {
    base: Base<Node>,
    bevy_app: Option<BevyApp>,
}

#[godot_api]
impl SingletonPlugin {
    #[func]
    fn install_bevy(&mut self) {
        if let Some(_) = self.bevy_app {
            self.bevy_app = None
        };
        self.bevy_app = Some(BevyApp::new());
        godot_print!("[Singleton] install_bevy() called");
    }

    #[func]
    fn uninstall_bevy(&mut self) {
        self.bevy_app = None;
        godot_print!("[Singleton] uninstall_bevy() called");
    }
}

#[godot_api]
impl INode for SingletonPlugin {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            bevy_app: None,
        }
    }
}
