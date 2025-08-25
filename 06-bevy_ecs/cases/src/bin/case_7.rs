use bevy_ecs::{prelude::*, system::IntoObserverSystem};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

struct AsyncWorld(Arc<RwLock<World>>);
type AsyncRead<'a> = tokio::sync::RwLockReadGuard<'a, World>;
type AsyncWrite<'a> = tokio::sync::RwLockWriteGuard<'a, World>;

impl AsyncWorld {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(World::default())))
    }
    async fn read_lock(&self) -> AsyncRead<'_> {
        self.0.read().await
    }
    async fn write_lock(&mut self) -> AsyncWrite<'_> {
        self.0.write().await
    }
    async fn add_observer<E: Event, B: Bundle, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> Entity {
        self.write_lock().await.add_observer(system).id()
    }
    async fn despwan(&mut self, entity: Entity) -> bool {
        self.write_lock().await.despawn(entity)
    }
}

impl Clone for AsyncWorld {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct PlayerIdf;

#[derive(Bundle)]
struct Player {
    idf: PlayerIdf,
    pos: Position,
    vel: Velocity,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            idf: PlayerIdf,
            pos: Position(0., 0.),
            vel: Velocity(0., 0.),
        }
    }
}

#[derive(Event)]
struct Echo;

#[tokio::main]
async fn main() {
    let mut world = AsyncWorld::new();
    let player;
    {
        let mut world = world.write_lock().await;
        player = world.spawn(Player::default()).id();
    }
    // io异步任务
    {
        let world = world.clone();
        tokio::spawn(async move {
            io_task(world).await;
        });
    }
    // 发布任务
    loop {
        sleep(Duration::from_secs(3)).await;
        let mut world = world.write_lock().await;
        let mut schedule = Schedule::default();
        schedule.add_systems(|query: Query<(&mut Position, &Velocity)>| {
            for (mut pos, vel) in query {
                if let Velocity(0.0, 0.0) = vel {
                    continue;
                }
                pos.0 += vel.0;
                pos.1 += vel.0;
            }
        });
        schedule.run(&mut world);
        world.flush();
        world.trigger_targets(Echo, player);
    }
}

async fn io_task(mut world: AsyncWorld) {
    enum IOCommand {
        Subscribe,
        Unsbucribe,
        Move,
        Stop,
        Invalid,
    }

    impl From<&str> for IOCommand {
        fn from(value: &str) -> Self {
            match value {
                "sub" => Self::Subscribe,
                "unsub" => Self::Unsbucribe,
                "move" => Self::Move,
                "stop" => Self::Stop,
                _ => Self::Invalid,
            }
        }
    }

    println!("可用命令: sub/unsub/move/stop/stop");

    let mut entity: Option<Entity> = None;

    loop {
        let stdin = std::io::stdin();
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            continue;
        }
        match Into::<IOCommand>::into(input.trim()) {
            IOCommand::Subscribe => {
                if let Some(_) = entity {
                    continue;
                }
                entity = Some(
                    world
                        .add_observer(move |trigger: Trigger<Echo>, world: &mut World| {
                            println!("订阅者收到消息");
                            let player = trigger.target();
                            let pos = world.get::<Position>(player);
                            println!("{:?}", pos);
                        })
                        .await,
                );
            }
            IOCommand::Unsbucribe => {
                if let Some(entity) = entity {
                    world.despwan(entity).await;
                };
                entity = None;
            }
            IOCommand::Move => {
                if let None = entity {
                    continue;
                }
                let mut world = world.write_lock().await;
                let mut schedule = Schedule::default();
                schedule.add_systems(|query: Query<&mut Velocity>| {
                    for mut vel in query {
                        vel.0 = 5.;
                        vel.0 = 5.;
                    }
                });
                schedule.run(&mut world);
                world.flush();
            }
            IOCommand::Stop => {
                if let None = entity {
                    continue;
                }
                let mut world = world.write_lock().await;
                let mut schedule = Schedule::default();
                schedule.add_systems(|query: Query<&mut Velocity>| {
                    for mut vel in query {
                        vel.0 = 0.;
                        vel.0 = 0.;
                    }
                });
                schedule.run(&mut world);
                world.flush();
            }
            IOCommand::Invalid => (),
        }
    }
}

