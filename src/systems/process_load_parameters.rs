use bevy::{asset::LoadState, prelude::*};

use crate::{
    structs::LoadParameters,
    traits::{HasIdentifier, SpawnsEntities},
};

pub(crate) fn process_load_parameters<T: Asset + HasIdentifier + SpawnsEntities>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    worlds: Res<Assets<T>>,
    worlds_query: Query<(Entity, &Handle<T>, &LoadParameters), With<LoadParameters>>,
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
                    debug!("Attempting to load world: {}", *world.identifier());
                    world.spawn_entities(&mut commands, entity);
                } else {
                    return;
                }
            }
        }

        commands.entity(entity).remove::<LoadParameters>();
    }
}
