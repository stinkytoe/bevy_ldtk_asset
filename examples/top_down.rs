use bevy::prelude::*;
use bevy_ldtk_toolkit::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyLdtkToolkitPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>) {
    let _: Handle<ProjectAsset> = asset_server.load("ldtk/top_down.ldtk");
}
