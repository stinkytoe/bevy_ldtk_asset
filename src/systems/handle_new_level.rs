use crate::assets::ldtk_level::LdtkLevel;
use bevy::{prelude::*, sprite::Anchor};

#[allow(clippy::type_complexity)]
pub(crate) fn handle_new_level(
    mut commands: Commands,
    mut changed_level_query: Query<
        (Entity, &mut Transform, &Handle<LdtkLevel>),
        Changed<Handle<LdtkLevel>>,
    >,
    ldtk_level_assets: Res<Assets<LdtkLevel>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut transform, level_handle) in changed_level_query.iter_mut() {
        let level = {
            let Some(asset) = ldtk_level_assets.get(level_handle) else {
                error!("bad handle?");
                return;
            };

            asset
        };

        info!("level loaded: {:#?}", level.value.identifier);
        commands
            .entity(entity)
            .insert(Name::from(level.value.identifier.to_owned()))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        // color: (),
                        // flip_x: (),
                        // flip_y: (),
                        // custom_size: (),
                        // rect: (),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    // transform: todo!(),
                    // global_transform: todo!(),
                    texture: asset_server.load(
                        level
                            .dir
                            .join(format!("png/{}_bg.png", level.value.identifier)),
                    ),
                    // visibility: todo!(),
                    // inherited_visibility: todo!(),
                    // view_visibility: todo!(),
                    ..default()
                });
            });

        transform.translation =
            Vec3::new(level.value.world_x as f32, -level.value.world_y as f32, 0.0);
    }
}
