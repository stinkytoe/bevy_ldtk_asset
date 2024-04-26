use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::layer::LayerComponent;
use crate::layer::LayerType;
use crate::layer::LoadTileLayerSettings;
use crate::project::ProjectAsset;
use crate::project::ProjectResolver;

#[derive(Debug, Error)]
pub(crate) enum NewTileLayerBundleError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad level uid!")]
    BadLevelUid(i64),
    #[error("Missing Layer Component!")]
    MissingLayerComponent,
    #[error("Entity layer in TileLayerBundle?")]
    UnexpectedEntityLayer,
}

pub(crate) fn new_tile_layer_bundle(
    mut commands: Commands,
    new_tile_layer_query: Query<
        (
            Entity,
            &Handle<ProjectAsset>,
            &LayerComponent,
            &LoadTileLayerSettings,
        ),
        Added<LoadTileLayerSettings>,
    >,
    project_assets: Res<Assets<ProjectAsset>>,
) -> Result<(), NewTileLayerBundleError> {
    for (entity, project_handle, layer_component, settings) in new_tile_layer_query.iter() {
        let project_asset = project_assets
            .get(project_handle)
            .ok_or(NewTileLayerBundleError::BadProjectHandle)?;

        let level_json = project_asset
            .get_level_by_uid(layer_component.level_id())
            .ok_or(NewTileLayerBundleError::BadLevelUid(
                layer_component.level_id(),
            ))?;

        let layer_instance_json = project_asset
            .get_layer_instance_by_level_layer_iid(&level_json.iid, layer_component.iid())
            .ok_or(NewTileLayerBundleError::MissingLayerComponent)?;

        let tiles = match layer_component.layer_type() {
            LayerType::IntGrid | LayerType::Autolayer => &layer_instance_json.auto_layer_tiles,
            LayerType::Entities => return Err(NewTileLayerBundleError::MissingLayerComponent),
            LayerType::Tiles => &layer_instance_json.grid_tiles,
        };

        // match settings {
        //     LoadTileLayerSettings::ComponentOnly => todo!(),
        //     LoadTileLayerSettings::Mesh => todo!(),
        // };

        commands
            .entity(entity)
            .insert(Name::from(layer_component.identifier()));
    }

    Ok(())
}
