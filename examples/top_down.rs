use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

const LEVEL_PATH: &str = "ldtk/top_down.ldtk#Island_of_Thieves";

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Top down example for bevy_ldtk_asset".into(),
                        ..default()
                    }),
                    ..default()
                }),
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (register_player_by_tag, move_player))
        .init_resource::<Player>()
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

#[derive(Resource, Debug, Default)]
struct Player(Option<Entity>);

fn register_player_by_tag(
    new_entity_instance_query: Query<(Entity, &LdtkEntityComponent), Added<LdtkEntityComponent>>,
    mut player: ResMut<Player>,
) {
    for (entity, ldtk_entity_component) in new_entity_instance_query.iter() {
        if ldtk_entity_component.has_tag("player") {
            if player.0.is_some() {
                error!("There are more than one entities spawned with the \"player\" tag!");
            } else {
                player.0 = Some(entity);
            }
        }
    }
}

fn move_player(
    mut ldtk_entity_query: Query<(&mut Transform, &LdtkEntityComponent)>,
    player: Res<Player>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    let level_handle: Handle<LdtkLevel> = asset_server.load(LEVEL_PATH);

    let Some((mut player_transform, player_ldtk_entity_component)) =
        player.0.map(|player_entity| {
            ldtk_entity_query
                .get_mut(player_entity)
                .expect("query failed!")
        })
    else {
        return;
    };

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

    if move_attempt == player_transform.translation {
        return;
    };

    let level = level_assets
        .get(level_handle)
        .expect("failed to get the level asset?");

    if let Some(int_grid_value) = level.get_int_grid_value_at_level_coord(move_attempt.truncate()) {
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
