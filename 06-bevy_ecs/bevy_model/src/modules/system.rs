use bevy_ecs::schedule::Schedule;
use crossbeam::queue::SegQueue;

pub(crate) trait ISystemQueue {
    fn get_queue() -> Self;
    fn clear(&self);
    fn schedule(&self) -> Schedule;
}

pub(crate) type SystemQueue = SegQueue<Box<dyn Fn(&mut Schedule) + Send + Sync + 'static>>;

impl ISystemQueue for SystemQueue {
    fn get_queue() -> Self {
        SegQueue::new()
    }
    fn clear(&self) {
        while let false = self.is_empty() {
            self.pop();
        }
    }
    fn schedule(&self) -> Schedule {
        let mut schedule = Schedule::default();
        while let false = self.is_empty() {
            if let Some(add) = self.pop() {
                add(&mut schedule);
            }
        }
        schedule
    }
}
