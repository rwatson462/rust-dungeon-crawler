use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            colour: ColorPair::new(WHITE,BLACK),
            glyph: to_cp437('@')
        },
        Health {
            current: 5,
            max: 10
        },
        Name("This is you! : ".to_string())
    ));
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {

    match rng.roll_dice(1,10) {
        1..=4 => spawn_goblin(ecs, pos),
        5..=7 => spawn_skeleton(ecs, pos),
        _ => spawn_orc(ecs, pos)
    };
}

fn spawn_goblin(ecs: &mut World, pos: Point) {
    ecs.push((
        Monster,
        SmarterMonster,
        pos,
        Render {
            colour: ColorPair::new(WHITE,BLACK),
            glyph: to_cp437('g')
        },
        Health { current: 1, max: 1 },
        Name("Goblin".to_string())
    ));
}

fn spawn_orc(ecs: &mut World, pos: Point) {
    ecs.push((
        Monster,
        SmarterMonster,
        pos,
        Render {
            colour: ColorPair::new(WHITE,BLACK),
            glyph: to_cp437('o')
        },
        Health { current: 3, max: 3 },
        Name("Orc".to_string())
    ));
}

fn spawn_skeleton(ecs: &mut World, pos: Point) {
    ecs.push((
        Monster,
        MovingRandomly,
        pos,
        Render {
            colour: ColorPair::new(WHITE,BLACK),
            glyph: to_cp437('g')
        },
        Health { current: 1, max: 1 },
        Name("Skeleton".to_string())
    ));
}