use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

const LEVEL_PATH: &str = "ldtk/int_grid_rules.ldtk#Level_0";

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
        level: asset_server.load(LEVEL_PATH),
        ..default()
    });
}

fn move_player(
    mut ldtk_entity_query: Query<(&mut Transform, &LdtkEntityComponent)>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    let level_handle: Handle<LdtkLevel> = asset_server.load(LEVEL_PATH);

    if let Some((mut player_transform, player_ldtk_entity_component)) = ldtk_entity_query
        .iter_mut()
        .find(|(_, ldtk_entity_component)| ldtk_entity_component.has_tag("player"))
    {
        let mut move_attempt = player_transform.translation;

        if keys.just_pressed(KeyCode::Right) {
            move_attempt.x += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Left) {
            move_attempt.x -= player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Up) {
            move_attempt.y += player_ldtk_entity_component.value.width as f32;
        }

        if keys.just_pressed(KeyCode::Down) {
            move_attempt.y -= player_ldtk_entity_component.value.width as f32;
        }

        let Some(level) = level_assets.get(level_handle) else {
            return;
        };

        if move_attempt != player_transform.translation {
            if let Some(int_grid_value) =
                level.get_int_grid_value_at_level_coord(move_attempt.truncate())
            {
                match int_grid_value.identifier.as_deref() {
                    Some("water") => info!("collision with water!"),
                    Some(identifier) => {
                        info!("walking on: {identifier}");
                        player_transform.translation = move_attempt;
                    }
                    None => info!("no identifier"),
                }
            }
        }
    }
}
