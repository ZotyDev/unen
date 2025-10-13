use unen::prelude::*;

fn main() {
    let mut app = create_app();

    app.runner(WinitRunner::default())
        .system(START, TracingLogger);

    app.run();
}
