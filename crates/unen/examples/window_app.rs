use unen::prelude::TracingLogger;
use unen_app::prelude::*;
use unen_winit::prelude::WinitRunner;

fn main() {
    let mut app = create_app();

    app.runner(WinitRunner::default())
        .system(START, TracingLogger);

    app.run();
}
