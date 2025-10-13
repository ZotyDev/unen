use std::sync::Arc;

use winit::window::Window;

pub struct WinitState {
    window: Arc<Window>,
}

impl WinitState {
    pub async fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}
