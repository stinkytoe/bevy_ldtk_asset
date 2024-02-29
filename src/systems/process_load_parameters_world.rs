use bevy::{asset::LoadState, prelude::*};

use crate::{assets::world::WorldAsset, structs::LoadParameters};

pub(crate) fn process_load_parameters_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    worlds: Res<Assets<WorldAsset>>,
    worlds_query: Query<(Entity, &Handle<WorldAsset>, &LoadParameters), With<LoadParameters>>,
) {
    for (entity, handle, load_parameters) in worlds_query.iter() {
        // debug!("A new world bundle has been spawned!");
        match load_parameters {
            LoadParameters::Nothing => (),
            LoadParameters::Everything => {
                if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
                    let Some(world) = worlds.get(handle) else {
                        error!("Couldn't get world from handle?");
                        return;
                    };
                    debug!("Attempting to load world: {}", world.identifier());
                    spawn_world(entity, world, &mut commands);
                } else {
                    return;
                }
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
