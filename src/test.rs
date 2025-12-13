#![allow(clippy::unwrap_used, clippy::panic)]

use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::io::embedded::GetAssetServer as _;
use bevy_asset::{AssetPlugin, AssetServer, Assets, Handle, LoadState};
use bevy_ecs::component::Component;
use bevy_image::ImagePlugin;

use crate::iid::iid;
use crate::layer::LayerInstance;
use crate::level::Level;
use crate::plugin::BevyLdtkAssetPlugin;
use crate::project::Project;
use crate::world::World;

macro_rules! wait_on_asset_loaded {
    ($asset_server:expr, $app:expr, $handle:expr) => {
        loop {
            match $asset_server.load_state($handle.id()) {
                LoadState::NotLoaded => panic!("not loaded?"),
                LoadState::Loading => {}
                LoadState::Loaded => break,
                LoadState::Failed(asset_load_error) => panic!("{asset_load_error}"),
            };
            $app.update();
        }
    };
}

macro_rules! do_the_iid_check {
    ($asset_server:expr, $path:expr, $asset_type:ident, $assets_resource:ident, $iid:expr) => {{
        let _handle = $asset_server.load::<$asset_type>($path);
        let _asset = $assets_resource.get(_handle.id()).unwrap();
        assert_eq!(_asset.iid, $iid);
    }};
}

fn perpare_and_wait_on_project(project_path: &'static str) -> (Handle<Project>, App, AssetServer) {
    let mut app = App::new();

    app.add_plugins(TaskPoolPlugin::default());
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(ImagePlugin::default());
    app.add_plugins(BevyLdtkAssetPlugin);

    let project_handle = app.get_asset_server().load::<Project>(project_path);

    #[derive(Component)]
    struct MyComponent {
        _handle: Handle<Project>,
    }

    app.world_mut().spawn(MyComponent {
        _handle: project_handle.clone(),
    });

    let asset_server = app.get_asset_server().clone();

    wait_on_asset_loaded!(asset_server, app, project_handle);

    (project_handle, app, asset_server)
}

#[test]
fn single_world() {
    let (_project_handle, app, asset_server) =
        perpare_and_wait_on_project("ldtk/single_world.ldtk");

    let world_assets = app.world().get_resource::<Assets<World>>().unwrap();
    do_the_iid_check!(
        asset_server,
        "ldtk/single_world.ldtk#world:World",
        World,
        world_assets,
        iid!("ea1bf700-ac70-11f0-b03c-ff22ab8e0301")
    );

    let level_assets = app.world().get_resource::<Assets<Level>>().unwrap();
    do_the_iid_check!(
        asset_server,
        "ldtk/single_world.ldtk#world:World/Level_0",
        Level,
        level_assets,
        iid!("ea1c1e10-ac70-11f0-b03c-5f243de911d6")
    );
    do_the_iid_check!(
        asset_server,
        "ldtk/single_world.ldtk#world:World/Level_1",
        Level,
        level_assets,
        iid!("fc64e1e0-ac70-11f0-b744-630b936a2808")
    );

    let layer_assets = app.world().get_resource::<Assets<LayerInstance>>().unwrap();
    do_the_iid_check!(
        asset_server,
        "ldtk/single_world.ldtk#world:World/Level_0/Terrain",
        LayerInstance,
        layer_assets,
        iid!("134da8d0-ac70-11f0-b744-152728b44161")
    );
    do_the_iid_check!(
        asset_server,
        "ldtk/single_world.ldtk#world:World/Level_1/Terrain",
        LayerInstance,
        layer_assets,
        iid!("fc6508f1-ac70-11f0-b744-b7538a3a8112")
    );
}

#[test]
fn multi_world() {
    let (_project_handle, app, asset_server) = perpare_and_wait_on_project("ldtk/multi_world.ldtk");

    let world_assets = app.world().get_resource::<Assets<World>>().unwrap();
    do_the_iid_check!(
        asset_server,
        "ldtk/multi_world.ldtk#world:Overworld",
        World,
        world_assets,
        iid!("ea1bf701-ac70-11f0-b03c-2b67fe2293e1")
    );
    do_the_iid_check!(
        asset_server,
        "ldtk/multi_world.ldtk#world:Underworld",
        World,
        world_assets,
        iid!("f4fecfc0-ac70-11f0-9854-79ba2b0a16c2")
    );
}

#[test]
#[should_panic]
fn embedded_asset_fail() {
    let (_project_handle, _app, _asset_server) =
        perpare_and_wait_on_project("ldtk/embedded_assets.ldtk");
}
