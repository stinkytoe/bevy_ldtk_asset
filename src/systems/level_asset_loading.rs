use crate::{prelude::LdtkLevel, resources::LdtkLevels};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

// #[derive(Component)]
// pub struct LdtkLevelLoaded;

pub fn level_asset_loaded(
    mut levels: ResMut<LdtkLevels>,
    mut ev_asset: EventReader<AssetEvent<LdtkLevel>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
) {
    for ev in ev_asset.read() {
        debug!("ev: {ev:?}");
        if let AssetEvent::<LdtkLevel>::LoadedWithDependencies { id } = ev {
            if let Some((entity, handle)) = level_query
                .iter()
                .find(|(_entity, handle)| handle.id() == *id)
            {
                debug!("Found a matching label and entity! so exciting!");
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
    mut query: Query<&mut Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    debug!("resource changed: {levels:?}");

    if !levels.to_load.is_empty() {
        let to_load: Vec<_> = levels.to_load.drain().collect();
        to_load.iter().for_each(|x| {
            levels.loaded.insert(x.clone());
            finish_level_asset_loading(
                x.clone(),
                &mut commands,
                &mut asset_server,
                &level_assets,
                &mut query,
                &mut meshes,
                &mut materials,
                &images,
            );
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn finish_level_asset_loading(
    entity_handle: (Entity, Handle<LdtkLevel>),
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    level_assets: &Assets<LdtkLevel>,
    query: &mut Query<&mut Transform>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    _images: &Assets<Image>,
) {
    let (entity, handle) = entity_handle;

    debug!("finish_level: {entity:?}");

    let Some(level) = level_assets.get(handle.id()) else {
        return;
    };

    commands
        .entity(entity)
        .insert(Name::from(level.value.identifier.to_owned()))
        .with_children(|parent| {
            let verts = vec![
                [0.0, 0.0, 0.0],
                [level.value.px_wid as f32, 0.0, 0.0],
                [level.value.px_wid as f32, -level.value.px_hei as f32, 0.0],
                [0.0, -level.value.px_hei as f32, 0.0],
            ];
            let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

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

            if let (Some(background_path), Some(background_pos)) = (
                level.value.bg_rel_path.as_ref(),
                level.value.bg_pos.as_ref(),
            ) {
                let background_handle: Handle<Image> =
                    asset_server.load(level.ldtk_sub_files_dir.join("../").join(background_path));

                // let background_image = images
                //     .get(background_handle.id())
                //     .expect("background image");

                let background_image_width = 460;
                let background_image_height = 307;

                let norm_left = background_pos.crop_rect[0] as f32 / background_image_width as f32;

                let norm_right = (background_pos.crop_rect[0] + background_pos.crop_rect[2]) as f32
                    / background_image_width as f32;

                let norm_top = background_pos.crop_rect[1] as f32 / background_image_height as f32;

                let norm_bottom = (background_pos.crop_rect[1] + background_pos.crop_rect[3])
                    as f32
                    / background_image_height as f32;

                debug!("norm_top: {norm_top}");
                debug!("norm_bottom: {norm_bottom}");

                let verts = vec![
                    [0.0, 0.0, 0.0],
                    [level.value.px_wid as f32, 0.0, 0.0],
                    [level.value.px_wid as f32, -level.value.px_hei as f32, 0.0],
                    [0.0, -level.value.px_hei as f32, 0.0],
                ];
                let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);
                let uvs = vec![
                    [norm_left, norm_top],
                    [norm_right, norm_top],
                    [norm_right, norm_bottom],
                    [norm_left, norm_bottom],
                ];

                parent.spawn((
                    Name::from("background"),
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
                            texture: Some(background_handle),
                        }),
                        transform: Transform {
                            translation: Vec3::new(
                                background_pos.top_left_px[0] as f32,
                                -background_pos.top_left_px[1] as f32,
                                f32::MIN_POSITIVE,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });

    let mut transform = query.get_mut(entity).unwrap();

    transform.translation = Vec3::new(level.value.world_x as f32, -level.value.world_y as f32, 0.0);
}
