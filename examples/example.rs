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
                    filter: "bevy_ldtk_asset=debug,example=debug".to_string(),
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
    asset_server: Res<AssetServer>,
    project_assets: Res<Assets<LdtkProject>>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    let level_handle: Handle<LdtkLevel> = asset_server.load("ldtk/example.ldtk#Level_0");

    if let Some((mut player_transform, player_ldtk_entity_component)) = ldtk_entity_query
        .iter_mut()
        .find(|(_, ldtk_entity_component)| ldtk_entity_component.has_tag("player"))
    {
        let mut move_attempt = player_transform.translation.truncate();

        if keys.just_pressed(KeyCode::Left) {
            move_attempt.x -= player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Right) {
            move_attempt.x += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Up) {
            move_attempt.y += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Down) {
            move_attempt.y -= player_ldtk_entity_component.value.width as f32;
        }

        if let Some(level) = level_assets.get(level_handle) {
            if let Some(project) = project_assets.get(&level.project) {
                let int_grid_value = level.get_int_grid_value_at_level_coord(project, move_attempt);
                info!("Int Grid value at move attempt: {int_grid_value:?}");
            }
        }

        player_transform.translation = move_attempt.extend(player_transform.translation.z);
    }
}
