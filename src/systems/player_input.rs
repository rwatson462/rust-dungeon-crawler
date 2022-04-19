use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Monster)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    if let Some(key) = key {

        let mut players = <(Entity, &Point)>::query()
            .filter(component::<Player>());

        let (player_entity, player_pos) = players
            .iter(ecs)
            .find_map(|(entity,pos)| Some((*entity, *pos)))
            .unwrap();

        let delta = match key {
            VirtualKeyCode::A => Point::new(-1,0),
            VirtualKeyCode::D => Point::new(1,0),
            VirtualKeyCode::W => Point::new(0,-1),
            VirtualKeyCode::S => Point::new(0,1),
            VirtualKeyCode::H => {
                if let Ok(mut health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
                    health.current = i32::min(health.max, health.current+1);
                }
                Point::zero()
            },
            _ => Point::zero()
        };

        let destination = player_pos + delta;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            <(Entity,&Point)>::query()
                .filter(component::<Monster>())
                .iter(ecs)
                .filter(|(_,pos)| **pos == destination)
                .for_each(|(entity,_)| {
                    hit_something = true;
                    commands.push(((), WantsToAttack {
                        attacker: player_entity,
                        victim: *entity
                    }));
                });

            if !hit_something {
                players.iter_mut(ecs).for_each(|(entity,pos)| {
                    let destination = *pos + delta;
                    commands.push(((), WantsToMove{ entity: *entity, destination}));
                });
            }
        
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
