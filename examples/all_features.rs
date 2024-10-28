use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin,
    log::{Level, LogPlugin},
    prelude::*,
};

use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            AssetPlugin::default(),
            ImagePlugin::default(),
            LogPlugin {
                level: Level::WARN,
                filter: "bevy_ldtk_asset=trace,example=trace".into(),
                ..default()
            },
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1.0 / 60.0,
            ))),
            //WorldInspectorPlugin::default(),
            bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component, Debug, Reflect)]
pub struct LdtkProject {
    project: Handle<ldtk_asset::Project>,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkProject {
        project: asset_server.load::<bevy_ldtk_asset::project::Project>("ldtk/all_features.ldtk"),
    });
}
