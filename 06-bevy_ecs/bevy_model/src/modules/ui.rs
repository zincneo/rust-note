use std::{
    any::{Any, TypeId},
    ops::DerefMut,
};

use bevy_ecs::entity::Entity;
use colored::Colorize;
use dashmap::DashMap;
use log::{error, info};

type UIData = Option<Box<dyn Any + Send + Sync + 'static>>;

pub trait UITriat: Any + Send + Sync + 'static + PartialEq + Clone {}
impl<T> UITriat for T where T: Any + Send + Sync + 'static + PartialEq + Clone {}

struct UIPair {
    new: UIData,
    old: UIData,
    type_id: TypeId,
}

impl UIPair {
    fn new(new: UIData, old: UIData, type_id: TypeId) -> Self {
        Self { new, old, type_id }
    }
}

pub(crate) struct UIStore {
    inner: DashMap<Entity, UIPair>,
}

impl UIStore {
    pub(crate) fn new() -> Self {
        Self {
            inner: DashMap::with_shard_amount(32),
        }
    }

    pub(crate) fn clear(&self) {
        self.inner.clear();
    }

    pub(crate) fn remove(&self, entity: Entity) -> Option<()> {
        self.inner.remove(&entity)?;
        Some(())
    }

    pub(crate) fn insert<T: UITriat>(&self, entity: Entity, data: T) -> Option<()> {
        if let Some(mut ui_data) = self.inner.get_mut(&entity) {
            let ui_data = ui_data.deref_mut();
            if data.type_id() != ui_data.type_id {
                #[cfg(debug_assertions)]
                error!("Error Type: {:?} {:?}", data.type_id(), ui_data.type_id);
                return None;
            }
            let old = ui_data.old.take()?.downcast::<T>().ok()?;
            if *old != data {
                ui_data.new = Some(Box::new(data.clone()));
            }
            ui_data.old = Some(Box::new(data));
        } else {
            let old_data = data.clone();
            let type_id = data.type_id();
            self.inner.insert(
                entity,
                UIPair::new(Some(Box::new(data)), Some(Box::new(old_data)), type_id),
            );
        }
        Some(())
    }

    pub(crate) fn query<T: UITriat>(&self, entity: Entity) -> Option<T> {
        let ui_data = self.inner.get(&entity);
        let Some(ui_data) = ui_data else {
            #[cfg(debug_assertions)]
            error!(
                "{}[{:?}] The UIStore doesn't contain a UIPair corresponding to the Entity.",
                "Entity".blue(),
                entity
            );
            return None;
        };
        if let None = ui_data.new {
            #[cfg(debug_assertions)]
            info!(
                "{}[{:?}] Data doesn't require updating",
                "Entity".blue(),
                entity
            );
            return None;
        };
        #[cfg(debug_assertions)]
        info!("{}[{:?}] Data has been updated", "Entity".blue(), entity);
        std::mem::drop(ui_data);
        let data = self.inner.get_mut(&entity)?.new.take()?;
        let data = *data.downcast::<T>().ok()?;
        Some(data)
    }
}
