use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

const PROJECT_PATH: &str = "ldtk/top_down.ldtk";

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
                })
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=debug,top_down=debug".into(),
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
            // translation: Vec3::ZERO,
            scale: Vec2::splat(0.25).extend(1.0),
            ..default()
        },
        ..default()
    });
    commands.spawn(LdtkProjectBundle {
        project: asset_server.load(PROJECT_PATH),
        ..default()
    });
}

#[derive(Resource, Debug, Default)]
struct Player(Option<Entity>);

fn register_player_by_tag(
    new_entity_instance_query: Query<
        (Entity, &EntityInstance, &GlobalTransform),
        Added<EntityInstance>,
    >,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut player: ResMut<Player>,
) {
    new_entity_instance_query
        .iter()
        .filter(|(_, ldtk_entity_component, _)| ldtk_entity_component.has_tag("player"))
        .for_each(
            |(ecs_entity, ldtk_entity_component, entity_global_transform)| {
                if ldtk_entity_component.has_tag("player") {
                    if player.0.is_some() {
                        error!("There are more than one entities spawned with the \"player\" tag!");
                    } else {
                        player.0 = Some(ecs_entity);
                        let mut camera_transform = camera_query.single_mut();
                        camera_transform.translation = entity_global_transform.translation()
                    }
                }
            },
        );
}

#[allow(clippy::too_many_arguments)]
fn move_player(
    mut commands: Commands,
    mut ldtk_entity_query: Query<
        (Entity, &mut Transform, &GlobalTransform, &EntityInstance),
        Without<Camera2d>,
    >,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    level_query: Query<(&Handle<LevelAsset>, &Children)>,
    parent_query: Query<&Parent>,
    layer_name_query: Query<&Name, With<LayerInstance>>,
    player: Res<Player>,
    keys: Res<ButtonInput<KeyCode>>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    let Some((player_ecs_entity, mut player_transform, player_global_transform, entity_instance)) =
        player.0.map(|player_entity| {
            ldtk_entity_query
                .get_mut(player_entity)
                .expect("ldtk entity query failed!")
        })
    else {
        return;
    };

    let layer_entity = parent_query
        .get(player_ecs_entity)
        .expect("Entity Instance isn't on a layer!")
        .get();

    let level_entity = parent_query
        .get(layer_entity)
        .expect("Layer Instance isn't on a level!")
        .get();

    let (level_handle, _) = level_query.get(level_entity).expect("Bad level entity!");

    let level = level_assets
        .get(level_handle)
        .expect("failed to get the level asset?");

    let mut move_attempt = Vec2::ZERO; //player_transform.translation;

    let entity_size = entity_instance.size();

    if keys.just_pressed(KeyCode::ArrowRight) {
        move_attempt.x = entity_size.x;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        move_attempt.x = -entity_size.x;
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        move_attempt.y = entity_size.y;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        move_attempt.y = -entity_size.y;
    }

    if move_attempt == Vec2::ZERO {
        return;
    };

    let mut camera_transform = camera_query.single_mut();

    if let Some(int_grid_value) = level.get_int_grid_value_at_level_coordinate(
        player_transform.translation.truncate() + move_attempt,
    ) {
        if is_passable_tile(&int_grid_value) {
            player_transform.translation += move_attempt.extend(0.0);
            camera_transform.translation =
                player_global_transform.translation() + move_attempt.extend(0.0);
        }
    } else {
        let new_player_global = player_global_transform.translation().truncate() + move_attempt;

        if let Some((new_level, new_level_children)) =
            level_query
                .iter()
                .find_map(|(level_handle, level_children)| {
                    Some((
                        level_assets
                            .get(level_handle.clone())
                            .and_then(|new_level| {
                                new_level
                                    .contains_world_coordinate(new_player_global)
                                    .then_some(new_level)
                            })?,
                        level_children,
                    ))
                })
        {
            let new_player_translation =
                new_player_global - new_level.get_world_coordinate().truncate();

            if let Some(new_int_grid_value) =
                new_level.get_int_grid_value_at_level_coordinate(new_player_translation)
            {
                if is_passable_tile(&new_int_grid_value) {
                    info!("Moving onto new level: {}", new_level.identifier());
                    let new_entity_layer = new_level_children
                        .iter()
                        .filter(|layer_entity| layer_name_query.contains(**layer_entity))
                        .find(|layer_entity| {
                            layer_name_query
                                .get(**layer_entity)
                                .expect("bad name query?")
                                .as_str()
                                == "Entities"
                        })
                        .expect("Couldn't find the \"Entities\" layer?");
                    commands
                        .entity(player_ecs_entity)
                        .set_parent(*new_entity_layer);
                    player_transform.translation = new_player_translation.extend(0.0);
                    camera_transform.translation = new_player_global.extend(0.0);
                }
            }
        }
    }
}

fn is_passable_tile(int_grid_value: &IntGridValue) -> bool {
    match int_grid_value.identifier().as_deref() {
        Some("water") => {
            info!("collision with water!");
            false
        }
        Some(identifier) => {
            info!("walking on: {identifier}");
            true
        }
        None => {
            info!("no identifier");
            false
        }
    }
}
