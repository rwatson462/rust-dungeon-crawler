mod map;
mod map_builder;
mod player;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::camera::*;
}

use prelude::*;


struct State {
    map: Map,
    player: Player,
    camera: Camera
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        }
    }

    fn poll_for_user_input(&self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            };
        }
    }

    fn clear_canvas(&self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
    }

    fn update(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &self.map, &mut self.camera);
    }

    fn render(&mut self, ctx: &mut BTerm) {
        self.clear_canvas(ctx);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.poll_for_user_input(ctx);
        self.update(ctx);
        self.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("The Rusty Dungeon")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32,32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
