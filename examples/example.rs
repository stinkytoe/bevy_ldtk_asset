const LDTK_EXAMPLE: &str = "ldtk/example.ldtk";

use bevy::log::LogPlugin;
use bevy::prelude::*;
use ldtk_bevy_loader::prelude::*;

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
            project: asset_server.load(LDTK_EXAMPLE),
        },
        world_set: WorldSet::All,
        level_set: LevelSet::All,
        ..default()
    });
}
