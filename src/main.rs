use bracket_lib::prelude::*;

struct State {}

impl State {
    fn new() -> Self {
        State {}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {}
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rust Dungeon Crawler")
        .build()?;

    main_loop(context, State::new())
}
