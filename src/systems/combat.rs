use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let defenders: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.defender))
        .collect();

    defenders.iter().for_each(|(message, defender)| {
        if let Ok(mut health) = ecs
            .entry_mut(*defender)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*defender);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}