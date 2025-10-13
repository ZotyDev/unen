use unen::prelude::*;

#[derive(Default)]
pub struct WindowApp {
    hue: f64,
}

impl System for WindowApp {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        self.hue = (self.hue + 0.005) % 360.0;

        let (r, g, b) = hsv_to_rgb(self.hue, 1.0, 1.0);
        commands.add(commands::SetClearColor { r, g, b, a: 1.0 });

        state
    }
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r1 + m, g1 + m, b1 + m)
}

fn main() {
    let mut app = create_app();

    app.runner(WinitRunner::default())
        .runner(WinitRunner::default())
        .system(START, TracingLogger)
        .system(STEP, Renderer::default())
        .system(STEP, WindowApp::default());

    app.run();
}
