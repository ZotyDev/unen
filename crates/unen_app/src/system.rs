use crate::app::AppState;

pub trait System: Send + Sync + 'static {
    fn execute(&mut self, state: AppState) -> AppState;
}

#[derive(Default)]
pub struct SystemContainer {
    container: Vec<Box<dyn System>>,
}

impl SystemContainer {
    pub fn push<S: System>(&mut self, system: S) {
        self.container.push(Box::new(system));
    }

    pub fn execute_all(&mut self, mut state: AppState) -> AppState {
        for system in &mut self.container {
            state = system.as_mut().execute(state);
        }
        state
    }
}
