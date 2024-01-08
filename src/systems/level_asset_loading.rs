use crate::{
    ldtk::{
        entity_instance::EntityInstance, layer_instance::LayerInstance, level_asset::LevelAsset,
        level_component::LevelComponent, project::Project,
    },
    ldtk_json::{self},
    resources::LdtkLevels,
};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{Anchor, MaterialMesh2dBundle},
};

pub fn process_level_loading(
    mut levels: ResMut<LdtkLevels>,
    mut ev_asset: EventReader<AssetEvent<LevelAsset>>,
    level_query: Query<(Entity, &Handle<LevelAsset>), With<LevelComponent>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::<LevelAsset>::LoadedWithDependencies { id } = ev {
            if let Some((entity, handle)) = level_query
                .iter()
                .find(|(_entity, handle)| handle.id() == *id)
            {
                trace!("Found a matching ldtk level label and entity! {entity:?} {handle:?}");
                levels.to_load.insert((entity, handle.clone()));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn levels_changed(
    mut commands: Commands,
    mut levels: ResMut<LdtkLevels>,
    mut asset_server: ResMut<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    project_assets: Res<Assets<Project>>,
    mut query: Query<&mut Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    if !levels.to_load.is_empty() {
        let to_load: Vec<_> = levels.to_load.drain().collect();
        to_load.iter().for_each(|x| {
            let ldtk_level = level_assets.get(&x.1).expect("level handle is None?");

            if ldtk_level.is_loaded(&asset_server) {
                levels.loaded.insert(x.clone());
                finish_level_asset_loading(
                    x.clone(),
                    &mut commands,
                    &mut asset_server,
                    &level_assets,
                    &project_assets,
                    &mut query,
                    &mut meshes,
                    &mut materials,
                    &images,
                );
            } else {
                levels.to_load.insert(x.clone());
            };
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn finish_level_asset_loading(
    entity_handle: (Entity, Handle<LevelAsset>),
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    level_assets: &Assets<LevelAsset>,
    project_assets: &Assets<Project>,
    query: &mut Query<&mut Transform>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    images: &Assets<Image>,
) {
    let (entity, handle) = entity_handle;

    debug!("finish_level: {entity:?}");

    let Some(level) = level_assets.get(handle.id()) else {
        error!("level handle returned none!");
        return;
    };

    let Some(project) = project_assets.get(&level.project_handle) else {
        error!("project handle returned none!");
        return;
    };

    commands
        .entity(entity)
        .insert(Name::from(level.value.identifier.to_owned()))
        .with_children(|parent| {
            spawn_bg_poly(level, parent, meshes, materials);
            spawn_bg_image(level, parent, meshes, materials, images);
            spawn_layers(project, level, parent, meshes, materials, asset_server);
        });

    match query.get_mut(entity) {
        Ok(mut transform) => {
            transform.translation =
                Vec3::new(level.value.world_x as f32, -level.value.world_y as f32, 0.0)
        }
        Err(e) => {
            error!("level entity doesn't have a transform, or we couldn't get to it! error: {e:?}");
        }
    };
}

fn spawn_bg_poly(
    level: &LevelAsset,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
    let verts = vec![
        [0.0, 0.0, 0.0],
        [level.value.px_wid as f32, 0.0, 0.0],
        [level.value.px_wid as f32, -level.value.px_hei as f32, 0.0],
        [0.0, -level.value.px_hei as f32, 0.0],
    ];

    parent.spawn((
        Name::from("background color"),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Mesh::new(PrimitiveTopology::TriangleList)
                        .with_indices(Some(indices))
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts),
                )
                .into(),
            material: materials.add(ColorMaterial {
                color: crate::util::get_bevy_color_from_ldtk(&level.value.bg_color)
                    .expect("good color"),
                texture: None,
            }),
            ..default()
        },
    ));
}

fn spawn_bg_image(
    level: &LevelAsset,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    images: &Assets<Image>,
) {
    let Some(background_pos) = &level.value.bg_pos else {
        return;
    };

    let Some(background_handle) = &level.bg_image else {
        return;
    };

    let Some(background_image) = images.get(background_handle) else {
        error!("couldn't get background image asset!");
        return;
    };

    let background_image_width = background_image.width() as f32;
    let background_image_height = background_image.height() as f32;

    let uv_left = background_pos.crop_rect[0] as f32 / background_image_width;
    let uv_right =
        (background_pos.crop_rect[0] + background_pos.crop_rect[2]) as f32 / background_image_width;
    let uv_top = background_pos.crop_rect[1] as f32 / background_image_height;
    let uv_bottom = (background_pos.crop_rect[1] + background_pos.crop_rect[3]) as f32
        / background_image_height;

    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
    let verts = vec![
        [0.0, 0.0, 0.0],
        [background_image_width, 0.0, 0.0],
        [background_image_width, -background_image_height, 0.0],
        [0.0, -background_image_height, 0.0],
    ];
    let uvs = vec![
        [uv_left, uv_top],
        [uv_right, uv_top],
        [uv_right, uv_bottom],
        [uv_left, uv_bottom],
    ];

    parent.spawn((
        Name::from("background image"),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Mesh::new(PrimitiveTopology::TriangleList)
                        .with_indices(Some(indices))
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs),
                )
                .into(),
            material: materials.add(ColorMaterial {
                color: Color::default(),
                texture: Some(background_handle.clone()),
            }),
            transform: Transform {
                translation: Vec3::new(
                    background_pos.top_left_px[0] as f32,
                    -background_pos.top_left_px[1] as f32,
                    f32::MIN_POSITIVE,
                ),
                scale: Vec3::new(
                    background_pos.scale[0] as f32,
                    background_pos.scale[1] as f32,
                    1.0,
                ),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_layers(
    project: &Project,
    level: &LevelAsset,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &mut AssetServer,
) {
    level
        .value
        .layer_instances
        .iter()
        .for_each(|layer_instances| {
            layer_instances
                .iter()
                .rev()
                .enumerate()
                .for_each(
                    |(layer_index, layer)| match layer.layer_instance_type.as_str() {
                        "Entities" => spawn_entities_layer(
                            project,
                            level,
                            layer_index,
                            layer,
                            parent,
                            meshes,
                            materials,
                            asset_server,
                        ),
                        "AutoLayer" | "IntGrid" | "Tiles" => spawn_tiles_layer(
                            level,
                            layer_index,
                            layer,
                            parent,
                            meshes,
                            materials,
                            asset_server,
                        ),
                        _ => {
                            error!("Unknown layer type! {}", layer.layer_instance_type);
                        }
                    },
                );
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_tiles_layer(
    level: &LevelAsset,
    layer_index: usize,
    layer: &ldtk_json::LayerInstance,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &mut AssetServer,
) {
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
    let verts = vec![
        [0.0, 0.0, 0.0],
        [level.value.px_wid as f32, 0.0, 0.0],
        [level.value.px_wid as f32, -level.value.px_hei as f32, 0.0],
        [0.0, -level.value.px_hei as f32, 0.0],
    ];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    let layer_handle = asset_server.load(level.ldtk_extras_directory.join("png/").join(format!(
        "{}__{}.png",
        level.value.identifier, layer.identifier
    )));

    parent.spawn((
        Name::from(layer.identifier.clone()),
        LayerInstance {
            value: layer.clone(),
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Mesh::new(PrimitiveTopology::TriangleList)
                        .with_indices(Some(indices))
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs),
                )
                .into(),
            material: materials.add(ColorMaterial {
                color: Color::default(),
                texture: Some(layer_handle.clone()),
            }),
            transform: Transform::from_xyz(0.0, 0.0, (layer_index + 2) as f32 * f32::MIN_POSITIVE),
            ..default()
        },
    ));
}

#[allow(clippy::too_many_arguments)]
fn spawn_entities_layer(
    project: &Project,
    level: &LevelAsset,
    layer_index: usize,
    layer: &ldtk_json::LayerInstance,
    parent: &mut ChildBuilder,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<ColorMaterial>,
    asset_server: &mut AssetServer,
) {
    parent
        .spawn((
            Name::from(layer.identifier.clone()),
            SpatialBundle {
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    (layer_index + 2) as f32 * f32::MIN_POSITIVE,
                ),
                ..default()
            },
        ))
        .with_children(|parent| {
            layer.entity_instances.iter().for_each(|entity_instance| {
                spawn_entity(project, level, entity_instance, parent, asset_server);
            })
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_entity(
    project: &Project,
    level: &LevelAsset,
    entity_instance: &ldtk_json::EntityInstance,
    parent: &mut ChildBuilder,
    asset_server: &mut AssetServer,
) {
    let Some(entity_definition) = project.get_entity_definition(entity_instance.def_uid) else {
        error!("couldn't find entity definition for a layer entity!");
        return;
    };

    parent
        .spawn((
            Name::from(entity_instance.identifier.clone()),
            EntityInstance {
                value: entity_instance.clone(),
                ldtk_project_directory: level.ldtk_project_directory.clone(),
            },
            SpatialBundle {
                transform: Transform::from_xyz(
                    entity_instance.px[0] as f32,
                    -entity_instance.px[1] as f32,
                    0.0,
                ),
                ..default()
            },
        ))
        .with_children(|parent| {
            enum RenderAs {
                _Mesh,
                Sprite,
                // DontRender,
            }

            if let (Some(tileset_rectangle), Some(_tileset_id)) =
                (&entity_instance.tile, entity_definition.tileset_id)
            {
                // let Some(tileset_definition) = project.get_tileset_definition(tileset_id) else {
                //     error!("couldn't find tileset definition!");
                //     return;
                // };

                // https://github.com/deepnight/ldtk/blob/dc348af58d846554cb3bafb9452f245aec275196/src/electron.renderer/display/EntityRender.hx#L138C10-L138C10
                let (render_as, scale) = match entity_definition.tile_render_mode {
                    ldtk_json::TileRenderMode::Cover => todo!(),
                    ldtk_json::TileRenderMode::FitInside => (
                        RenderAs::Sprite,
                        Vec2::splat(f32::min(
                            entity_instance.width as f32 / tileset_rectangle.w as f32,
                            entity_instance.height as f32 / tileset_rectangle.h as f32,
                        )),
                    ),
                    ldtk_json::TileRenderMode::FullSizeCropped => todo!(),
                    ldtk_json::TileRenderMode::FullSizeUncropped => {
                        (RenderAs::Sprite, Vec2::splat(1.0))
                    }
                    ldtk_json::TileRenderMode::NineSlice => todo!(),
                    ldtk_json::TileRenderMode::Repeat => todo!(),
                    ldtk_json::TileRenderMode::Stretch => (
                        RenderAs::Sprite,
                        Vec2::new(
                            entity_instance.width as f32 / tileset_rectangle.w as f32,
                            entity_instance.height as f32 / tileset_rectangle.h as f32,
                        ),
                    ),
                };
                match render_as {
                    RenderAs::_Mesh => todo!(),
                    RenderAs::Sprite => spawn_entity_sprite(
                        project,
                        level,
                        entity_instance,
                        entity_definition,
                        scale,
                        tileset_rectangle,
                        parent,
                        asset_server,
                    ),
                };
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_entity_sprite(
    project: &Project,
    level: &LevelAsset,
    entity_instance: &ldtk_json::EntityInstance,
    entity_definition: &ldtk_json::EntityDefinition,
    scale: Vec2,
    tileset_rectangle: &ldtk_json::TilesetRectangle,
    parent: &mut ChildBuilder,
    asset_server: &mut AssetServer,
) {
    #[allow(illegal_floating_point_literal_pattern)]
    let anchor = match entity_instance.pivot.as_slice() {
        [0.0, 0.0] => Anchor::TopLeft,
        [0.5, 0.0] => Anchor::TopCenter,
        [1.0, 0.0] => Anchor::TopRight,
        [0.0, 0.5] => Anchor::CenterLeft,
        [0.5, 0.5] => Anchor::Center,
        [1.0, 0.5] => Anchor::CenterRight,
        [0.0, 1.0] => Anchor::BottomLeft,
        [0.5, 1.0] => Anchor::BottomCenter,
        [1.0, 1.0] => Anchor::BottomRight,
        _ => {
            error!("bad pivot found! {:?}", entity_instance.pivot);
            return;
        }
    };

    let Some(tilemap_definition) = project.get_tileset_definition(tileset_rectangle.tileset_uid)
    else {
        error!("couldn't find a matching tilemap definition!");
        return;
    };

    let Some(texture) = tilemap_definition
        .rel_path
        .clone()
        .map(|rel_path| asset_server.load(level.ldtk_project_directory.join(rel_path)))
    else {
        error!("no rel_path tilemap definition!");
        return;
    };

    parent.spawn((
        Name::from(format!("Sprite({})", entity_instance.identifier)),
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: entity_definition.tile_opacity as f32,
                },
                // flip_x: todo!(),
                // flip_y: todo!(),
                // custom_size: todo!(),
                rect: Some(Rect::new(
                    tileset_rectangle.x as f32,
                    tileset_rectangle.y as f32,
                    (tileset_rectangle.x + tileset_rectangle.w) as f32,
                    (tileset_rectangle.y + tileset_rectangle.h) as f32,
                )),
                anchor,
                ..default()
            },
            transform: Transform::from_scale(scale.extend(1.0)),
            texture,
            ..default()
        },
    ));
}
