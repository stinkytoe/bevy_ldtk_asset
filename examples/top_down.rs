use bevy::{asset::LoadState, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ldtk_asset::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            BevyLdtkAssetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, try_load_image)
        .run();
}

#[derive(Bundle)]
struct TestBundle {
    project: Handle<ProjectAsset>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let _: Handle<ProjectAsset> = asset_server.load("ldtk/top_down.ldtk");
    commands.spawn(TestBundle {
        project: asset_server.load("ldtk/top_down.ldtk"),
    });
}

fn try_load_image(
    asset_server: Res<AssetServer>,
    project_assets: Res<Assets<ProjectAsset>>,
    image_assets: Res<Assets<Image>>,
    world_assets: Res<Assets<LdtkWorld>>,
    mut ev_asset: EventReader<AssetEvent<ProjectAsset>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            info!("Bevy claims asset is loaded!");
            let project = project_assets.get(*id).unwrap();

            project
                .worlds()
                .for_each(|world_name| info!("{world_name}"));

            project.world_handles().for_each(|handle| {
                info!("{:?}", world_assets.get(handle.clone_weak()).unwrap());
            });

            project.tilesets().for_each(|handle| {
                info!("{:?}", image_assets.get(handle.clone_weak()).unwrap());
            })
        }
    }
}
