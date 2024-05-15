use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::Mesh2dHandle;
use bevy::{prelude::*, render::mesh::PrimitiveTopology};
use image::imageops::crop;
use image::imageops::flip_horizontal;
use image::imageops::flip_vertical;
use image::imageops::overlay;
use image::ColorType;
use image::DynamicImage;
use thiserror::Error;

use crate::layer::LayerComponent;
use crate::layer::LoadTileLayerMeshSettings;
use crate::layer::LoadTileLayerSettings;
use crate::layer::Tiles;
use crate::ldtk;
use crate::project::ProjectAsset;
use crate::project::ProjectResolver;

#[derive(Debug, Error)]
pub(crate) enum NewTilesError {
    #[error("Bad project handle!")]
    BadProjectHandle,
    #[error("Bad level uid!")]
    BadLevelUid(i64),
    #[error("Missing Layer Component!")]
    MissingLayerComponent,
    #[error("Tileset path not found in project asset!")]
    BadTilesetPath,
    #[error("Tileset handle not found!")]
    BadTilesetHandle,
    // #[error("Fail to convert Bevy image to dynamic image! {0:?}")]
    // IntoDynamicImageError(#[from] IntoDynamicImageError),
}

#[allow(clippy::type_complexity)]
pub(crate) fn new_tiles(
    mut commands: Commands,
    new_tiles_query: Query<
        (
            Entity,
            &Handle<ProjectAsset>,
            &LayerComponent,
            &LoadTileLayerSettings,
            &Tiles,
        ),
        Changed<Tiles>,
    >,
    project_assets: Res<Assets<ProjectAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<(), NewTilesError> {
    for (entity, project_handle, layer_component, settings, tiles) in new_tiles_query.iter() {
        let project_asset = project_assets
            .get(project_handle)
            .ok_or(NewTilesError::BadProjectHandle)?;

        let level_json = project_asset
            .get_level_by_uid(layer_component.level_id())
            .ok_or(NewTilesError::BadLevelUid(layer_component.level_id()))?;

        let layer_instance_json = project_asset
            .get_layer_instance_by_level_layer_iid(&level_json.iid, layer_component.iid())
            .ok_or(NewTilesError::MissingLayerComponent)?;

        if let Some(material) = match settings.mesh_settings {
            LoadTileLayerMeshSettings::ComponentOnly => None,
            LoadTileLayerMeshSettings::Mesh => match project_asset.value().image_export_mode {
                ldtk::ImageExportMode::None | ldtk::ImageExportMode::OneImagePerLevel => {
                    if let Some(tileset_rel_path) = &layer_instance_json.tileset_rel_path {
                        // Need to construct the image
                        let tileset_handle = project_asset
                            .get_tileset_handle(tileset_rel_path)
                            .ok_or(NewTilesError::BadTilesetPath)?;

                        let tileset_image = images
                            .get(tileset_handle)
                            .ok_or(NewTilesError::BadTilesetHandle)?;

                        // Waiting on bevy 0.14
                        // see https://github.com/bevyengine/bevy/pull/13223
                        // let mut tileset = tileset_image.clone().try_into_dynamic()?;
                        let mut tileset = tileset_image
                            .clone()
                            .try_into_dynamic()
                            .expect("try_into_dynamic fail!");

                        let mut dynamic_image = DynamicImage::new(
                            level_json.px_wid as u32,
                            level_json.px_hei as u32,
                            ColorType::Rgba8,
                        );

                        tiles.tiles.iter().for_each(|tile| {
                            trace!("Tile loaded! {tile:?}");
                            let mut cropped = crop(
                                &mut tileset,
                                tile.source().x,
                                tile.source().y,
                                layer_instance_json.c_wid as u32,
                                layer_instance_json.c_hei as u32,
                            )
                            .to_image();

                            if tile.flip_h() {
                                cropped = flip_horizontal(&cropped);
                            }

                            if tile.flip_v() {
                                cropped = flip_vertical(&cropped);
                            }

                            overlay(
                                &mut dynamic_image,
                                &cropped,
                                tile.location().x,
                                tile.location().y,
                            );
                        });

                        let color = Color::rgba(1.0, 1.0, 1.0, layer_instance_json.opacity as f32);

                        let new_image =
                            Image::from_dynamic(dynamic_image, true, RenderAssetUsages::default());

                        let texture = Some(images.add(new_image));

                        Some(materials.add(ColorMaterial { color, texture }))
                    } else {
                        None
                    }
                }
                ldtk::ImageExportMode::LayersAndLevels
                | ldtk::ImageExportMode::OneImagePerLayer => {
                    // can pull the image from assets
                    todo!()
                }
            },
        } {
            let mesh = Mesh2dHandle(meshes.add(create_tile_layer_mesh(UVec2::new(
                level_json.px_wid as u32,
                level_json.px_hei as u32,
            ))));

            commands.entity(entity).insert((mesh, material));
        };
    }
    Ok(())
}

fn create_tile_layer_mesh(size: UVec2) -> Mesh {
    let size = Vec2::new(size.x as f32, size.y as f32);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [size.x, 0.0, 0.0],
            [size.x, -size.y, 0.0],
            [0.0, -size.y, 0.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    )
}
