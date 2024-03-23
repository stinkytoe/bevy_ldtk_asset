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
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
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

fn move_player(
    mut ldtk_entity_query: Query<(&mut Transform, &LdtkEntity)>,
    ldtk_entities_with_tag: LdtkEntitiesWithTag,
    levels_at_location: LevelsAtLocation,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player_entity = match ldtk_entities_with_tag.find_single("player") {
        Ok(entity) => entity,
        Err(FindSingleError::NoEntities(_)) => {
            // It could be that the entity just isn't loaded yet...
            return;
        }
        Err(FindSingleError::MultipleEntities(e)) => {
            error!("Multiple ldtk entities with player tag!");
            panic!("{e}");
        }
    };

    let (mut player_transform, player_ldtk_entity_component) = ldtk_entity_query
        .get_mut(player_entity)
        .expect("query failed!");

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

    let levels_at = levels_at_location.find(move_attempt.truncate());
    debug!("Player standing on level: {levels_at:?}");

    player_transform.translation = move_attempt;
}
