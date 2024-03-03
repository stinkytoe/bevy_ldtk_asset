use std::any::type_name_of_val;

use bevy::{asset::LoadState, prelude::*};

use crate::{
    prelude::{LevelAsset, ProjectAsset, WorldAsset},
    structs::SpawnEntities,
    traits::{HasIdentifier, SpawnsEntities},
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_load_parameters<T: Asset + HasIdentifier + SpawnsEntities>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entity_spawner_asset: Res<Assets<T>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    projects: Res<Assets<ProjectAsset>>,
    worlds: Res<Assets<WorldAsset>>,
    levels: Res<Assets<LevelAsset>>,
    worlds_query: Query<(Entity, &Handle<T>, &SpawnEntities), With<SpawnEntities>>,
) {
    for (entity, handle, load_parameters) in worlds_query.iter() {
        match load_parameters {
            SpawnEntities::Nothing => (),
            SpawnEntities::Everything => {
                if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
                    let Some(world) = entity_spawner_asset.get(handle) else {
                        error!("Couldn't get world from handle?");
                        return;
                    };
                    debug!(
                        "Loading entities for LDtk object {}: {}",
                        type_name_of_val(world),
                        *world.identifier()
                    );
                    world.spawn_entities(
                        &mut commands,
                        entity,
                        &asset_server,
                        &mut meshes,
                        &mut materials,
                        &mut images,
                        &projects,
                        &worlds,
                        &levels,
                    )
                } else {
                    return;
                }
            }
        }

        commands.entity(entity).remove::<SpawnEntities>();
    }
}
