use std::{collections::HashMap, path::Path};

use unen_app::prelude::{AppState, CommandRegistry, System};

use crate::{resource::Resource, resource_loader::ResourceLoader};

pub struct ResourceManager {
    resources: HashMap<String, Box<dyn Resource>>,
    loaders: HashMap<String, Box<dyn ResourceLoader>>,
}

pub mod commands {
    use std::path::PathBuf;

    use unen_app::prelude::Command;

    use crate::{resource_loader::ResourceLoader, resource_manager::ResourceManager};

    pub struct RegisterLoader {
        pub extension: String,
        pub loader: Box<dyn ResourceLoader>,
    }

    impl Command for RegisterLoader {
        type Target = ResourceManager;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            target.register_loader(&self.extension, self.loader);
        }
    }

    pub struct LoadResource {
        pub path: PathBuf,
    }

    impl Command for LoadResource {
        type Target = ResourceManager;

        fn apply(self: Box<Self>, target: &mut Self::Target) {
            target.load_from_path(&self.path);
        }
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        let resources = HashMap::new();
        let loaders = HashMap::new();

        Self { resources, loaders }
    }

    pub fn register_loader(&mut self, extension: &str, loader: Box<dyn ResourceLoader>) {
        self.loaders.insert(extension.to_string(), loader);
    }

    pub fn load_from_path(&mut self, path: &Path) {
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        if let Some(loader) = self.loaders.remove(extension) {
            loader.load(path, self);

            self.loaders.insert(extension.to_string(), loader);
        } else {
            tracing::error!("Loader not found for the file extension: {}", extension);
        }
    }

    pub fn get<T: Resource>(&self, id: &str) -> Option<&T> {
        self.resources.get(id)?.as_any().downcast_ref::<T>()
    }
}

impl System for ResourceManager {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        commands.apply_all(self);
        state
    }
}
