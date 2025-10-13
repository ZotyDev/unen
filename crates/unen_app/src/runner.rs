use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use signal_hook::{
    consts::{SIGINT, SIGTERM},
    flag,
};

use crate::{
    app::AppState,
    command::CommandRegistry,
    stage::{StageContainer, START, STEP, STOP},
};

pub trait Runner: Send + Sync + 'static {
    fn run(&mut self, data: RunnerData) -> RunnerData;
}

pub struct RunnerData {
    pub stages: StageContainer,
    pub state: AppState,
    pub term: Arc<AtomicBool>,
    pub commands: CommandRegistry,
}

#[derive(Default)]
pub struct MininalRunner;

impl MininalRunner {
    pub fn new_boxed() -> Box<dyn Runner> {
        Box::new(Self {})
    }
}

impl Runner for MininalRunner {
    fn run(&mut self, data: RunnerData) -> RunnerData {
        let RunnerData {
            mut stages,
            mut state,
            term,
            mut commands,
        } = data;

        state = stages.get(START).execute_all(state, &mut commands);

        flag::register(SIGINT, Arc::clone(&term)).expect("Failed to register SIGINT flag.");
        flag::register(SIGTERM, Arc::clone(&term)).expect("Failed to register SIGTERM flag.");

        while !term.load(Ordering::Relaxed) {
            state = stages.get(STEP).execute_all(state, &mut commands);

            thread::sleep(Duration::from_millis(1));
        }

        state = stages.get(STOP).execute_all(state, &mut commands);

        RunnerData {
            stages,
            state,
            term,
            commands,
        }
    }
}
