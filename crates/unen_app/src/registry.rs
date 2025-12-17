use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Registry {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Registry {
    pub fn add<T: 'static + Send + Sync>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }
}
