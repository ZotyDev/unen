use unen::prelude::TracingLogger;
use unen_app::prelude::*;

fn main() {
    let mut app = create_app();

    app.system(START, TracingLogger);

    app.run();
}
