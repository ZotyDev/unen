use std::{any::TypeId, collections::HashMap};

use crate::system::SystemContainer;

pub trait Stage: Send + Sync + 'static {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub struct AppStageStart;
pub struct AppStageStep;
pub struct AppStageStop;

impl Stage for AppStageStart {}
impl Stage for AppStageStep {}
impl Stage for AppStageStop {}

#[derive(Default)]
pub struct StageContainer {
    container: HashMap<TypeId, SystemContainer>,
}

impl StageContainer {
    pub fn insert<S: Stage>(&mut self) {
        self.container.entry(TypeId::of::<S>()).or_default();
    }

    pub fn get<S: Stage>(&mut self) -> &mut SystemContainer {
        self.container.entry(TypeId::of::<S>()).or_default()
    }
}
