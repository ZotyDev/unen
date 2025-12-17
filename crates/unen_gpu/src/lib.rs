mod context;
mod state;

pub mod prelude {
    pub use crate::context::GpuContext;
    pub use crate::state::GpuState;
    pub mod gpu_commands {
        pub use crate::context::commands::*;
    }
}
