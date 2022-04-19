use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query()
        .filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut batch_draw = DrawBatch::new();
    batch_draw.target(2);
    batch_draw.print_centered(1, "Explore the dungeon. [W/A/S/D] to move");
    batch_draw.bar_horizontal(
        Point::new(0,3),
        20,
        player_health.current,
        player_health.max,
        ColorPair::new(RED,BLACK)
    );

    batch_draw.print_color(
        Point::new(1,2),
        format!(" Health: {} / {} ", player_health.current, player_health.max),
        ColorPair::new(WHITE,GREEN)
    );

    // 10000 gives ample room for other sprites to draw first
    batch_draw.submit(10000).expect("Batch error");
}