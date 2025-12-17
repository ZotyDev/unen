use unen_app::prelude::{AppState, CommandRegistry, System};

use crate::state::GpuState;

pub struct GpuContext {
    state: Option<GpuState>,
}

pub mod commands {
    use std::sync::Arc;

    use unen_app::prelude::Command;

    use crate::{context::GpuContext, state::GpuState};

    pub struct Start {
        pub instace: Arc<wgpu::Instance>,
        pub surface: Arc<wgpu::Surface<'static>>,
    }

    impl Command for Start {
        type Target = GpuContext;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            let state = pollster::block_on(async {
                GpuState::from_surface_and_instance(&self.surface, &self.instace).await
            });

            target.state = Some(state);
        }
    }
}

impl System for GpuContext {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        commands.apply_all(self);
        state
    }
}
