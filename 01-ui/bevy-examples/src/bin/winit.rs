use bevy::prelude::*;
/**
# bevy最小窗口
- bevy空场景占用大量内存和资源主要是wgpu导致的
- WinitPlugin插件依赖的最少插件(运行时能够正常运行)
    1. MinimalPlugins
    2. AssetPlugin
    3. AccessibilityPlugin
    4. ImagePlugin
    5. InputPlugin
- windows下测试空窗口只占用5MB左右内存，并且配置了计划表循环一秒执行60次的情况下几乎不会占用CPU
- 但是这样其实什么都不会渲染，Camera也没法使用
*/
use std::time::Duration;

fn main() {
    use bevy::{
        a11y::AccessibilityPlugin,
        app::ScheduleRunnerPlugin,
        input::InputPlugin,
        winit::{WakeUp, WinitPlugin},
    };
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                title: "Minimal plugins required to create a window".into(),
                resolution: (800, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
        .add_plugins((
            AssetPlugin::default(),
            AccessibilityPlugin::default(),
            ImagePlugin::default(),
            InputPlugin::default(),
        ))
        .add_plugins(WinitPlugin::<WakeUp>::default())
        .run();
}
