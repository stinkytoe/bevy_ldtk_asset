use bevy::asset::LoadState;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::layer::EntityLayerBundle;
use crate::layer::LayerComponent;
use crate::layer::LayerComponentError;
use crate::layer::LayerType;
use crate::layer::TileLayerBundle;
use crate::level::LevelAsset;
use crate::level::LevelBackgroundPosition;
use crate::level::LevelBundleLoadSettings;
use crate::level::LevelComponent;
use crate::level::LevelComponentError;
use crate::level::LoadLayers;
use crate::project::ProjectAsset;

#[derive(Component, Debug)]
pub(crate) struct LevelBundleLoadStub;

#[derive(Debug, Error)]
pub enum NewLevelBundleError {
    #[error("Failed to load level asset after receiving LoadState::Loaded?")]
    LevelAssetLoadFail,
    #[error("Project asset not loaded before world asset?")]
    ProjectAssetLoadFail,
    #[error("IID not found in project! {0}")]
    IidNotFound(String),
    #[error("LevelComponentError: {0}")]
    LevelComponentError(#[from] LevelComponentError),
    #[error("LayerComponentError: {0}")]
    LayerComponentError(#[from] LayerComponentError),
    #[error("bg_rel_path cannot resolve to a string?")]
    BadBgRelPath,
    #[error("bg_rel_path not found in project's background handles?")]
    MissingBgRelPath,
    #[error("bg_pos is None when there should be a background texture?")]
    MissingBgPos,
    #[error("Image handle not valid!")]
    BadImageHandle,
    // #[error("Bad level handle in project, or bad level iid!")]
    // BadLevelIid,
}

pub(crate) fn new_level_bundle(
    mut commands: Commands,
    new_world_query: Query<Entity, Added<LevelBundleLoadSettings>>,
) {
    for entity in &new_world_query {
        commands.entity(entity).insert(LevelBundleLoadStub);
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn level_bundle_loaded(
    mut commands: Commands,
    mut new_level_query: Query<
        (
            Entity,
            &Handle<LevelAsset>,
            &mut Transform,
            &LevelBundleLoadSettings,
        ),
        With<LevelBundleLoadStub>,
    >,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result<(), NewLevelBundleError> {
    for (entity, level_handle, mut transform, load_settings) in new_level_query.iter_mut() {
        let Some(LoadState::Loaded) = asset_server.get_load_state(level_handle) else {
            return Ok(());
        };

        let level_asset = level_assets
            .get(level_handle)
            .ok_or(NewLevelBundleError::LevelAssetLoadFail)?;

        // This is probably paranoia
        let Some(LoadState::Loaded) = asset_server.get_load_state(&level_asset.project_handle)
        else {
            return Ok(());
        };

        let project_asset = project_assets
            .get(level_asset.project_handle.clone())
            .ok_or(NewLevelBundleError::ProjectAssetLoadFail)?;

        debug!("LevelAsset loaded! {:?}", level_handle.path());

        let level_json = project_asset
            .get_level_by_iid(&level_asset.iid)
            .ok_or(NewLevelBundleError::IidNotFound(level_asset.iid.clone()))?;

        let level_component: LevelComponent = level_json.try_into()?;

        let mut entity_commands = commands.entity(entity);

        if load_settings.load_bg_color {
            let mesh: Mesh2dHandle =
                Mesh2dHandle(meshes.add(create_bg_color_mesh(level_component.size())));

            let color = level_component.bg_color();

            let material = materials.add(ColorMaterial { color, ..default() });

            entity_commands.with_children(|parent| {
                parent.spawn((
                    Name::from("bg_color"),
                    MaterialMesh2dBundle {
                        mesh,
                        material,
                        ..default()
                    },
                ));
            });
        }

        if load_settings.load_bg_image {
            if let Some(bg_rel_path) = level_component.bg_rel_path() {
                let color = level_component.bg_color();

                let (image, texture) = {
                    let texture = project_asset
                        .get_background_handle(
                            bg_rel_path
                                .to_str()
                                .ok_or(NewLevelBundleError::BadBgRelPath)?,
                        )
                        .ok_or(NewLevelBundleError::MissingBgRelPath)?
                        .clone();

                    let image = images
                        .get(texture.clone())
                        .ok_or(NewLevelBundleError::BadImageHandle)?;

                    (image, Some(texture))
                };

                let bg_pos = level_component
                    .bg_pos()
                    .ok_or(NewLevelBundleError::MissingBgPos)?;

                let mesh: Mesh2dHandle =
                    Mesh2dHandle(meshes.add(create_bg_image_mesh(image.size(), bg_pos)));

                let material = materials.add(ColorMaterial { color, texture });

                entity_commands.with_children(|parent| {
                    parent.spawn((
                        Name::from("bg_image"),
                        MaterialMesh2dBundle {
                            mesh,
                            material,
                            transform: Transform::from_xyz(
                                0.0,
                                0.0,
                                load_settings.layer_separation,
                            ),
                            ..default()
                        },
                    ));
                });
            }
        }

        let spawn_tile_layer = move |entity_commands: &mut EntityCommands, layer, index| {
            entity_commands.with_children(|parent| {
                parent.spawn(TileLayerBundle {
                    project: level_asset.project_handle.clone(),
                    layer,
                    settings: load_settings.load_tile_layer_settings.clone(),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, index)),
                });
            });
        };

        let spawn_entity_layer = move |entity_commands: &mut EntityCommands, layer, index| {
            entity_commands.with_children(|parent| {
                parent.spawn(EntityLayerBundle {
                    project: level_asset.project_handle.clone(),
                    layer,
                    settings: load_settings.load_entity_layer_settings.clone(),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, index)),
                });
            });
        };

        if let Some(layer_instances) = level_json.layer_instances.as_ref() {
            for (index, layer_json) in layer_instances.iter().rev().enumerate() {
                let index = load_settings.layer_separation * (index + 2) as f32;
                let layer: LayerComponent = layer_json.try_into()?;

                match (&load_settings.load_layers, layer.layer_type()) {
                    // Tile layer variants
                    (
                        LoadLayers::ByIdentifiers(ids),
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) if ids.contains(&layer_json.identifier) => {
                        spawn_tile_layer(&mut entity_commands, layer, index);
                    }
                    (
                        LoadLayers::ByIids(ids),
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) if ids.contains(&layer_json.identifier) => {
                        spawn_tile_layer(&mut entity_commands, layer, index);
                    }
                    (
                        LoadLayers::TileLayers | LoadLayers::All,
                        LayerType::Tiles | LayerType::IntGrid | LayerType::Autolayer,
                    ) => {
                        spawn_tile_layer(&mut entity_commands, layer, index);
                    }

                    // Entity layer variants
                    (LoadLayers::ByIdentifiers(ids), LayerType::Entities)
                        if ids.contains(&layer_json.identifier) =>
                    {
                        spawn_entity_layer(&mut entity_commands, layer, index);
                    }
                    (LoadLayers::ByIids(ids), LayerType::Entities)
                        if ids.contains(&layer_json.identifier) =>
                    {
                        spawn_entity_layer(&mut entity_commands, layer, index);
                    }
                    (LoadLayers::EntityLayers | LoadLayers::All, LayerType::Entities) => {
                        spawn_entity_layer(&mut entity_commands, layer, index);
                    }

                    // ignore me!
                    _ => (),
                };
            }
        }

        transform.translation =
            level_component.world_location() * Vec3::new(1.0, 1.0, load_settings.level_separation);

        entity_commands
            .insert((Name::from(level_component.identifier()), level_component))
            .remove::<LevelBundleLoadStub>();
    }

    Ok(())
}

fn create_bg_color_mesh(size: Vec2) -> Mesh {
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
}

fn create_bg_image_mesh(image_size: UVec2, bg_pos: &LevelBackgroundPosition) -> Mesh {
    let crop_location = bg_pos.crop_location();
    let crop_size = bg_pos.crop_size();
    let scale = bg_pos.scale();
    let top_left = bg_pos.top_left();

    let image_size = Vec2::new(image_size.x as f32, image_size.y as f32);

    let uv_top_left = crop_location / image_size;
    let uv_bottom_right = (crop_location + crop_size) / image_size;

    let bottom_right = top_left + (image_size * scale);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [top_left.x, -top_left.y, 0.0],
            [top_left.x, -bottom_right.y, 0.0],
            [bottom_right.x, -bottom_right.y, 0.0],
            [bottom_right.x, -top_left.y, 0.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [uv_top_left.x, uv_top_left.y],
            [uv_top_left.x, uv_bottom_right.y],
            [uv_bottom_right.x, uv_bottom_right.y],
            [uv_bottom_right.x, uv_top_left.y],
        ],
    )
}
