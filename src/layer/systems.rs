use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use thiserror::Error;

use crate::project::ProjectAsset;

use crate::layer::LayerAsset;
use crate::layer::Tiles;

use super::util::build_image_from_tiles;
use super::util::create_tile_layer_mesh;
use super::util::BuildImageFromTilesError;

#[derive(Debug, Error)]
pub(crate) enum HandleLayerTilesError {
    #[error(transparent)]
    BuildImageFromTilesError(#[from] BuildImageFromTilesError),
    #[error("Bad entity asset handle!")]
    BadEntityAssetHandle,
    #[error("Bad project asset handle!")]
    BadProjectAssetHandle,
    #[error("Bad tileset path?")]
    BadTilesetPath,
    #[error("Bad tileset handle?")]
    BadTilesetHandle,
}

#[allow(clippy::type_complexity)]
pub(crate) fn handle_layer_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &Handle<LayerAsset>, Option<&Tiles>), Changed<Tiles>>,
    layer_assets: Res<Assets<LayerAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), HandleLayerTilesError> {
    for (entity, layer_handle, tiles) in query.iter_mut() {
        let layer_asset = layer_assets
            .get(layer_handle)
            .ok_or(HandleLayerTilesError::BadEntityAssetHandle)?;

        let project_asset = project_assets
            .get(&layer_asset.project)
            .ok_or(HandleLayerTilesError::BadProjectAssetHandle)?;

        let mut delete_stub = || {
            commands.entity(entity).remove::<Mesh2dHandle>();
            commands.entity(entity).remove::<Handle<ColorMaterial>>();
        };

        let Some(tileset_rel_path) = layer_asset.tileset_rel_path.as_ref() else {
            debug!("no tileset_rel_path");
            delete_stub();
            return Ok(());
        };

        debug!("tileset_rel_path: {tileset_rel_path:?}");

        let tileset_handle = project_asset
            .tileset_assets
            .get(tileset_rel_path)
            .ok_or(HandleLayerTilesError::BadTilesetPath)?;

        let tileset = images
            .get(tileset_handle)
            .ok_or(HandleLayerTilesError::BadTilesetHandle)?;

        match tiles {
            Some(tiles) if tiles.tiles.is_empty() => {
                delete_stub();
            }
            None => {
                delete_stub();
            }
            Some(tiles) => {
                debug!("making a canvas!");
                let canvas_size = layer_asset.grid_size * layer_asset.grid_cell_size;

                let mesh = create_tile_layer_mesh(canvas_size.as_vec2());
                let mesh = Mesh2dHandle(meshes.add(mesh));

                let image = build_image_from_tiles(
                    tileset,
                    canvas_size.as_uvec2(),
                    UVec2::splat(layer_asset.grid_cell_size as u32),
                    tiles,
                )?;

                let color = Color::rgba(01.0, 1.0, 1.0, layer_asset.opacity as f32);

                let texture_handle = images.add(image);

                let texture = Some(texture_handle);

                let material = materials.add(ColorMaterial { color, texture });

                commands.entity(entity).insert((mesh, material));
            }
        };
    }

    Ok(())
}
