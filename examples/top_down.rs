use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_levels::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    // level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_levels=trace,top_down=trace".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            LdtkLevelsPlugins,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(WorldBundle {
        world: asset_server.load("ldtk/top_down.ldtk#World"),
        settings: WorldBundleLoadSettings {
            load_levels: LoadLevels::All,
            level_bundle_load_settings: LevelBundleLoadSettings {
                // load_bg_color: false,
                // load_bg_image: false,
                // load_int_grids: (),
                // load_layers: (),
                // load_layer_settings: (),
                // load_entities: (),
                // load_entity_settings: (),
                ..default()
            },
        },
    });
}
