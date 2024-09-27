use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(asset_server.load::<bevy_ldtk_asset::project::Project>("ldtk/all_features.ldtk"));
}
