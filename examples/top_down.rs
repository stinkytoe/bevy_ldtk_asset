use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=debug,top_down=debug".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
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
            translation: (120.0, -136.0, 1000.0).into(),
            scale: Vec2::splat(0.2).extend(1.0),
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
    mut ldtk_entity_query: Query<
        (&GlobalTransform, &mut Transform, &LdtkEntity),
        Without<Camera2d>,
    >,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_entities_with_tag: LdtkEntitiesWithTag,
    int_grid_at_location: IntGridAtLocation,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_entity) = ldtk_entities_with_tag.find_single("player") else {
        return;
    };

    let Ok((player_global_transform, mut player_transform, player_ldtk_entity_component)) =
        ldtk_entity_query.get_mut(player_entity)
    else {
        return;
    };

    let move_attempt: Vec2 = {
        let entity_size = player_ldtk_entity_component.size();
        match (
            keys.just_pressed(KeyCode::ArrowUp),
            keys.just_pressed(KeyCode::ArrowLeft),
            keys.just_pressed(KeyCode::ArrowDown),
            keys.just_pressed(KeyCode::ArrowRight),
        ) {
            (true, false, false, false) => (0.0, entity_size.y),
            (false, true, false, false) => (-entity_size.x, 0.0),
            (false, false, true, false) => (0.0, -entity_size.y),
            (false, false, false, true) => (entity_size.x, 0.0),
            _ => return,
        }
        .into()
    };

    let player_global_location = player_global_transform.translation().truncate();

    if let Some(IntGridValueDefinition {
        identifier: Some(identifier),
        ..
    }) = int_grid_at_location.top(player_global_location + move_attempt)
    {
        match identifier.as_ref() {
            "bridge" | "dirt" | "grass" => {
                info!("Walking on: {identifier}");
                let mut camera_transform = camera_query.single_mut();
                player_transform.translation += move_attempt.extend(0.0);
                camera_transform.translation = (player_global_location + move_attempt).extend(0.0);
            }
            "water" => info!("Cannot walk on {identifier}!"),
            _ => info!("Unknown int grid value: {identifier}"),
        }
    };
}
