use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::project::ProjectAsset;
use crate::world::WorldAsset;
use crate::world::WorldBundleLoadSettings;

pub(crate) fn respond_to_new_world_bundle(
    mut commands: Commands,
    new_level_query: Query<(Entity, &Handle<WorldAsset>, &WorldBundleLoadSettings)>,
    asset_server: Res<AssetServer>,
    world_assets: Res<Assets<WorldAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
) {
    new_level_query
        .iter()
        .for_each(|(entity, id, _load_settings)| {
            if let Some(LoadState::Loaded) = asset_server.get_load_state(id) {
                debug!("WorldAsset loaded!");
                let _world_asset = world_assets
                    .get(id)
                    .expect("Failed to load world asset after receiving LoadState::Loaded?");
                commands
                    .entity(entity)
                    .insert(Name::from(""))
                    .remove::<WorldBundleLoadSettings>();
            }
        });
}
