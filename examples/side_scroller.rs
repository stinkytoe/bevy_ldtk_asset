use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin};

const LEVEL_PATH: &str = "ldtk/side_scroller.ldtk#The_Grotto";

pub mod sprites {
    use bevy_mod_aseprite::aseprite;
    aseprite!(pub Crabby, "../assets/Treasure Hunters/The Crusty Crew/Aseprite/Crabby.aseprite");
}

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
                }),
            AsepritePlugin,
            WorldInspectorPlugin::new(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_entities_added,
                handle_aseprite_loaded,
                draw_collision_boxes,
                debug_keys,
            ),
        )
        .init_resource::<CollisionBoxes>()
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
    // ecs_entity: Entity,
    handle: Handle<Aseprite>,
}

fn handle_entities_added(
    mut commands: Commands,
    // level_query: Query<&LdtkLevelComponent>,
    entity_instance_query: Query<(Entity, &LdtkEntityComponent), Added<LdtkEntityComponent>>,
    asset_server: Res<AssetServer>,
) {
    for (ecs_entity, entity_instance) in entity_instance_query.iter() {
        if let Some(path) = entity_instance
            .value
            .field_instances
            .iter()
            .find(|field_instance| {
                field_instance.field_instance_type.as_str() == "FilePath"
                    && field_instance.identifier == "aseprite"
            })
            .and_then(|field_instance| field_instance.value.as_ref())
            .and_then(|value| value.as_str())
        {
            commands
                .entity(ecs_entity)
                .despawn_descendants()
                .insert(AsepriteImport {
                    // ecs_entity,
                    handle: asset_server.load(entity_instance.ldtk_project_directory.join(path)),
                });
        };
    }
}

fn handle_aseprite_loaded(
    mut commands: Commands,
    aseprite_entity_query: Query<(Entity, &AsepriteImport), With<LdtkEntityComponent>>,
    mut asset_events: EventReader<AssetEvent<Aseprite>>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for event in asset_events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some((ecs_entity, entity_instance_import)) = aseprite_entity_query
                .iter()
                .find(|(_, entity_instance_import)| entity_instance_import.handle.id() == *id)
            {
                let aseprite = aseprites.get(&entity_instance_import.handle).unwrap();
                let animation =
                    AsepriteAnimation::new(aseprite.info(), sprites::Crabby::tags::GROUND);
                commands.entity(ecs_entity).with_children(|parent| {
                    parent.spawn(AsepriteBundle {
                        texture_atlas: aseprite.atlas().clone_weak(),
                        sprite: TextureAtlasSprite::new(animation.current_frame()),
                        aseprite: entity_instance_import.handle.clone_weak(),
                        animation,
                        // sprite: todo!(),
                        // texture_atlas: todo!(),
                        transform: Transform::from_xyz(0.0, 16.0, 0.0),
                        // global_transform: todo!(),
                        // visibility: todo!(),
                        // inherited_visibility: todo!(),
                        // view_visibility: todo!(),
                        ..default()
                    });
                });
            }
        };
    }
}

#[derive(Resource, Default)]
struct CollisionBoxes {
    tiles: bool,
    ecs_entities: bool,
}

fn draw_collision_boxes(
    collision_boxes: Res<CollisionBoxes>,
    level_handles: Query<&Handle<LdtkLevel>, With<LdtkLevelComponent>>,
    levels: Res<Assets<LdtkLevel>>,
    mut gizmos: Gizmos,
) {
    if collision_boxes.tiles {
        level_handles.iter().for_each(|level_handle| {
            let level = levels.get(level_handle).unwrap();

            if let Some(layer_instances) = level.value.layer_instances.as_ref() {
                layer_instances
                    .iter()
                    .filter(|layer_instance| layer_instance.identifier == "Cave")
                    .for_each(|layer_instance| {
                        layer_instance.int_grid_csv.iter().enumerate().for_each(
                            |(index, int_grid_value)| {
                                if *int_grid_value != 0 {
                                    let x = level.value.world_x
                                        + layer_instance.grid_size / 2
                                        + (layer_instance.grid_size
                                            * (index as i64 % layer_instance.c_wid));
                                    let y = level.value.world_y
                                        + layer_instance.grid_size / 2
                                        + (layer_instance.grid_size
                                            * (index as i64 / layer_instance.c_wid));
                                    let location = Vec3::new(x as f32, -y as f32, 0.0);

                                    let size = Vec2::splat(layer_instance.grid_size as f32);

                                    gizmos.rect(location, Quat::IDENTITY, size, Color::RED);
                                }
                            },
                        )
                    });
            }
        });
    }

    if collision_boxes.ecs_entities {}
}

fn debug_keys(keys: Res<Input<KeyCode>>, mut collision_boxes: ResMut<CollisionBoxes>) {
    if keys.just_pressed(KeyCode::F3) {
        collision_boxes.tiles = !collision_boxes.tiles;
    }
    if keys.just_pressed(KeyCode::F4) {
        collision_boxes.ecs_entities = !collision_boxes.ecs_entities;
    }
}
