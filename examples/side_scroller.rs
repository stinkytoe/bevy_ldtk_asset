use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;
use bevy_mod_aseprite::{AsepriteBundle, AsepritePlugin};

const LEVEL_PATH: &str = "ldtk/side_scroller.ldtk#The_Grotto";

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Side scroller example for bevy_ldtk_asset".into(),
                        ..default()
                    }),
                    ..default()
                }),
            AsepritePlugin,
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, process_entities_with_aseprite_files)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load(LEVEL_PATH),
        ..default()
    });
}

fn process_entities_with_aseprite_files(
    mut commands: Commands,
    entity_instance_query: Query<(Entity, &Children), Added<LdtkEntityComponent>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, children) in entity_instance_query.iter() {
        commands
            .entity(entity)
            .remove_children(children)
            .with_children(|parent| {
                parent.spawn(AsepriteBundle {
                    // transform: todo!(),
                    // global_transform: todo!(),
                    // animation: AsepriteAnimation::new(, tag),
                    aseprite: asset_server
                        .load("Treasure Hunters/The Crusty Crew/Aseprite/Crabby.aseprite"),
                    ..default()
                });
            });
    }
}
