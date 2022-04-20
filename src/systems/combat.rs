use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat( ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity,Entity,Entity)> = attackers
        .iter(ecs)
        .map(|(message,attack)| (*message, attack.victim, attack.attacker))
        .collect();

    victims.iter().for_each(|(message,victim,attacker)| {
        let is_player = ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok();

        if let Ok(mut victim_health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            victim_health.current -= 1;
            if victim_health.current < 1 {

                // only remove the entity from the game if it's not the player (as a Player is required)
                if !is_player {
                    commands.remove(*victim);
                }

                let health_to_heal = (victim_health.max / 2) as i32;

                if let Ok(attacker_health) = ecs.entry_mut(*attacker).unwrap().get_component_mut::<Health>() {
                    attacker_health.current = std::cmp::min(
                        attacker_health.current + health_to_heal,
                        attacker_health.max
                    );
                }
            }
        }
        commands.remove(*message);
    })
}