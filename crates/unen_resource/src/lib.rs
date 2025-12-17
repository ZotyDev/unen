mod resource;
mod resource_loader;
mod resource_manager;

pub mod prelude {
    pub use crate::resource::Resource;
    pub use crate::resource_loader::ResourceLoader;
    pub use crate::resource_manager::ResourceManager;
    pub mod resource_commands {
        pub use crate::resource_manager::commands::*;
    }
}
