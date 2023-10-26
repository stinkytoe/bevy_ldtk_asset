use crate::{bundles::WorldBundle, components, prelude::LdtkProject};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

pub fn update_loaded_ldtk_project(
    mut commands: Commands,
    mut ldtk_load_events: EventReader<AssetEvent<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in ldtk_load_events.read() {
        let LdtkProject {
            value: _,
            worlds,
            assets_path: _assets_path,
        } = {
            let AssetEvent::LoadedWithDependencies { id } = event else {
                return;
            };
            ldtk_project_assets
                .get(asset_server.get_id_handle(*id).unwrap())
                .unwrap()
        };

        for (_world_iid, world) in worlds {
            commands
                .spawn(WorldBundle {
                    world: components::World {
                        iid: _world_iid.clone(),
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for level in world.levels.values() {
                        let verts = vec![
                            [0.0, 0.0, 0.0],
                            [level.px_width as f32, 0.0, 0.0],
                            [level.px_width as f32, -level.px_height as f32, 0.0],
                            [0.0, -level.px_height as f32, 0.0],
                        ];
                        let normals = vec![[0.0, 0.0, 1.0]; 4];
                        let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
                        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

                        parent.spawn(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(
                                    Mesh::new(PrimitiveTopology::TriangleList)
                                        .with_indices(Some(indices))
                                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
                                        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                                        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs),
                                )
                                .into(),
                            material: materials.add(ColorMaterial::from(level.bg_color)),
                            transform: Transform::from_xyz(
                                level.world_x as f32,
                                -level.world_y as f32,
                                level.world_depth as f32,
                            ),
                            ..default()
                        });
                        if let Some(_background) = &level.background {
                            // parent.spawn(SpriteBundle {
                            //     sprite: Sprite {
                            //         // color: background._position,
                            //         // flip_x: (),
                            //         // flip_y: (),
                            //         // custom_size: (),
                            //         // rect: (),
                            //         anchor: bevy::sprite::Anchor::TopLeft,
                            //         ..default()
                            //     },
                            //     transform: Transform::from_xyz(
                            //         level._world_x as f32,
                            //         -level._world_y as f32,
                            //         level._world_depth as f32,
                            //     ),
                            //     // global_transform: todo!(),
                            //     texture: asset_server
                            //         .load(ldtk_project_path_join(assets_path, &background.path)),
                            //     // visibility: todo!(),
                            //     // inherited_visibility: todo!(),
                            //     // view_visibility: todo!(),
                            //     ..default()
                            // });
                        }
                    }
                });
        }
    }
}
