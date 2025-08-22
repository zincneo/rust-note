/*!
# Event
- 事件可以认为是World中通信用的通道，事件可以通过系统函数的参数EventWriter或者EventReader发送和接收
- 事件可以通过world调用add_abserver来监听
- 事件可以通过world调用trigger方式触发
- 事件可以触发到特定的实体上
*/

#![allow(dead_code, unused)]
use bevy_ecs::prelude::*;

#[derive(Debug, Event)]
struct MyEvent {
    message: String,
}

fn writer(mut writer: EventWriter<MyEvent>) {
    writer.write(MyEvent {
        message: "hello!".to_string(),
    });
}

fn reader(mut reader: EventReader<MyEvent>) {
    for event in reader.read() {
        println!("{:?}", event);
    }
}
fn main() {
    let mut world = World::new();

    world.add_observer(|trigger: Trigger<MyEvent>| {
        println!("{}", trigger.event().message);
    });

    world.flush();

    world.trigger(MyEvent {
        message: "hello!".to_string(),
    });

    // 将事件触发到特定实体上
    {
        #[derive(Event)]
        struct Explode;

        let mut world = World::new();
        let entity = world.spawn_empty().id();
        println!("Entity {}", entity);
        let mut count = 0;
        let observer = move |trigger: Trigger<Explode>, mut commands: Commands| {
            println!("{count}");
            count += 1;
            println!("Entity {} goes BOOM!", trigger.target());
            commands.entity(trigger.target()).despawn();
        };
        // 添加的监听器也是world中的实体
        let observer_id = world.add_observer(observer).id();

        world.flush();

        world.trigger_targets(Explode, entity);

        // 销毁对应实体就不会再监听了
        world.despawn(observer_id);

        world.trigger_targets(Explode, entity);
    }
}
