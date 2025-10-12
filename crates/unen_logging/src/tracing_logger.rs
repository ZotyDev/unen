use unen_app::prelude::{AppState, System};

pub struct TracingLogger;

impl System for TracingLogger {
    fn execute(&mut self, state: AppState) -> AppState {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .with_level(true)
            .init();
        tracing::info!("Logger {} initialized.", self.name());

        state
    }
}
