use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "bevy_ldtk_asset=trace".to_string(),
            }),
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_0"),
        ..default()
    });
}

fn system(mut _gizmos: Gizmos) {
    // gizmos.circle(Vec3::ZERO, Vec3::Z, 10.0, Color::ORANGE_RED);
    // gizmos.circle(Vec3::new(256.0, 0.0, 0.0), Vec3::Z, 10.0, Color::ORANGE_RED);
    // gizmos.circle(Vec3::new(128.0, -256.0, 0.0), Vec3::Z, 10.0, Color::RED);
}
