use unen::prelude::*;

pub struct WindowApp;

impl System for WindowApp {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        commands.add(commands::SetClearColor {
            r: 0.3,
            g: 0.2,
            b: 0.1,
            a: 1.0,
        });

        state
    }
}

fn main() {
    let mut app = create_app();

    app.runner(WinitRunner::default())
        .runner(WinitRunner::default())
        .system(START, TracingLogger)
        .system(STEP, Renderer::default())
        .system(STEP, WindowApp);

    app.run();
}
