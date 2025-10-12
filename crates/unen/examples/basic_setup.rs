use unen::prelude::*;
use unen_runner::prelude::MininalRunner;

fn main() {
    let _ = create_engine()
        .set_runner(MininalRunner::default())
        .start()
        .stop();
}
