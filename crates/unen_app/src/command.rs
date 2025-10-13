use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Command: Send + Sync + 'static {
    type Target: Send + Sync + 'static;
    fn apply(self: Box<Self>, target: &mut Self::Target);
}

#[derive(Default)]
pub struct CommandRegistry {
    queues: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CommandRegistry {
    pub fn add<C: Command>(&mut self, command: C) {
        let queue = self
            .queues
            .entry(TypeId::of::<C::Target>())
            .or_insert_with(|| Box::new(Vec::<Box<dyn Command<Target = C::Target>>>::new()));

        let queue = queue
            .downcast_mut::<Vec<Box<dyn Command<Target = C::Target>>>>()
            .unwrap();

        queue.push(Box::new(command));
    }

    pub fn apply_all<T: 'static + Send + Sync>(&mut self, target: &mut T) {
        if let Some(queue) = self.queues.remove(&TypeId::of::<T>()) {
            let queue = *queue
                .downcast::<Vec<Box<dyn Command<Target = T>>>>()
                .unwrap();
            for command in queue {
                command.apply(target);
            }
        }
    }
}
