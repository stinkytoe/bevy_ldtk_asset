use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "bevy_ldtk_asset=debug".to_string(),
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::ZERO,
            scale: Vec2::splat(0.25).extend(1.0),
            ..default()
        },
        ..default()
    });
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load("ldtk/example.ldtk#Level_0"),
        ..default()
    });
}

fn move_player(
    // mut commands: Commands,
    mut ldtk_entity_query: Query<(&mut Transform, &LdtkEntityComponent)>,
    keys: Res<Input<KeyCode>>,
) {
    if let Some((mut player_transform, player_ldtk_entity_component)) = ldtk_entity_query
        .iter_mut()
        .find(|(_, ldtk_entity_component)| {
            ldtk_entity_component
                .value
                .tags
                .contains(&"player".to_string())
        })
    {
        if keys.just_pressed(KeyCode::Left) {
            player_transform.translation.x -= player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Right) {
            player_transform.translation.x += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Up) {
            player_transform.translation.y += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Down) {
            player_transform.translation.y -= player_ldtk_entity_component.value.width as f32;
        }
    }
}
