use bracket_lib::prelude::*;

mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;
struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let starting_point = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        State {
            map: Map::new(),
            player: Player::new(starting_point),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("DinoBane 4: Barbarian Carnage")
        .with_fps_cap(60.0)
        .build()?;
    main_loop(context, State::new())
}
