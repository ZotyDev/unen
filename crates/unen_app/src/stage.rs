use std::{any::TypeId, collections::HashMap};

use crate::system::SystemContainer;

pub trait Stage: Send + Sync + 'static {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

mod _app_stages {
    use super::Stage;

    pub struct AppStageStart;
    pub struct AppStageStep;
    pub struct AppStageStop;

    impl Stage for AppStageStart {}
    impl Stage for AppStageStep {}
    impl Stage for AppStageStop {}

    pub const START: AppStageStart = AppStageStart;
    pub const STEP: AppStageStep = AppStageStep;
    pub const STOP: AppStageStop = AppStageStop;
}

pub use _app_stages::{START, STEP, STOP};

#[derive(Default)]
pub struct StageContainer {
    container: HashMap<TypeId, SystemContainer>,
}

impl StageContainer {
    pub fn insert<S: Stage>(&mut self, _stage: S) {
        self.container.entry(TypeId::of::<S>()).or_default();
    }

    pub fn get<S: Stage>(&mut self, _stage: S) -> &mut SystemContainer {
        self.container.entry(TypeId::of::<S>()).or_default()
    }
}
