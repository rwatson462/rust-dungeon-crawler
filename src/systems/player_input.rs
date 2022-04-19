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
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1,0),
            VirtualKeyCode::D => Point::new(1,0),
            VirtualKeyCode::W => Point::new(0,-1),
            VirtualKeyCode::S => Point::new(0,1),
            _ => Point::zero()
        };

        let mut players = <(Entity, &Point)>::query()
            .filter(component::<Player>());

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity,pos)| Some((*entity, *pos+delta)))
            .unwrap();

        let mut did_something = false;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            <(Entity,&Point)>::query()
                .filter(component::<Monster>())
                .iter(ecs)
                .filter(|(_,pos)| **pos == destination)
                .for_each(|(entity,_)| {
                    hit_something = true;
                    did_something = true;
                    commands.push(((), WantsToAttack {
                        attacker: player_entity,
                        victim: *entity
                    }));
                });

            if !hit_something {
                players.iter_mut(ecs).for_each(|(entity,pos)| {
                    let destination = *pos + delta;
                    commands.push(((), WantsToMove{ entity: *entity, destination}));
                    did_something = true;
                });
            }
        
            *turn_state = TurnState::PlayerTurn;
        }

        if !did_something {
            if let Ok(mut health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current+1);
            }
        }
    }
}