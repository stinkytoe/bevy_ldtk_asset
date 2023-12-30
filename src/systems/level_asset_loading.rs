use crate::{
    ldtk_json,
    prelude::{LdtkEntityComponent, LdtkLevel, LdtkLevelComponent, LdtkProject},
    resources::LdtkLevels,
};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

pub fn process_level_loading(
    mut levels: ResMut<LdtkLevels>,
    mut ev_asset: EventReader<AssetEvent<LdtkLevel>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>), With<LdtkLevelComponent>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::<LdtkLevel>::LoadedWithDependencies { id } = ev {
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
    level_assets: Res<Assets<LdtkLevel>>,
    project_assets: Res<Assets<LdtkProject>>,
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
    entity_handle: (Entity, Handle<LdtkLevel>),
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    level_assets: &Assets<LdtkLevel>,
    project_assets: &Assets<LdtkProject>,
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

    let Some(project) = project_assets.get(&level.project) else {
        error!("project handle returned none!");
        return;
    };

    commands
        .entity(entity)
        .insert(Name::from(level.value.identifier.to_owned()))
        .with_children(|parent| {
            spawn_bg_poly(level, parent, meshes, materials);
            spawn_bg_image(level, parent, meshes, materials, images);
            spawn_layers(level, project, parent, meshes, materials, asset_server);
        });

    let mut transform = query.get_mut(entity).unwrap();
    transform.translation = Vec3::new(level.value.world_x as f32, -level.value.world_y as f32, 0.0);
}

fn spawn_bg_poly(
    level: &LdtkLevel,
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
    level: &LdtkLevel,
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
    level: &LdtkLevel,
    project: &LdtkProject,
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
                        "IntGrid" => (),
                        "Entities" => spawn_entities_layer(
                            level,
                            project,
                            layer_index,
                            layer,
                            parent,
                            meshes,
                            materials,
                            asset_server,
                        ),
                        "Tiles" => spawn_tiles_layer(
                            level,
                            layer_index,
                            layer,
                            parent,
                            meshes,
                            materials,
                            asset_server,
                        ),
                        "AutoLayer" => (),
                        _ => {
                            error!("Unknown layer type! {}", layer.layer_instance_type);
                        }
                    },
                );
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_tiles_layer(
    level: &LdtkLevel,
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
    level: &LdtkLevel,
    project: &LdtkProject,
    layer_index: usize,
    layer: &ldtk_json::LayerInstance,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
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
            layer.entity_instances.iter().for_each(|layer_entity| {
                let mut entity_builder = parent.spawn((
                    Name::from(layer_entity.identifier.clone()),
                    LdtkEntityComponent {
                        value: layer_entity.clone(),
                    },
                    SpatialBundle {
                        transform: Transform::from_xyz(
                            layer_entity.px[0] as f32,
                            -layer_entity.px[1] as f32,
                            0.0,
                        ),
                        ..default()
                    },
                ));

                if let Some(tileset_rectangle) = layer_entity.tile.as_ref() {
                    let Some(tilemap_definition) =
                        project
                            .value
                            .defs
                            .tilesets
                            .iter()
                            .find(|tileset_definition| {
                                tileset_definition.uid == tileset_rectangle.tileset_uid
                            })
                    else {
                        error!("couldn't find a matching tilemap definition!");
                        return;
                    };

                    let tilemap_handle = tilemap_definition.rel_path.clone().map(|rel_path| {
                        asset_server.load(level.ldtk_project_directory.join(rel_path))
                    });

                    let image_width = tilemap_definition.px_wid as f32;
                    let image_height = tilemap_definition.px_hei as f32;

                    let uv_left = tileset_rectangle.x as f32 / image_width;
                    let uv_right = tileset_rectangle.w as f32 / image_width;
                    let uv_top = tileset_rectangle.y as f32 / image_height;
                    let uv_bottom = tileset_rectangle.h as f32 / image_height;

                    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
                    let verts = vec![
                        [0.0, 0.0, 0.0],
                        [layer_entity.width as f32, 0.0, 0.0],
                        [layer_entity.width as f32, -layer_entity.height as f32, 0.0],
                        [0.0, -layer_entity.height as f32, 0.0],
                    ];
                    let uvs = vec![
                        [uv_left, uv_top],
                        [uv_right, uv_top],
                        [uv_right, uv_bottom],
                        [uv_left, uv_bottom],
                    ];

                    entity_builder.with_children(|parent| {
                        parent.spawn(MaterialMesh2dBundle {
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
                                texture: tilemap_handle,
                            }),
                            transform: Transform::from_xyz(
                                // layer_entity.px[0] as f32,
                                // -layer_entity.px[1] as f32,
                                0.0, 0.0, 0.0,
                            ),
                            ..default()
                        });
                    });
                } else {
                    // entity_builder.insert(SpatialBundle {
                    //     transform: Transform::from_xyz(
                    //         layer_entity.px[0] as f32,
                    //         -layer_entity.px[1] as f32,
                    //         0.0,
                    //     ),
                    //     ..default()
                    // });
                }
            })
        });
}
