use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(Point)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate)| {
        let item = ecs.entry_ref(activate.item);
        if let Ok(item) = item {
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                healing_to_apply.push((activate.used_by, healing.amount));
            }
            if let Ok(_map_item) = item.get_component::<ProvidesDungeonMap>() {
                if let Ok(target) = ecs.entry_ref(activate.used_by) {
                    if let Ok(point) = target.get_component::<Point>() {
                        let tiles_to_reveal: Vec<(usize, f32)> = map.revealed_tiles
                            .iter()
                            .enumerate()
                            .map(|(idx, _)| (
                                idx,
                                DistanceAlg::Pythagoras.distance2d(
                                    *point, map.index_to_point2d(idx)
                                )
                            ))
                            .filter(|(_, distance)| *distance < 16.0)
                            .collect();

                        tiles_to_reveal
                            .iter()
                            .for_each(|(idx, _)| map.revealed_tiles[*idx] = true);
                    }
                }
            }
        }

        commands.remove(activate.item);
        commands.remove(*entity);
    });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(
                    health.max,
                    health.current + heal.1
                );
            }
        }
    }
}
