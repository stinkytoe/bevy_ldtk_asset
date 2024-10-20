use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::project::Project;

pub enum LdtkLabel {
    // type: Handle<Project>
    // project.ldtk
    Project {
        path: String,
    },
    // type: Handle<World>
    // project.ldtk#World
    World {
        path: String,
        world: String,
    },
    // type: Handle<Level>
    // project.ldtk#World/Level_1
    Level {
        path: String,
        world: String,
        level: String,
    },
    // type: Handle<Layer>
    // project.ldtk#World/Level_1/Bottom_Layer
    Layer {
        path: String,
        world: String,
        level: String,
        layer: String,
    },
    // type: Handle<Image>
    // project.ldtk#World/Level_1/Bottom_Layer@image
    LayerImage {
        path: String,
        world: String,
        level: String,
        layer: String,
    },
    // type: Handle<Entity>
    // project.ldtk#World/Level_1/Bottom_Layer/Player@ba96b5b0-8990-11ee-b369-6bec2cf1cf1a
    Entity {
        path: String,
        world: String,
        level: String,
        layer: String,
        entity_identity: String,
        entity_iid: String,
    },
}

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
