use crate::{
    runner::{MininalRunner, Runner, RunnerData},
    stage::{Stage, StageContainer},
    system::System,
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
            runner: MininalRunner::new_boxed(),
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

    pub fn system<S: Stage, M: System>(&mut self, stage: S, system: M) -> &mut Self {
        let system_name = system.name();
        let stage_name = stage.name();
        self.stages.get(stage).push(system);
        tracing::info!("Attached {} system to {} stage.", system_name, stage_name);
        self
    }

    pub fn stage<S: Stage>(&mut self, stage: S) -> &mut Self {
        let stage_name = stage.name();
        self.stages.insert(stage);
        tracing::info!("Inserted '{}' stage.", stage_name);
        self
    }
}
