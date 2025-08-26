use bevy_ecs::prelude::*;

#[derive(Resource)]
struct SystemStore(Vec<Box<dyn Fn(&mut Schedule) + Sync + Send>>);

impl SystemStore {
    fn add<T>(&mut self, func: T) -> &mut Self
    where
        T: Fn(&mut Schedule) + Sync + Send + 'static,
    {
        self.0.push(Box::new(func));
        self
    }

    fn get_schedule(&self) -> Schedule {
        let mut schedule = Schedule::default();

        for add in &self.0 {
            add(&mut schedule);
        }
        schedule
    }
}

#[derive(Component, Debug)]
struct Position(f32, f32);

#[derive(Component, Debug)]
struct Velocity(f32, f32);

fn system_1(query: Query<&Position>) {
    for pos in query {
        println!("{:?}", pos);
    }
}

fn system_2(query: Query<&Velocity>) {
    for vel in query {
        println!("{:?}", vel);
    }
}

fn main() {
    let mut world = World::default();

    world.insert_resource(SystemStore(vec![]));
    world.spawn(Position(0., 0.));
    world.spawn((Position(100., 100.), Velocity(10., 10.)));

    let systems = world.get_resource_mut::<SystemStore>();

    let Some(mut systems) = systems else {
        return;
    };

    systems
        .add(|schedule| {
            schedule.add_systems(system_1);
        })
        .add(|schedule| {
            schedule.add_systems(system_2);
        });

    let mut schedule = systems.get_schedule();

    schedule.run(&mut world);
}
