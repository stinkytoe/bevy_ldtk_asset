// use bevy::ecs::system::SystemParam;
use bevy::log::LogPlugin;
use bevy::prelude::*;
// use bevy::render::primitives::Aabb;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

#[derive(Debug, Default, Resource)]
struct PlayerEntity(Option<Entity>);

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
            // WorldInspectorPlugin::default(),
            LdtkLevelsPlugins,
        ))
        .insert_resource(PlayerEntity::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (identify_player_entity, update))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: (256.0, -128.0, 0.0).into(),
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
    mut player_entity: ResMut<PlayerEntity>,
    new_entity_component_query: Query<(Entity, &EntityComponent), Added<EntityComponent>>,
) {
    for (entity, entity_component) in new_entity_component_query
        .iter()
        .filter(|(_, ec)| ec.has_tag("player"))
    {
        if player_entity.0.is_some() {
            error!(
                "An entity with \"player\" tag already registered! {} will be ignored!",
                entity_component.identifier()
            );
        } else {
            info!(
                "Registering new player entity: {}",
                entity_component.identifier()
            );
            player_entity.0 = Some(entity);
        }
    }
}

// #[derive(SystemParam)]
// struct MySysParam<'w, 's> {
//     pub world_assets: Res<'w, Assets<WorldAsset>>,
//     // pub query: Query<'w, 's, Entity, With<EntityComponent>>,
//     layer_query: Query<'w, 's, (&'static Aabb, Entity, &'static LayerComponent)>,
// }

fn update(// my_sys_param: MySysParam
) {
    // for x in my_sys_param.layer_query.iter() {}
}
