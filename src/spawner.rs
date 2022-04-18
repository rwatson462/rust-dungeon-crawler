use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            colour: ColorPair::new(WHITE,BLACK),
            glyph: to_cp437('@')
        }
    ));
}