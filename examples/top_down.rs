use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

#[derive(Debug, Resource)]
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
    new_entities_with_tag: NewEntitiesWithTag,
) {
    for (entity, entity_component) in new_entities_with_tag.with_tag("player") {
        if player_entity.is_some() {
            error!(
                "An entity with \"player\" tag already registered! {} will be ignored!",
                entity_component.identifier()
            );
        } else {
            info!(
                "Registering new player entity: {}",
                entity_component.identifier()
            );
            commands.insert_resource(PlayerEntity(entity))
        };
    }
}

fn update(
    player_entity: Option<Res<PlayerEntity>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut entity_component_tileset: EntityComponentTileset,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let Some(player_entity) = player_entity else {
            return;
        };

        entity_component_tileset
            .set_tileset_rectangle_to_field_instance(player_entity.0, "Swing")
            .expect("Couldn't set tile!");
    }
}
