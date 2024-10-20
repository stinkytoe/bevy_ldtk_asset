use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::WARN,
                filter: "bevy_ldtk_asset=debug,example=trace".into(),
                ..default()
            }),
            WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, startup)
        .register_type::<Iid>()
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(asset_server.load::<ldtk_asset::Project>("ldtk/top_down.ldtk"));
}
