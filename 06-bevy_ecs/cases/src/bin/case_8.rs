use bevy_ecs::prelude::*;

// 无法直接存储System函数的对应类型，因此使用一个参数为可变计划表的闭包来包裹System方法
#[derive(Resource)]
struct SystemStore(Vec<Box<dyn Fn(&mut Schedule) + Sync + Send>>);

#[derive(Component, Debug)]
struct Position(f32, f32);

fn system_1(query: Query<&Position>) {
    for pos in query {
        println!("{:?}", pos);
    }
}

fn main() {
    let mut world = World::default();

    world.insert_resource(SystemStore(vec![]));
    world.spawn(Position(0., 0.));
    world.spawn(Position(100., 100.));

    let systems = world.get_resource_mut::<SystemStore>();

    let Some(mut systems) = systems else {
        return;
    };

    systems.0.push(Box::new(|schedule| {
        schedule.add_systems(system_1);
    }));

    let mut schedule = Schedule::default();

    for add in &systems.0 {
        add(&mut schedule);
    }

    schedule.run(&mut world);
}
