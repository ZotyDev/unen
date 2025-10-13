mod event;
mod handler;
mod manager;

/// The prelude.
pub mod prelude {
    pub use crate::event::{Event, EventBox};
    pub use crate::handler::EventHandler;
    pub use crate::{manager::EventEmitter, manager::EventManager};
    #[cfg(feature = "derive")]
    pub use unen_event_derive::Event;
}
