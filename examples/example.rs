use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "bevy_ldtk_asset=debug".to_string(),
            }),
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec2::splat(0.4).extend(1.0)),
        ..default()
    });
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_0"),
        ..default()
    });
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_1"),
        ..default()
    });
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_2"),
        ..default()
    });
}
