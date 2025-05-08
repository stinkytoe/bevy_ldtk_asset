#![allow(missing_docs)]

use std::path::Path;
use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_log::trace;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use bevy_tasks::block_on;

use crate::asset_labels::ProjectAssetPath;
use crate::iid::{Iid, IidMap};
use crate::ldtk;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::level::Level;
use crate::project_loader::{ProjectContext, ProjectDefinitionContext, UniqueIidAuditor};
use crate::{Result, ldtk_import_error};

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
    ) -> Result<Self> {
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
    pub(crate) fn create_handle_pair(
        multi_world: bool,
        ldtk_world: &ldtk::World,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        unique_iid_auditor: &mut UniqueIidAuditor,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<(Iid, Handle<Self>)> {
        let identifier = ldtk_world.identifier.clone();
        let world_asset_path = project_asset_path.to_world_asset_path(&identifier)?;

        let iid = Iid::from_str(&ldtk_world.iid)?;
        if multi_world {
            unique_iid_auditor.check(iid)?;
        }

        let world_layout = WorldLayout::new(
            &ldtk_world.world_layout,
            ldtk_world.world_grid_width,
            ldtk_world.world_grid_height,
        )?;

        // TODO: I think we can clean this up and remove some allocations while avoiding the
        // temporary binding. Possibly with Either:: or similar.
        let levels_iter = if project_context.external_levels {
            &ldtk_world
                .levels
                .iter()
                .map(|ldtk_level| -> Result<ldtk::Level> {
                    let external_rel_path =
                        ldtk_level
                            .external_rel_path
                            .as_ref()
                            .ok_or(ldtk_import_error!(
                                "external_rel_path is None when external_levels is true!"
                            ))?;

                    trace!("Attempting to load external level from path: {external_rel_path}");

                    let ldtk_path = Path::new(external_rel_path);
                    let bevy_path =
                        ldtk_path_to_bevy_path(project_context.project_directory, ldtk_path);
                    let bytes = block_on(async { load_context.read_asset_bytes(bevy_path).await })?;
                    let level: ldtk::Level = serde_json::from_slice(&bytes)?;
                    Ok(level)
                })
                .collect::<Result<_>>()?
        } else {
            &ldtk_world.levels
        };

        let levels = levels_iter
            .iter()
            .enumerate()
            .map(|(index, ldtk_level)| {
                Level::create_handle_pair(
                    ldtk_level,
                    index,
                    &world_asset_path,
                    load_context,
                    unique_iid_auditor,
                    project_context,
                    project_definition_context,
                )
            })
            .collect::<Result<_>>()?;

        let world = World {
            identifier,
            iid,
            world_layout,
            levels,
        };

        let handle = load_context.add_labeled_asset(world_asset_path.to_asset_label(), world);

        Ok((iid, handle))
    }
}

impl LdtkAsset for World {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<Level> for World {
    fn get_children(&self) -> impl Iterator<Item = &Handle<Level>> {
        self.levels.values()
    }
}
