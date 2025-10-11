mod app;

pub mod prelude {
    pub use crate::app::{create_app, RunningApp, StoppedApp};
}
