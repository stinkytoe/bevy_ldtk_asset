use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=debug,top_down=debug".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        // .add_systems(Update, move_player)
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

    commands.spawn(WorldBundle {
        world: asset_server.load("ldtk/side_scroller.ldtk#World"),
        spawn_entities: SpawnEntities::Everything,
        ..default()
    });
}
