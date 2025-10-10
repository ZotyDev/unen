use unen_event::prelude::{EventBox, EventHandler};
use unen_window::prelude::WindowEvent;

use crate::state::RendererState;

#[derive(Default)]
pub struct RendererEventHandler {
    state: Option<RendererState>,
}

impl EventHandler for RendererEventHandler {
    fn handle(&mut self, event: &EventBox) -> bool {
        if let Some(window_event) = event.downcast_ref::<WindowEvent>() {
            match window_event {
                WindowEvent::Created(handle) => {
                    tracing::info!("Window created, initializing renderer...");
                    self.state = Some(RendererState::new(handle));
                }
                WindowEvent::Resized { width, height } => {
                    if let Some(state) = &mut self.state {
                        state.resize(*width, *height);
                    }
                }
                WindowEvent::Redraw => {
                    if let Some(state) = &mut self.state {
                        state.render();
                    }
                }
                WindowEvent::Destroyed => {
                    tracing::info!("Window destroyed, cleaning up renderer...");
                    self.state = None;
                }
            }
        }

        false
    }
}
