use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=debug,side_scroller=debug".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(64.0),
            RapierDebugRenderPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, added_ldtk_entities)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        // transform: Transform {
        //     translation: (120.0, -136.0, 1000.0).into(),
        //     scale: Vec2::splat(0.2).extend(1.0),
        //     ..default()
        // },
        ..default()
    });

    commands.spawn(LevelBundle {
        level: asset_server.load("ldtk/side_scroller.ldtk#World/The_Grotto"),
        spawn_entities: SpawnEntities::Everything,
        ..default()
    });
}

fn added_ldtk_entities(
    mut commands: Commands,
    added_ldtk_entities_query: Query<(Entity, &LdtkEntity), Added<LdtkEntity>>,
) {
    for (entity, ldtk_entity) in added_ldtk_entities_query.iter() {
        if ldtk_entity.has_tag("player") {
            info!("Adding KinematicCharacterController to entity: {entity:?}");
            commands.entity(entity).insert((
                // Collider::cuboid(ldtk_entity.size().x / 2.0, ldtk_entity.size().y / 2.0),
                KinematicCharacterController {
                    // translation: Some(ldtk_entity.pivot() * ldtk_entity.size() * Vec2::splat(0.5)),
                    custom_shape: Some((
                        Collider::cuboid(ldtk_entity.size().x / 2.0, ldtk_entity.size().y / 2.0),
                        (ldtk_entity.pivot() * ldtk_entity.size() * Vec2::splat(0.5)),
                        // ldtk_entity.pivot(),
                        0.0,
                    )),
                    // custom_mass: todo!(),
                    up: Vec2::Y,
                    // offset: todo!(),
                    // slide: todo!(),
                    // autostep: todo!(),
                    // max_slope_climb_angle: todo!(),
                    // min_slope_slide_angle: todo!(),
                    // apply_impulse_to_dynamic_bodies: todo!(),
                    // snap_to_ground: todo!(),
                    // filter_flags: todo!(),
                    // filter_groups: todo!(),
                    ..default()
                },
            ));
        }
    }
}
