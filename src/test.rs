use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::io::embedded::GetAssetServer as _;
use bevy_asset::{AssetPlugin, Assets, Handle, LoadState};
use bevy_ecs::component::Component;
use bevy_image::ImagePlugin;

use crate::iid::iid;
use crate::layer::LayerInstance;
use crate::level::Level;
use crate::plugin::BevyLdtkAssetPlugin;
use crate::project::Project;
use crate::world::World;

#[test]
#[allow(clippy::unwrap_used, clippy::panic)]
fn test_me() {
    let mut app = App::new();

    app.add_plugins(TaskPoolPlugin::default());
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(ImagePlugin::default());
    app.add_plugins(BevyLdtkAssetPlugin);

    let project_handle = app
        .get_asset_server()
        .load::<Project>("ldtk/empty_single_world.ldtk");

    #[derive(Component)]
    struct MyComponent {
        _handle: Handle<Project>,
    }

    app.world_mut().spawn(MyComponent {
        _handle: project_handle.clone(),
    });

    let asset_server = app.get_asset_server().clone();

    // loop {
    //     match asset_server.load_state(project_handle.id()) {
    //         LoadState::NotLoaded => panic!("not loaded?"),
    //         LoadState::Loading => {}
    //         LoadState::Loaded => break,
    //         LoadState::Failed(asset_load_error) => panic!("{asset_load_error}"),
    //     };
    //     app.update();
    // }

    macro_rules! wait_on_asset_loaded {
        ($handle:expr) => {
            loop {
                match asset_server.load_state($handle.id()) {
                    LoadState::NotLoaded => panic!("not loaded?"),
                    LoadState::Loading => {}
                    LoadState::Loaded => break,
                    LoadState::Failed(asset_load_error) => panic!("{asset_load_error}"),
                };
                app.update();
            }
        };
    }

    wait_on_asset_loaded!(project_handle);

    macro_rules! do_the_iid_check {
        ($path:expr, $asset_type:ident, $assets_resource:ident, $iid:expr) => {{
            let _handle = asset_server.load::<$asset_type>($path);
            let _asset = $assets_resource.get(_handle.id()).unwrap();
            assert_eq!(_asset.iid, $iid);
        }};
    }

    let world_assets = app.world().get_resource::<Assets<World>>().unwrap();
    do_the_iid_check!(
        "ldtk/empty_single_world.ldtk#world:World",
        World,
        world_assets,
        iid!("ea1bf700-ac70-11f0-b03c-ff22ab8e0301")
    );

    let level_assets = app.world().get_resource::<Assets<Level>>().unwrap();
    do_the_iid_check!(
        "ldtk/empty_single_world.ldtk#world:World/Level_0",
        Level,
        level_assets,
        iid!("ea1c1e10-ac70-11f0-b03c-5f243de911d6")
    );
    do_the_iid_check!(
        "ldtk/empty_single_world.ldtk#world:World/Level_1",
        Level,
        level_assets,
        iid!("fc64e1e0-ac70-11f0-b744-630b936a2808")
    );

    let layer_assets = app.world().get_resource::<Assets<LayerInstance>>().unwrap();
    do_the_iid_check!(
        "ldtk/empty_single_world.ldtk#world:World/Level_0/Terrain",
        LayerInstance,
        layer_assets,
        iid!("134da8d0-ac70-11f0-b744-152728b44161")
    );
    do_the_iid_check!(
        "ldtk/empty_single_world.ldtk#world:World/Level_1/Terrain",
        LayerInstance,
        layer_assets,
        iid!("fc6508f1-ac70-11f0-b744-b7538a3a8112")
    );
}
