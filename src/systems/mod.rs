mod player_input;
mod map_render;
mod entity_render;
mod collision;
mod move_randomly;
mod end_turn;

use crate::prelude::*;


pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(collision::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(move_randomly::move_randomly_system())
        .add_system(collision::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}