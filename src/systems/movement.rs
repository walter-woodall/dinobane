use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    movement_intent: &MovementIntent,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(movement_intent.destination) {
        commands.add_component(movement_intent.entity, movement_intent.destination);

        if ecs.entry_ref(movement_intent.entity).unwrap().get_component::<Player>().is_ok() {
            camera.on_player_move(movement_intent.destination);
        }
    }

    commands.remove(*entity);
}
