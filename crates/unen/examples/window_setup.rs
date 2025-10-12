use unen::prelude::*;
use unen_render::prelude::RendererEventHandler;
use unen_winit::prelude::WinitRunner;

fn main() {
    let _ = create_engine()
        .set_runner(WinitRunner::default())
        .add_event_handler(RendererEventHandler::default())
        .start()
        .stop();
}
