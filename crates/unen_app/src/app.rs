use crate::{
    runner::{MininalRunner, Runner, RunnerData},
    stage::StageContainer,
};

pub fn create_app() -> App {
    App::default()
}

#[derive(Debug, Default)]
pub struct AppState {
    pub running: bool,
}

pub struct App {
    state: AppState,
    runner: Box<dyn Runner>,
    stages: StageContainer,
}

impl Default for App {
    fn default() -> Self {
        Self {
            runner: MininalRunner::new(),
            state: AppState::default(),
            stages: StageContainer::default(),
        }
    }
}

impl App {
    /// Starts the application, consumes `self`.
    pub fn run(self) -> Self {
        let App {
            state,
            mut runner,
            stages,
        } = self;

        let mut data = RunnerData { stages, state };

        data = runner.as_mut().run(data);

        let RunnerData { state, stages } = data;

        Self {
            state,
            runner,
            stages,
        }
    }

    pub fn system(&mut self) -> &mut Self {
        self
    }

    pub fn stage(&mut self) -> &mut Self {
        self
    }
}
