use std::path::Path;

use crate::resource_manager::ResourceManager;

pub trait ResourceLoader: Send + Sync + 'static {
    fn load(&self, path: &Path, target: &mut ResourceManager);
}
