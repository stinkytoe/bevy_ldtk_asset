use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::Mesh2dHandle;
use image::imageops::crop;
use image::imageops::flip_horizontal;
use image::imageops::flip_vertical;
use image::imageops::overlay;
use image::ColorType;
use image::DynamicImage;
use thiserror::Error;

use crate::layer::LayerComponent;
use crate::layer::LayerType;
use crate::layer::LoadTileLayerSettings;
use crate::prelude::ProjectResolver;
use crate::project::ProjectAsset;

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
    #[error("Tileset path not found in project asset!")]
    BadTilesetPath,
    #[error("Tileset handle not found!")]
    BadTilesetHandle,
    // #[error("Fail to convert Bevy image to dynamic image! {0}")]
    // XXIntoDynamicImageError(#[from] IntoDynamicImageError),
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
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

        debug!("TileLayerBundle loaded! {}", layer_component.identifier());

        if let Some(material) = match settings {
            LoadTileLayerSettings::ComponentOnly => None,
            LoadTileLayerSettings::Mesh => match project_asset.value().image_export_mode {
                crate::ldtk::ImageExportMode::None
                | crate::ldtk::ImageExportMode::OneImagePerLevel => {
                    if let Some(tileset_rel_path) = &layer_instance_json.tileset_rel_path {
                        // Need to construct the image
                        let tiles = match layer_component.layer_type() {
                            LayerType::IntGrid | LayerType::Autolayer => {
                                &layer_instance_json.auto_layer_tiles
                            }
                            LayerType::Entities => {
                                return Err(NewTileLayerBundleError::UnexpectedEntityLayer);
                            }
                            LayerType::Tiles => &layer_instance_json.grid_tiles,
                        };

                        let tileset_handle = project_asset
                            .get_tileset_handle(tileset_rel_path)
                            .ok_or(NewTileLayerBundleError::BadTilesetPath)?;

                        let tileset_image = images
                            .get(tileset_handle)
                            .ok_or(NewTileLayerBundleError::BadTilesetHandle)?;

                        // had to use .expect(..) because IntoDynamicImageError isn't re-exported
                        // public
                        let mut tileset = tileset_image
                            .clone()
                            .try_into_dynamic()
                            .expect("couldn't convert bevy image to dynamic image!");

                        let mut dynamic_image = DynamicImage::new(
                            level_json.px_wid as u32,
                            level_json.px_hei as u32,
                            ColorType::Rgba8,
                        );

                        tiles.iter().for_each(|tile| {
                            let mut cropped = crop(
                                &mut tileset,
                                tile.src[0] as u32,
                                tile.src[1] as u32,
                                layer_instance_json.c_wid as u32,
                                layer_instance_json.c_hei as u32,
                            )
                            .to_image();

                            if tile.f & 0x1 == 0x1 {
                                cropped = flip_horizontal(&cropped);
                            }

                            if tile.f & 0x2 == 0x2 {
                                cropped = flip_vertical(&cropped);
                            }

                            overlay(&mut dynamic_image, &cropped, tile.px[0], tile.px[1]);
                        });

                        let color = Color::srgba(1.0, 1.0, 1.0, layer_instance_json.opacity as f32);

                        let new_image =
                            Image::from_dynamic(dynamic_image, true, RenderAssetUsages::default());

                        let texture = Some(images.add(new_image));

                        Some(materials.add(ColorMaterial { color, texture }))
                    } else {
                        None
                    }
                }
                crate::ldtk::ImageExportMode::LayersAndLevels
                | crate::ldtk::ImageExportMode::OneImagePerLayer => {
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

        commands
            .entity(entity)
            .insert(Name::from(layer_component.identifier()));
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
