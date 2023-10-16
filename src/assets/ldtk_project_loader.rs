use crate::assets::ldtk_project::LdtkProject;
use crate::assets::util::ldtk_file_to_asset_path;
use crate::util;
use crate::{ldtk_json, world};
use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};
use std::collections::HashMap;
use std::path::Path;

pub struct LdtkRootLoader;

impl AssetLoader for LdtkRootLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            debug!(
                "Loading LDtk root project file: {}",
                load_context.path().to_str().unwrap_or_default()
            );

            let value: ldtk_json::LdtkJson = serde_json::from_slice(bytes)?;

            let ldtk_file_path = load_context.path().parent().unwrap_or(Path::new(""));

            let worlds = if value.worlds.is_empty() {
                let new_world = world::World::new_from_ldtk_json(&value, load_context);
                debug!("Loading world data from project root.");
                debug!("Since we're constructing from the old style, one world representation,");
                debug!("we'll use (root) as the identifier since one isn't supplied.");
                debug!("Loaded world: {}", new_world.identifier);
                debug!("    with iid: {}", new_world.iid);
                HashMap::from([(value.iid.clone(), new_world)])
            } else {
                debug!("Multi world file found! Will load all levels.");
                value
                    .worlds
                    .iter()
                    .map(|value| {
                        let new_world = world::World::new_from_ldtk_world(value, load_context);
                        debug!("Loaded world: {}", new_world.identifier);
                        debug!("    with iid: {}", new_world.iid);
                        (value.iid.clone(), new_world)
                    })
                    .collect()
            };

            let level_backgrounds = worlds
                .values()
                .flat_map(|world| {
                    world.levels.values().filter_map(|level| {
                        level.bg_rel_path.as_ref().map(|bg_rel_path| {
                            (
                                level.iid.clone(),
                                load_context.get_handle(ldtk_file_to_asset_path(
                                    bg_rel_path,
                                    ldtk_file_path,
                                )),
                            )
                        })
                    })
                })
                .collect::<HashMap<_, Handle<Image>>>();

            debug!("level_backgrounds: {level_backgrounds:?}");

            // let layer_images = value.defs.layers.iter().map(|layer| ());

            let ldtk_project = LdtkProject {
                bg_color: util::get_bevy_color_from_ldtk(&value.bg_color)?,
                defs: value.defs,
                external_levels: value.external_levels,
                iid: value.iid,
                json_version: value.json_version,
                worlds,
                level_backgrounds,
            };

            load_context.set_default_asset(LoadedAsset::new(ldtk_project));

            debug!(
                "Loading LDtk root project file: {} success!",
                load_context.path().to_str().unwrap_or_default()
            );

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
