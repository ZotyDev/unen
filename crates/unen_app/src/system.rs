use crate::{app::AppState, command::CommandRegistry};

pub trait System: Send + Sync + 'static {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState;
}

#[derive(Default)]
pub struct SystemContainer {
    container: Vec<Box<dyn System>>,
}

impl SystemContainer {
    pub fn push<S: System>(&mut self, system: S) {
        self.container.push(Box::new(system));
    }

    pub fn execute_all(&mut self, mut state: AppState, commands: &mut CommandRegistry) -> AppState {
        for system in &mut self.container {
            state = system.as_mut().execute(state, commands);
        }
        state
    }
}
