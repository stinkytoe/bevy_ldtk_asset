use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

#[derive(Clone, Copy, Debug, Resource)]
struct PlayerEntity(Entity);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    // level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=trace,top_down=trace".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            LdtkLevelsPlugins,
            FpsCounterPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (identify_player_entity, update))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::splat(0.3),
            ..default()
        },
        ..default()
    });

    commands.spawn(WorldBundle {
        world: asset_server.load("ldtk/top_down.ldtk#World"),
        ..default()
    });
}

fn identify_player_entity(
    mut commands: Commands,
    player_entity: Option<Res<PlayerEntity>>,
    entity_component_query: EntityComponentQuery,
) {
    let mut new_player: Option<PlayerEntity> = player_entity.map(|player_entity| *player_entity);

    for (entity, entity_component) in entity_component_query.new_with_tag("player") {
        if new_player.is_some() {
            error!(
                "An entity with \"player\" tag already registered! {} will be ignored!",
                entity_component.identifier()
            );
        } else {
            info!(
                "Registering new player entity: {}",
                entity_component.identifier()
            );
            // commands.insert_resource(PlayerEntity(entity))
            new_player = Some(PlayerEntity(entity));
        };
    }

    if let Some(new_player) = new_player {
        commands.insert_resource(new_player);
    }
}

fn update(
    player_entity: Option<Res<PlayerEntity>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut entity_component_query: EntityComponentQuery,
    entity_global_transform_query: Query<&GlobalTransform>,
    level_component_query: LevelComponentQuery,
) {
    let Some(PlayerEntity(player_entity)) = player_entity.map(|player_entity| *player_entity)
    else {
        return;
    };

    let player_global_transform = entity_global_transform_query
        .get(player_entity)
        .expect("Player doesn't have a global transform? {e:?}");

    if keyboard_input.just_pressed(KeyCode::Space) {
        let tile = entity_component_query
            .get_field_instance(player_entity, "Swing")
            .expect("No field instance named \"Swing\"?")
            .as_tile()
            .expect("Field Instance \"Swing\" exists, but isn't a tile! {e:?}");

        entity_component_query.set_tile(player_entity, tile.clone());
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        info!(
            "Levels at location: {:?}",
            level_component_query
                .levels_at_world_location(player_global_transform.translation().truncate())
                .map(|(_, level_component)| level_component.identifier())
                .collect::<Vec<_>>()
        );
    }

    let up =
        keyboard_input.just_pressed(KeyCode::ArrowUp) | keyboard_input.just_pressed(KeyCode::KeyW);
    let left = keyboard_input.just_pressed(KeyCode::ArrowLeft)
        | keyboard_input.just_pressed(KeyCode::KeyA);
    let down = keyboard_input.just_pressed(KeyCode::ArrowDown)
        | keyboard_input.just_pressed(KeyCode::KeyS);
    let right = keyboard_input.just_pressed(KeyCode::ArrowRight)
        | keyboard_input.just_pressed(KeyCode::KeyD);

    match (up, left, down, right) {
        (true, false, false, false) /* ↑ */ => todo!(),
        (false, true, false, false) /* ← */ => todo!(),
        (false, false, true, false) /* ↓ */ => todo!(),
        (false, false, false, true) /* → */ => todo!(),
        _ => (),
    }
}
