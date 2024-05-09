use bevy::log::LogPlugin;
use bevy::prelude::*;
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
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let Some(player_entity) = player_entity else {
            return;
        };

        entity_component_query
            .set_tile_to_field_instance(player_entity.0, "Swing")
            .expect("Couldn't set tile!");
    }
}
