use crate::{
    app::AppState,
    stage::{AppStageStart, AppStageStep, AppStageStop, StageContainer},
};

pub trait Runner: Send + Sync + 'static {
    fn run(&mut self, data: RunnerData) -> RunnerData;
}

pub struct RunnerData {
    pub stages: StageContainer,
    pub state: AppState,
}

#[derive(Default)]
pub struct MininalRunner;

impl MininalRunner {
    pub fn new() -> Box<dyn Runner> {
        Box::new(Self {})
    }
}

impl Runner for MininalRunner {
    fn run(&mut self, data: RunnerData) -> RunnerData {
        let RunnerData {
            mut stages,
            mut state,
        } = data;

        state = stages.get::<AppStageStart>().execute_all(state);
        state.running = true;

        while state.running {
            state = stages.get::<AppStageStep>().execute_all(state);
        }

        state = stages.get::<AppStageStop>().execute_all(state);

        RunnerData { stages, state }
    }
}
