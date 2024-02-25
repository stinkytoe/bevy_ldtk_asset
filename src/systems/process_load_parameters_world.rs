use bevy::prelude::*;

use crate::{assets::world::WorldAsset, structs::LoadParameters};

pub(crate) fn process_load_parameters_world(
    mut commands: Commands,
    worlds: Res<Assets<WorldAsset>>,
    worlds_query: Query<(Entity, &Handle<WorldAsset>, &LoadParameters), Added<LoadParameters>>,
) {
    for (entity, handle, load_parameters) in worlds_query.iter() {
        debug!("A new world bundle has been spawned!");
        match load_parameters {
            LoadParameters::Nothing => (),
            LoadParameters::Everything => {
                let Some(world) = worlds.get(handle) else {
                    debug!("Couldn't get world from handle?");
                    return;
                };
                debug!("Attempting to load world: {}", world.identifier());
                spawn_world(entity, world, &mut commands);
            }
        }

        commands.entity(entity).remove::<LoadParameters>();
    }
}

fn spawn_world(entity: Entity, world: &WorldAsset, commands: &mut Commands) {
    commands
        .entity(entity)
        .insert((Name::from(world.identifier().as_str()),));
}
