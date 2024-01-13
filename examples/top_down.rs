use bevy::{log::LogPlugin, prelude::*, utils::info};
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
                    filter: "bevy_ldtk_asset=debug".into(),
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
    commands.spawn(LdtkProjectBundle {
        project: asset_server.load(PROJECT_PATH),
        ..default()
    });
}

#[derive(Resource, Debug, Default)]
struct Player(Option<Entity>);

fn register_player_by_tag(
    new_entity_instance_query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    mut player: ResMut<Player>,
) {
    for (ecs_entity, ldtk_entity_component) in new_entity_instance_query.iter() {
        if ldtk_entity_component.has_tag("player") {
            if player.0.is_some() {
                error!("There are more than one entities spawned with the \"player\" tag!");
            } else {
                player.0 = Some(ecs_entity);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn move_player(
    mut ldtk_entity_query: Query<(Entity, &mut Transform, &EntityInstance), Without<Camera2d>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    level_query: Query<&Handle<LevelAsset>>,
    parent_query: Query<&Parent>,
    player: Res<Player>,
    keys: Res<Input<KeyCode>>,
    // asset_server: Res<AssetServer>,
    level_assets: Res<Assets<LevelAsset>>,
    // project_assets: Res<Assets<ProjectAsset>>,
) {
    let Some((player_ecs_entity, mut player_transform, entity_instance)) =
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

    let level_handle = level_query.get(level_entity).expect("Bad level entity!");

    let level = level_assets
        .get(level_handle)
        .expect("failed to get the level asset?");

    // let project_handle: Handle<ProjectAsset> = asset_server.load(PROJECT_PATH);
    //
    // let project = project_assets
    //     .get(project_handle)
    //     .expect("failed to get the project asset?");

    let mut move_attempt = player_transform.translation;

    let entity_size = entity_instance.size();

    if keys.just_pressed(KeyCode::Right) {
        move_attempt.x += entity_size.x;
    }

    if keys.just_pressed(KeyCode::Left) {
        move_attempt.x -= entity_size.x;
    }

    if keys.just_pressed(KeyCode::Up) {
        move_attempt.y += entity_size.y;
    }

    if keys.just_pressed(KeyCode::Down) {
        move_attempt.y -= entity_size.y;
    }

    if move_attempt == player_transform.translation {
        return;
    };

    if let Some(int_grid_value) =
        level.get_int_grid_value_at_level_coordinate(move_attempt.truncate())
    {
        match int_grid_value.identifier().as_deref() {
            Some("water") => info!("collision with water!"),
            Some(identifier) => {
                info!("walking on: {identifier}");
                let mut camera_transform = camera_query.single_mut();
                camera_transform.translation += move_attempt - player_transform.translation;
                player_transform.translation = move_attempt;
            }
            None => info!("no identifier"),
        }
    } else {
        info("no int grid at attempted move location. We don't know what we're going to be walking on!");
    }
}
