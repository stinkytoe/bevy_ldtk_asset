const LDTK_EXAMPLE: &str = "ldtk/example.ldtk";

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use ldtk_bevy_loader::prelude::*;

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "ldtk_bevy_loader=trace".to_string(),
            }),
            LdtkBevyLoaderPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkBundle {
        root: LdtkRoot {
            project: asset_server.load(LDTK_EXAMPLE),
        },
        ..default()
    });
}

fn system(mut gizmos: Gizmos) {
    gizmos.circle(Vec3::ZERO, Vec3::Z, 10.0, Color::ORANGE_RED);
    gizmos.circle(Vec3::new(256.0, 0.0, 0.0), Vec3::Z, 10.0, Color::ORANGE_RED);
}
