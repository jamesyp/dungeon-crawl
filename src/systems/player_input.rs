use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
) {
    let (player, player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .map(|(entity, pos)| (*entity, *pos))
        .next()
        .unwrap();

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::E => {
                get_item(player, player_pos, ecs, commands);

                Point::zero()
            },
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };
        let destination  = player_pos + delta;

        if delta.x != 0 || delta.y != 0 {
            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            let mut hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, pos)| { **pos == destination })
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push(((), WantsToAttack{
                            attacker: player,
                            defender: *entity,
                        }));
                });

            if !hit_something {
                commands.push(((), WantsToMove {
                        entity: player,
                        destination,
                    }));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn get_item(player: Entity, pos: Point, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut items = <(Entity, &Item, &Point)>::query();
    items.iter(ecs)
        .filter(|(_entity, _item, &item_pos)| item_pos == pos)
        .for_each(|(entity, _item, _item_pos)| {
            commands.remove_component::<Point>(*entity);
            commands.add_component(*entity, Carried(player));
        });
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _player)| *entity)
        .next()
        .unwrap();

    let item = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .enumerate()
        .filter(|(item_index, (_, _, _))| *item_index == n)
        .map(|(_, (item, _, _))| *item)
        .next();

    if let Some(item) = item {
        commands.push(((), ActivateItem{
            used_by: player,
            item
        }));
    }

    Point::zero()
}
