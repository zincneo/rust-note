use bevy_ecs::component::Component;
use dashmap::{DashMap, DashSet};
use std::{
    any::{Any, TypeId},
    ops::DerefMut,
    sync::LazyLock,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Component, Debug)]
pub struct UIInstanceId(TypeId, u16);

pub trait UITrait: Any + PartialEq + Clone + Send + Sync + 'static {}
impl<T: Any + PartialEq + Clone + Send + Sync + 'static> UITrait for T {}

impl UIInstanceId {
    pub(crate) fn new_instance<T>() -> UIInstanceId
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();
        let instance_id;
        let Some(mut count) = TYPE_INSTANCE_MAP.get_mut(&type_id) else {
            instance_id = UIInstanceId(type_id, 0);
            INSTANCE_SET.insert(instance_id);
            return instance_id;
        };
        let count = count.deref_mut();
        while let Some(_) = INSTANCE_SET.get(&UIInstanceId(type_id, *count)) {
            *count = count.wrapping_add(1);
        }
        instance_id = UIInstanceId(type_id, *count);
        INSTANCE_SET.insert(instance_id);
        TYPE_INSTANCE_MAP.insert(type_id, *count);
        instance_id
    }
    pub(crate) fn destory_instance(instancd_id: UIInstanceId) -> Option<UIInstanceId> {
        INSTANCE_SET.remove(&instancd_id)
    }
}

pub(crate) static TYPE_INSTANCE_MAP: LazyLock<DashMap<TypeId, u16>> = LazyLock::new(DashMap::new);
pub(crate) static INSTANCE_SET: LazyLock<DashSet<UIInstanceId>> = LazyLock::new(DashSet::new);

pub(crate) struct UIState {
    new: Option<Box<dyn Any + Send + Sync + 'static>>,
    old: Option<Box<dyn Any + Send + Sync + 'static>>,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            new: None,
            old: None,
        }
    }
}

impl UIState {
    pub(crate) fn new<T>(instance: T) -> Self
    where
        T: UITrait,
    {
        let old_instance = instance.clone();
        UIState {
            new: Some(Box::new(instance)),
            old: Some(Box::new(old_instance)),
        }
    }

    pub(crate) fn new_is_none(&self) -> bool {
        self.new.is_none()
    }

    pub(crate) fn take_new<T>(&mut self) -> Option<T>
    where
        T: UITrait,
    {
        let old = *self.new.take()?.downcast::<T>().ok()?;
        self.old = Some(Box::new(old.clone()));
        Some(old)
    }

    pub(crate) fn set_new<T>(&mut self, instance: T) -> Option<()>
    where
        T: UITrait,
    {
        let old = self.take_new::<T>();
        match old {
            Some(old) if old == instance => (),
            _ => {
                let instance = instance.clone();
                self.new = Some(Box::new(instance));
            }
        }
        self.set_old(instance);
        Some(())
    }

    fn set_old<T>(&mut self, instance: T)
    where
        T: UITrait,
    {
        self.old = Some(Box::new(instance));
    }
}

pub(crate) type UIStore = DashMap<UIInstanceId, UIState>;
