use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin};

const LEVEL_PATH: &str = "ldtk/side_scroller.ldtk#The_Grotto";

pub mod sprites {
    use bevy_mod_aseprite::aseprite;
    aseprite!(pub Crabby, "../assets/Treasure Hunters/The Crusty Crew/Aseprite/Crabby.aseprite");
}

#[derive(Resource, Debug, Default)]
struct Player(Option<Entity>);

fn main() {
    App::new() //
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Side scroller example for bevy_ldtk_asset".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::WARN,
                    filter: "bevy_ldtk_asset=debug,side_scroller=debug".into(),
                }),
            AsepritePlugin,
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .init_resource::<Player>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_entities_added,
                handle_layers_added,
                handle_aseprite_loaded,
                draw_debug_collision_boxes,
                debug_keys,
                detect_collisions,
            ),
        )
        .init_resource::<CollisionBoxSettings>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkLevelBundle {
        level: asset_server.load(LEVEL_PATH),
        ..default()
    });
}

#[derive(Component)]
struct AsepriteImport {
    handle: Handle<Aseprite>,
}

#[derive(Component)]
struct LdtkCollider;

fn handle_entities_added(
    mut commands: Commands,
    entity_instance_query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
) {
    for (ecs_entity, entity_instance) in entity_instance_query.iter() {
        // This section will scan for entity instances which have a field instance
        // named 'aseprite,' which points to an aseprite file (presumably).
        // If it does, then we remove all children of the entity instance
        // and inserts an AsepriteImport component.
        entity_instance
            .field_instances()
            .find(|field_instance| {
                field_instance.field_instance_type == "FilePath"
                    && field_instance.identifier == "aseprite"
            })
            .and_then(|field_instance| field_instance.value.as_ref())
            .and_then(|value| value.as_str())
            .iter()
            .for_each(|&aseprite_path| {
                commands
                    .entity(ecs_entity)
                    .despawn_descendants()
                    .insert(AsepriteImport {
                        handle: asset_server
                            .load(entity_instance.ldtk_project_directory.join(aseprite_path)),
                    });
            });

        if entity_instance.has_tag("collider") {
            commands.entity(ecs_entity).insert(LdtkCollider);
        }

        if entity_instance.has_tag("player") {
            if player.0.is_some() {
                error!("There are more than one entities spawned with the \"player\" tag!");
            } else {
                player.0 = Some(ecs_entity);
            }
        }
    }
}

#[derive(Component)]
struct LdtkCollisionBlock {
    _size: Vec2,
}

fn handle_layers_added(
    mut commands: Commands,
    layer_query: Query<(Entity, &LayerInstance), Added<LayerInstance>>,
) {
    layer_query
        .iter()
        .filter(|(_, layer_instance)| layer_instance.identifier() == "Cave")
        .for_each(|(layer_entity, cave_layer)| {
            cave_layer
                .int_grid_csv()
                .iter()
                .enumerate()
                .filter(|(_, &value)| value != 0)
                .filter_map(|(index, _)| cave_layer.get_cell_bounds_from_index(index))
                .for_each(|(cell_coord, size)| {
                    commands.entity(layer_entity).with_children(|parent| {
                        parent.spawn((
                            LdtkCollisionBlock { _size: size },
                            SpatialBundle {
                                transform: Transform::from_translation(cell_coord),
                                ..default()
                            },
                        ));
                    });
                });
        });
}

fn handle_aseprite_loaded(
    mut commands: Commands,
    aseprite_entity_query: Query<(Entity, &AsepriteImport), With<EntityInstance>>,
    mut asset_events: EventReader<AssetEvent<Aseprite>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            aseprite_entity_query
                .iter()
                .filter(|(_, entity_instance_import)| entity_instance_import.handle.id() == *id)
                .for_each(|(ecs_entity, entity_instance_import)| {
                    let aseprite = aseprites.get(&entity_instance_import.handle).unwrap();
                    let animation =
                        AsepriteAnimation::new(aseprite.info(), sprites::Crabby::tags::GROUND);
                    commands.entity(ecs_entity).with_children(|parent| {
                        parent.spawn((
                            Name::from("aseprite"),
                            AsepriteBundle {
                                texture_atlas: aseprite.atlas().clone_weak(),
                                sprite: TextureAtlasSprite::new(animation.current_frame()),
                                aseprite: entity_instance_import.handle.clone_weak(),
                                animation,
                                transform: Transform::from_xyz(0.0, 16.0, 0.0),
                                ..default()
                            },
                        ));
                    });
                });
        }
    }
}

#[derive(Resource, Default)]
struct CollisionBoxSettings {
    tiles: bool,
    ecs_entities: bool,
}

fn draw_debug_collision_boxes(
    debug_collision_box_settings: Res<CollisionBoxSettings>,
    level_handles: Query<&Handle<LevelAsset>, With<LevelComponent>>,
    levels: Res<Assets<LevelAsset>>,
    entities: Query<(&GlobalTransform, &EntityInstance)>,
    mut gizmos: Gizmos,
) {
    if debug_collision_box_settings.tiles {
        level_handles
            .iter()
            .filter_map(|level_handle| levels.get(level_handle))
            .filter_map(|level| {
                Some((
                    level.get_world_coordinate(),
                    level.get_layer_instance_by_identifier("Cave")?,
                ))
            })
            .for_each(|(level_world_coord, cave_layer)| {
                cave_layer
                    .int_grid_csv()
                    .iter()
                    .enumerate()
                    .filter(|(_, &value)| value != 0)
                    .filter_map(|(index, _)| cave_layer.get_cell_bounds_from_index(index))
                    .for_each(|(level_coord, size)| {
                        gizmos.rect(
                            level_coord + level_world_coord,
                            Quat::IDENTITY,
                            size,
                            Color::RED,
                        );
                    });
            });
    }

    if debug_collision_box_settings.ecs_entities {
        entities
            .iter()
            .for_each(|(entity_global_transform, entity_instance)| {
                let location = entity_global_transform.translation();
                let size = entity_instance.size();
                let pivot = entity_instance.pivot();
                let offset = Vec3::new(size.x * (pivot.x - 0.5), size.y * (pivot.y - 0.5), 0.0);
                gizmos.rect(location + offset, Quat::IDENTITY, size, Color::GREEN);
            })
    }
}

fn debug_keys(
    keys: Res<Input<KeyCode>>,
    mut collision_boxes: ResMut<CollisionBoxSettings>,
    // mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if keys.just_pressed(KeyCode::F3) {
        collision_boxes.tiles = !collision_boxes.tiles;
    }
    if keys.just_pressed(KeyCode::F4) {
        collision_boxes.ecs_entities = !collision_boxes.ecs_entities;
    }
}

// fn player_move_keys() {}

fn detect_collisions(
    _collider_query: Query<(Entity, &GlobalTransform, &LdtkCollisionBlock), With<LdtkCollider>>,
    _collision_box_query: Query<
        (Entity, &GlobalTransform, &LdtkCollisionBlock),
        Without<LdtkCollider>,
    >,
) {
    // for (collider_entity, collider_transform, collider_block) in collider_query.iter() {
    //     for (collision_box_entity, collision_transform, collision_block) in
    //         collision_box_query.iter()
    //     {
    //         let collided = collide_aabb::collide(
    //             collider_transform.translation(),
    //             collider_block.size,
    //             collision_transform.translation(),
    //             collision_block.size,
    //         );
    //     }
    // }
}
