#![allow(missing_docs)]

use std::str::FromStr;
use std::sync::{Arc, RwLock};

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_log::debug;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use either::Either;
use futures::future::try_join_all;
use futures::lock::Mutex;

use crate::iid::{Iid, IidMap};
use crate::ldtk;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren};
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::level::Level;
use crate::project::ProjectContext;
use crate::result::LdtkResult;

/// The layout of the world's levels.
///
/// See [world layout](https://ldtk.io/docs/general/world/#layouts) in the
/// LDtk documentation for a description.
#[derive(Debug, Reflect)]
pub enum WorldLayout {
    /// Can be placed anywhere.
    Free,
    /// Worlds are freely placed, but constrained to a grid.
    GridVania(I64Vec2),
    /// Levels are layed out in order horizontally.
    ///
    /// The levels [crate::level::Level::index] field will be used to determine
    /// the order. The exact layout (spacing, alignment, etc) is not defined in
    /// this plugin and is left up to the user.
    LinearHorizontal,
    /// Same as [WorldLayout::LinearHorizontal], but aligned vertically.
    LinearVertical,
}

impl WorldLayout {
    fn new(
        layout: &Option<ldtk::WorldLayout>,
        world_grid_width: i64,
        world_grid_height: i64,
    ) -> LdtkResult<Self> {
        match layout {
            Some(ldtk::WorldLayout::GridVania) => Ok(Self::GridVania(
                (world_grid_width, world_grid_height).into(),
            )),
            Some(ldtk::WorldLayout::Free) => Ok(Self::Free),
            Some(ldtk::WorldLayout::LinearHorizontal) => Ok(Self::LinearHorizontal),
            Some(ldtk::WorldLayout::LinearVertical) => Ok(Self::LinearVertical),
            // TODO: WTF is up with this?
            None => todo!(),
        }
    }
}

/// A single world instance.
#[allow(missing_docs)]
#[derive(Debug, Asset, Reflect)]
pub struct World {
    pub identifier: String,
    pub iid: Iid,
    pub world_layout: WorldLayout,
    pub levels: IidMap<Handle<Level>>,
}

impl World {
    pub(crate) async fn new(
        world_json: ldtk::World,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
        world_label: &str,
    ) -> LdtkResult<Self> {
        let identifier = world_json.identifier;

        let iid = Iid::from_str(&world_json.iid)?;

        let world_layout = WorldLayout::new(
            &world_json.world_layout,
            world_json.world_grid_width,
            world_json.world_grid_height,
        )?;

        let external_levels = project_context.read()?.external_levels;

        let levels_json = if external_levels {
            let levels_json_iter =
                world_json
                    .levels
                    .into_iter()
                    .enumerate()
                    .map(|(index, level_json)| {
                        let project_context = project_context.clone();
                        let load_context = load_context.clone();

                        async move {
                            let ldtk_path =
                                level_json.external_rel_path.ok_or(ldtk_import_error!(
                                    "external_rel_path is `None` in an external_levels project?"
                                ))?;

                            let path = ldtk_path_to_bevy_path(
                                &project_context.read()?.project_directory,
                                ldtk_path,
                            );

                            let external_level_json =
                                load_context.lock().await.read_asset_bytes(path).await?;
                            let external_level_json: ldtk::Level =
                                serde_json::from_slice(&external_level_json)?;

                            LdtkResult::Ok((index, external_level_json))
                        }
                    });

            let levels_json_iter = try_join_all(levels_json_iter).await?.into_iter();
            Either::Left(levels_json_iter)
        } else {
            let levels_json_iter = world_json.levels.into_iter().enumerate();
            Either::Right(levels_json_iter)
        };

        let levels_iter = levels_json.into_iter().map(|(index, level_json)| {
            let load_context = load_context.clone();
            let project_context = project_context.clone();

            async move {
                let level_label = format!("{world_label}/{}", level_json.identifier);
                debug!("constructing level asset: {level_label}");
                let level = Level::new(
                    level_json,
                    index,
                    project_context,
                    load_context.clone(),
                    &level_label,
                )
                .await?;
                let iid = level.iid;
                let handle = load_context
                    .lock()
                    .await
                    .add_labeled_asset(level_label, level);
                LdtkResult::Ok((iid, handle))
            }
        });

        let levels = try_join_all(levels_iter).await?.into_iter().collect();

        Ok(Self {
            identifier,
            iid,
            world_layout,
            levels,
        })
    }
}

impl LdtkAsset for World {
    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<Level> for World {
    fn get_children(&self) -> impl Iterator<Item = &Handle<Level>> {
        self.levels.values()
    }
}
