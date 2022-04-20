mod player_input;
mod map_render;
mod entity_render;
mod move_randomly;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod chasing;

use crate::prelude::*;


pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(move_randomly::move_randomly_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
