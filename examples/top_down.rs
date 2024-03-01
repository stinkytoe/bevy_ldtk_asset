use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "bevy_ldtk_asset=debug,top_down=debug".into(),
                ..default()
            }),
            WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(WorldBundle {
        world: asset_server.load("ldtk/top_down.ldtk#World"),
        load_parameters: LoadParameters::Everything,
        ..default()
    });

    commands.spawn(LevelBundle {
        level: asset_server.load("ldtk/top_down.ldtk#World/Island_of_Thieves"),
        load_parameters: LoadParameters::Everything,
        ..default()
    });
}
