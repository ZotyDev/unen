use unen::prelude::*;

fn main() {
    let mut app = create_app();

    app.system(START, TracingLogger);

    app.run();
}
