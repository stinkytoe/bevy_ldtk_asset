use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::{
    inspector_egui_impls::InspectorPrimitive, quick::WorldInspectorPlugin,
    DefaultInspectorConfigPlugin,
};
use bevy_ldtk_asset::{iid::Iid, plugin::BevyLdtkAssetPlugin, project::Project};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: log::Level::WARN,
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
    commands.spawn(asset_server.load::<Project>("ldtk/top_down.ldtk"));
}
