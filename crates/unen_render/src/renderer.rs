use crate::state::RendererState;
use unen_app::prelude::{AppState, CommandRegistry, System};

#[derive(Default)]
pub struct Renderer {
    state: Option<RendererState>,
}

pub mod commands {
    use unen_app::prelude::Command;
    use unen_window::prelude::SendableWindowHandle;

    use crate::state::RendererState;

    use super::Renderer;

    pub struct Start {
        pub sendable_window_handle: SendableWindowHandle,
    }

    impl Command for Start {
        type Target = Renderer;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            target.state = Some(RendererState::new(&self.sendable_window_handle));
            tracing::info!("Successfully started Renderer.");
        }
    }

    pub struct Stop;

    impl Command for Stop {
        type Target = Renderer;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            target.state = None;
            tracing::info!("Successfully stopped Renderer.");
        }
    }

    pub struct Resize {
        pub width: u32,
        pub height: u32,
    }

    impl Command for Resize {
        type Target = Renderer;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            if let Some(state) = &mut target.state {
                state.resize(self.width, self.height);
            }
        }
    }

    pub struct Render;

    impl Command for Render {
        type Target = Renderer;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            if let Some(state) = &mut target.state {
                state.render();
            }
        }
    }

    pub struct SetClearColor {
        pub r: f64,
        pub g: f64,
        pub b: f64,
        pub a: f64,
    }

    impl Command for SetClearColor {
        type Target = Renderer;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            if let Some(state) = &mut target.state {
                state.clear_color = wgpu::Color {
                    r: self.r,
                    g: self.g,
                    b: self.b,
                    a: self.a,
                }
            }
        }
    }
}

impl System for Renderer {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        commands.apply_all(self);
        state
    }
}
