const LDTK_EXAMPLE: &str = "ldtk/example.ldtk";

use bevy::log::LogPlugin;
use bevy::prelude::*;
use ldtk_bevy_loader::LdtkBevyLoaderPlugin;
use ldtk_bevy_loader::LdtkBundle;
use ldtk_bevy_loader::LdtkRoot;

fn main() {
    App::new()
        // Bevy
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "ldtk_bevy_loader=trace".to_string(),
                ..default()
            }),
            LdtkBevyLoaderPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkBundle {
        root: LdtkRoot {
            root: asset_server.load(LDTK_EXAMPLE),
        },
        ..default()
    });
}
