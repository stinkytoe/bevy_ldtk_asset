use crate::assets::ldtk_level::LdtkLevel;
use crate::assets::ldtk_project::LdtkProject;
use crate::assets::util::ldtk_file_to_asset_path;
use crate::ldtk_json;
use crate::util;
use anyhow::Result;
use bevy::asset::AssetPath;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};
use std::collections::HashMap;
use std::path::Path;

pub(crate) struct LdtkRootLoader;

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

            let ldtk_project_path = load_context.path().parent().unwrap_or(Path::new(""));

            let level_backgrounds_meta = if value.worlds.is_empty() {
                value.levels.iter().collect()
            } else {
                value
                    .worlds
                    .iter()
                    .flat_map(|world| &world.levels)
                    .collect::<Vec<_>>()
            }
            .iter()
            .filter_map(|level| {
                level.bg_rel_path.as_ref().map(|bg_rel_path| {
                    let ldtk_level_background_asset_path =
                        ldtk_file_to_asset_path(bg_rel_path, ldtk_project_path);
                    trace!("Adding level background to set: {ldtk_level_background_asset_path:?}");
                    (
                        level.iid.clone(),
                        ldtk_level_background_asset_path.clone(),
                        load_context.get_handle(ldtk_level_background_asset_path),
                    )
                })
            })
            .collect::<Vec<(String, AssetPath, Handle<Image>)>>();

            let level_file_handles_meta = if value.worlds.is_empty() {
                value.levels.iter().collect()
            } else {
                value
                    .worlds
                    .iter()
                    .flat_map(|world| &world.levels)
                    .collect::<Vec<_>>()
            }
            .iter()
            .filter_map(|level| {
                level.external_rel_path.as_ref().map(|external_rel_path| {
                    let ldtk_level_asset_path =
                        ldtk_file_to_asset_path(external_rel_path, ldtk_project_path);
                    trace!("Adding level file to set: {ldtk_level_asset_path:?}");
                    (
                        level.iid.clone(),
                        ldtk_level_asset_path.clone(),
                        load_context.get_handle(ldtk_level_asset_path),
                    )
                })
            })
            .collect::<Vec<(String, AssetPath, Handle<LdtkLevel>)>>();

            let tilesets = value.defs.tilesets.iter().filter_map(|tileset| {
                if tileset.embed_atlas.is_none() {
                    tileset.rel_path.as_ref().map(|rel_path| {
                        let ldtk_tileset_asset_path =
                            ldtk_file_to_asset_path(rel_path, ldtk_project_path);
                        trace!("Adding tileset to set: {ldtk_tileset_asset_path:?}");
                        (
                            tileset.uid,
                            ldtk_tileset_asset_path.clone(),
                            load_context.get_handle_untyped(ldtk_tileset_asset_path),
                        )
                    })
                } else {
                    None
                }
            });

            let _world_level_map = if value.worlds.is_empty() {
                HashMap::from([(
                    value.iid.clone(),
                    value.levels.iter().map(|value| value.iid.clone()).collect(),
                )])
            } else {
                value
                    .worlds
                    .iter()
                    .map(|value| {
                        (
                            value.iid.clone(),
                            value.levels.iter().map(|value| value.iid.clone()).collect(),
                        )
                    })
                    .collect()
            };

            let ldtk_project = LdtkProject {
                _bg_color: util::get_bevy_color_from_ldtk(&value.bg_color)?,
                level_backgrounds: level_backgrounds_meta
                    .iter()
                    .map(|(id, _, handle)| (id.clone(), handle.clone()))
                    .collect(),
                level_file_handles: level_file_handles_meta
                    .iter()
                    .map(|(id, _, handle)| (id.clone(), handle.clone()))
                    .collect(),
                tilesets: //tilesets_meta
                    //.iter()
                    tilesets.clone()
                    .map(|(id, _, handle)| (id, handle.typed()))
                    .collect(),
                _value: value.clone(),
                world_level_map: _world_level_map, // _worlds: worlds,
            };

            load_context.set_default_asset(
                LoadedAsset::new(ldtk_project)
                    .with_dependencies(
                        level_backgrounds_meta
                            .iter()
                            .map(|(_, asset_path, _)| asset_path.clone())
                            .collect(),
                    )
                    .with_dependencies(
                        level_file_handles_meta
                            .iter()
                            .map(|(_, asset_path, _)| asset_path.clone())
                            .collect(),
                    )
                    .with_dependencies(
                        // tilesets_meta
                        //     .iter()
                        tilesets
                            .map(|(_, asset_path, _)| asset_path.clone())
                            .collect(),
                    ),
            );

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
