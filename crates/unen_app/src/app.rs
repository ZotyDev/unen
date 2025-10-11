pub fn create_app() -> StoppedApp {
    StoppedApp::default()
}

#[derive(Debug, Default)]
pub struct AppState {}

#[derive(Debug, Default)]
pub struct StoppedApp {
    state: AppState,
}

impl StoppedApp {
    /// Starts the application, consumes `self` and returns a [`RunningApp`].
    pub fn run(self) -> RunningApp {
        RunningApp::default()
    }
}

#[derive(Debug, Default)]
pub struct RunningApp {}

impl RunningApp {}
