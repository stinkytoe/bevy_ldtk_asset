use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "bevy_ldtk_asset=debug,top_down=debug".into(),
                ..default()
            }),
            WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(Player::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (register_player_by_tag, update, move_player))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: (256.0, -256.0, 1000.0).into(),
            scale: Vec2::splat(0.5).extend(1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(WorldBundle {
        world: asset_server.load("ldtk/top_down.ldtk#World"),
        spawn_entities: SpawnEntities::Everything,
        ..default()
    });
}

fn update(levels: Res<Assets<LevelAsset>>, levels_query: LevelAtLocationQuery) {
    let _levels_at = levels_at_location(Vec2::ZERO, &levels, levels_query);
    // debug!("{levels_at:?}");
}

#[derive(Resource, Debug, Default)]
struct Player(Option<Entity>);

fn register_player_by_tag(
    new_entity_instance_query: Query<(Entity, &LdtkEntity), Added<LdtkEntity>>,
    mut player: ResMut<Player>,
) {
    for (entity, ldtk_entity) in new_entity_instance_query.iter() {
        if ldtk_entity.has_tag("player") {
            if player.0.is_some() {
                error!("There are more than one entities spawned with the \"player\" tag!");
            } else {
                debug!("Registering player: {}", ldtk_entity.identifier());
                player.0 = Some(entity);
            }
        }
    }
}

fn move_player(
    mut ldtk_entity_query: Query<(&mut Transform, &LdtkEntity)>,
    levels: Res<Assets<LevelAsset>>,
    levels_at_position_query: LevelAtLocationQuery,
    player: Res<Player>,
    keys: Res<ButtonInput<KeyCode>>,
) {
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

    let entity_size = player_ldtk_entity_component.size();

    if keys.just_pressed(KeyCode::ArrowRight) {
        move_attempt.x += entity_size.x;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        move_attempt.x -= entity_size.x;
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        move_attempt.y += entity_size.y;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        move_attempt.y -= entity_size.y;
    }

    if move_attempt == player_transform.translation {
        return;
    };

    let levels_at = levels_at_location(move_attempt.truncate(), &levels, levels_at_position_query);

    // let int_grid_at = int_grid_at_location(move_attempt.truncate());

    debug!("Player standing on level: {levels_at:?}");

    player_transform.translation = move_attempt;

    // if let Some(int_grid_value) =
    //     level.get_int_grid_value_at_level_coordinate(move_attempt.truncate())
    // {
    //     match int_grid_value.identifier().as_deref() {
    //         Some("water") => info!("collision with water!"),
    //         Some(identifier) => {
    //             info!("walking on: {identifier}");
    //             player_transform.translation = move_attempt;
    //         }
    //         None => info!("no identifier"),
    //     }
    // } else {
    //     info("no int grid at attempted move location. We don't know what we're going to be walking on!");
    // }
}
