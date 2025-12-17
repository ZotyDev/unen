use unen::prelude::*;

pub struct BasicShard {}

impl System for BasicShard {
    fn execute(&mut self, state: AppState, commands: &mut CommandRegistry) -> AppState {
        commands.add(shard_commands::Start);

        state
    }
}

fn main() {
    let mut app = create_app();

    app.system(START, TracingLogger)
        .system(STEP, ShardManager::default());

    app.run();
}
