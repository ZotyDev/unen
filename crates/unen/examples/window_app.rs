use unen::prelude::*;

fn main() {
    let mut app = create_app();

    app.runner(WinitRunner::default())
        .runner(WinitRunner::default())
        .system(START, TracingLogger)
        .system(STEP, Renderer::default());

    app.run();
}
