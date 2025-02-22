//! The LDtk layer, represented as a Bevy asset.
//!
//! This is an import of an LDtk
//! [LayerInstance](https://ldtk.io/json/#ldtk-LayerInstanceJson).

use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext, VisitAssetDependencies};
use bevy_image::Image;
use bevy_log::error;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;

use crate::asset_labels::{LayerAssetPath, LevelAssetPath};
use crate::entity::Entity;
use crate::iid::{Iid, IidMap};
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::{ProjectContext, ProjectDefinitionContext};
use crate::tile_instance::TileInstance;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::Uid;
use crate::{ldtk_import_error, Result};

/// A layer instance which contains [Entity] children.
///
/// See [LayerType].
#[derive(Debug, Reflect)]
pub struct EntitiesLayer {
    /// Handles pointing to the [Entity] instances which belong to this layer.
    pub entities: IidMap<Handle<Entity>>,
}

impl EntitiesLayer {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definitions_context: &ProjectDefinitionContext,
    ) -> Result<Self> {
        (value.int_grid_csv.is_empty()
            && value.grid_tiles.is_empty()
            && value.auto_layer_tiles.is_empty()
            && value.tileset_rel_path.is_none()
            && value.tileset_def_uid.is_none())
        .then(|| -> Result<_> {
            let entity_handles = value
                .entity_instances
                .iter()
                .map(|value| {
                    Entity::create_handle_pair(
                        value,
                        layer_asset_path,
                        load_context,
                        project_context,
                        project_definitions_context,
                    )
                })
                .collect::<Result<_>>()?;
            Ok(Self {
                entities: entity_handles,
            })
        })
        .transpose()?
        .ok_or(ldtk_import_error!("Entities layer with Tile data!"))
    }
}

/// A layer which can optionally contain either [TileInstance]s and/or Int Grid values.
///
/// See [LayerType].
#[derive(Debug, Reflect)]
pub struct TilesLayer {
    /// Int grid values for this layer instance.
    ///
    /// This will either be empty (representing a layer with no int grid values), or will be a vec
    /// of length grid_size.x * grid_size.y, with one entry per grid. Entries start at the top left
    /// corner of the grid, going to the right and wrapping to the next row until they reach the
    /// bottom right corner.
    ///
    /// For information on what the int grid value represents, see
    /// [LayerDefinition::int_grid_values].
    pub int_grid: Vec<i64>,
    /// A vec of [TileInstance]s. There is no guaranteed order, except that any tile whose region
    /// overlays a tile before it, is expected to be drawn on top of the previous tile.
    pub tiles: Vec<TileInstance>,
    /// A handle pointing to the [TilesetDefinition] which the tiles will use as their source
    /// image.
    ///
    /// If the [LayerDefinition] does not have a tileset definition assigned, AND the user does not
    /// select one in the tile instance, then this will be `None`. This is technically an invalid
    /// tiles layer and the LDtk GUI will show an error, but the user can still choose to save the
    /// file with none defined.
    pub tileset_definition: Option<Handle<TilesetDefinition>>,
    /// The image that the instances of [TileInstance] in the field [TilesLayer::tiles] are
    /// referring to.
    ///
    /// This will be filled with the default image handle if the JSON is null (a single white
    /// pixel). See [TilesLayer::tileset_definition] for a description on how this can occur.
    pub tileset_image: Handle<Image>,
}

impl TilesLayer {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<TilesLayer> {
        let tiles: &[_] = match value.layer_instance_type.as_str() {
            "Tiles" => &value.grid_tiles,
            "AutoLayer" | "IntGrid" => &value.auto_layer_tiles,
            // Since this was checked in LayerType::new(..), things are screwy if we reach here and
            // therefore we panic!
            _ => unreachable!(),
        };

        value
            .entity_instances
            .is_empty()
            .then(|| -> Result<_> {
                let int_grid = value.int_grid_csv.clone();
                let tiles = tiles.iter().map(TileInstance::new).collect::<Result<_>>()?;
                let tileset_definition = value
                    .tileset_def_uid
                    .and_then(|uid| project_definition_context.tileset_definitions.get(&uid))
                    .cloned();
                let tileset_image = value
                    .tileset_rel_path
                    .as_deref()
                    .map(|ldtk_path| {
                        ldtk_path_to_bevy_path(project_context.project_directory, ldtk_path)
                    })
                    .map(|bevy_path| load_context.load(bevy_path))
                    .unwrap_or_default();

                // Not a parse error, but should be reported to the user.
                if tileset_definition.is_none() {
                    error!(
                        "tileset_definition is None in layer: {}! This is technically \
                        a valid LDtk file, but the editor will show an error message for the layer \
                        with the missing tileset. Please correct inside of LDtk!",
                        value.identifier
                    );
                }

                Ok(Self {
                    int_grid,
                    tiles,
                    tileset_definition,
                    tileset_image,
                })
            })
            .transpose()?
            .ok_or(ldtk_import_error!("Entities layer with Tile data!"))
    }
}

/// The type of the [Layer] instance, defining what it contains.
///
/// In LDtk, layers can be of one of four types: `Entities`, `Tiles`, `IntGrid`, or `AutoLayer`.
#[derive(Debug, Reflect)]
pub enum LayerType {
    /// This layer will contain zero or more child [Entity]s, and no int grid, tiles, or
    /// other visualization fields. Represented by [EntitiesLayer].
    Entities(EntitiesLayer),
    /// This layer will contain a vec of [TileInstance]s. Represented by
    /// [TilesLayer], but with the [TilesLayer::int_grid] field empty.
    Tiles(TilesLayer),
    /// This layer will contain a vec of entries in its [TilesLayer::int_grid] field, and may or
    /// mao not contain tiles in its [TilesLayer::tiles] field.
    IntGrid(TilesLayer),
    /// This layer will contain entries in its [TilesLayer::tiles] field, but its
    /// [TilesLayer::int_grid] field will be empty.
    AutoLayer(TilesLayer),
}

// [LayerType::Entities]
// [LayerType::Tiles]
// [LayerType::IntGrid]
// [LayerType::AutoLayer]
impl LayerType {
    /// Returns `true` if it is a [LayerType::Tiles], [LayerType::IntGrid], or [LayerType::AutoLayer] type,
    /// but `false` for [LayerType::Entities].
    pub fn is_tiles_layer(&self) -> bool {
        !matches!(self, Self::Entities(_))
    }

    /// Returns `true` if it is a [LayerType::Entities] type,
    /// but `false` for [LayerType::Tiles], [LayerType::IntGrid], or [LayerType::AutoLayer].
    pub fn is_entities_layer(&self) -> bool {
        matches!(self, Self::Entities(_))
    }

    /// Returns `Some(TilesLayer)` for [LayerType::Tiles], [LayerType::IntGrid], or [LayerType::AutoLayer],
    /// or `None` for [LayerType::Entities].
    pub fn get_tiles_layer(&self) -> Option<&TilesLayer> {
        match self {
            LayerType::Entities(_) => None,
            LayerType::IntGrid(tiles_layer)
            | LayerType::Tiles(tiles_layer)
            | LayerType::AutoLayer(tiles_layer) => Some(tiles_layer),
        }
    }

    /// Returns `Some(EntitiesLayer)` for [LayerType::Entities],
    /// or `None` for [LayerType::Tiles], [LayerType::IntGrid], or [LayerType::AutoLayer].
    pub fn get_entities_layer(&self) -> Option<&EntitiesLayer> {
        if let Self::Entities(entities_layer) = self {
            Some(entities_layer)
        } else {
            None
        }
    }
}

impl LayerType {
    fn new(
        value: &ldtk::LayerInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<Self> {
        match value.layer_instance_type.as_str() {
            "Entities" => Ok(Self::Entities(EntitiesLayer::new(
                value,
                layer_asset_path,
                load_context,
                project_context,
                project_definition_context,
            )?)),
            "Tiles" | "AutoLayer" | "IntGrid" => Ok(Self::Tiles(TilesLayer::new(
                value,
                load_context,
                project_context,
                project_definition_context,
            )?)),
            unknown => Err(ldtk_import_error!("Unknown layer type! given: {unknown}")),
        }
    }
}

/// An asset representing an [LDtk Layer Instance](https://ldtk.io/json/#ldtk-LayerInstanceJson).
///
/// See [crate::asset_labels] for a description of the label format.
#[derive(Debug, Reflect)]
pub struct Layer {
    /// The size of the logical grid, in two dimensions.
    ///
    /// This is derived from the `c_wid` and `c_hei` LDtk fields.
    pub grid_size: I64Vec2,
    /// How many pixels each grid cell represents.
    ///
    /// This is derived from the `grid_size` LDtk field.
    pub grid_cell_size: i64,
    /// The identifier for the specific [LayerInstance](https://ldtk.io/json/#ldtk-LayerInstanceJson).
    ///
    /// Although not unique on its own, when combined with the containing Level, it will refer to
    /// the specific instance.
    pub identifier: String,
    /// The opacity of this layer instance.
    ///
    /// Represented by a value betwween 0.0 to 1.0, with 1.0 being completely opaque and 0.0 being
    /// completely transparent.
    pub opacity: f64,
    /// The layer type of this specific layer.
    pub layer_type: LayerType,
    /// The Iid. This will likely always be unique, even across projects.
    pub iid: Iid,
    /// A handle pointing to the [LayerDefinition] asset.
    pub layer_definition: Handle<LayerDefinition>,
    /// The [Uid] of the containing level.
    pub level_id: Uid,
    /// Location of this layer in relation to its containing [crate::level::Level].
    ///
    /// Derived from [pdTotalOffsetX](https://ldtk.io/json/#ldtk-LayerInstanceJson;__pxTotalOffsetX)
    /// and [pdTotalOffsetY](https://ldtk.io/json/#ldtk-LayerInstanceJson;__pxTotalOffsetY), and is
    /// the cumulative offset of both the definition and the specific instance.
    pub location: I64Vec2,
    /// Index from 0 to (number of layers - 1), in ascending order. When developing a
    /// visualization, higher index values should be above lower ones.
    pub index: usize,
}

impl Layer {
    pub(crate) fn create_handle_pair(
        value: &ldtk::LayerInstance,
        index: usize,
        level_asset_path: &LevelAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<(Iid, Handle<Self>)> {
        let grid_size: I64Vec2 = (value.c_wid, value.c_hei).into();
        let grid_cell_size = value.grid_size;
        let identifier = value.identifier.clone();
        let layer_asset_path = level_asset_path.to_layer_asset_path(&identifier)?;
        let opacity = value.opacity;
        let layer_type = LayerType::new(
            value,
            &layer_asset_path,
            load_context,
            project_context,
            project_definition_context,
        )?;
        let iid = Iid::from_str(&value.iid)?;
        let layer_definition = project_definition_context
            .layer_definitions
            .get(&value.layer_def_uid)
            .ok_or(ldtk_import_error!(
                "Bad layer definition uid! given: {}",
                value.layer_def_uid
            ))?
            .clone();
        let level_id = value.level_id;
        let location = (value.px_total_offset_x, value.px_total_offset_y).into();

        // Sanity check to guarantee that the int_grid size makes sense
        let int_grid_len = value.int_grid_csv.len();
        let total_grids = (grid_size.x * grid_size.y) as usize;
        if int_grid_len != 0 && int_grid_len != total_grids {
            return Err(ldtk_import_error!(
                "Bad length for int_grid_csv in layer {identifier}! length:{int_grid_len}"
            ));
        }

        let layer = Layer {
            grid_size,
            grid_cell_size,
            identifier,
            opacity,
            layer_type,
            iid,
            layer_definition,
            level_id,
            location,
            index,
        };

        let handle =
            load_context.add_loaded_labeled_asset(layer_asset_path.to_asset_label(), layer.into());

        Ok((iid, handle))
    }
}

impl LdtkAsset for Layer {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<Entity> for Layer {
    fn get_children(&self) -> impl Iterator<Item = &Handle<Entity>> {
        match &self.layer_type {
            LayerType::Entities(entities_layer) => either::Left(entities_layer.entities.values()),
            LayerType::IntGrid(_) | LayerType::Tiles(_) | LayerType::AutoLayer(_) => {
                either::Right([].iter())
            }
        }
    }
}

impl Asset for Layer {}
impl VisitAssetDependencies for Layer {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy_asset::UntypedAssetId)) {
        self.layer_definition.visit_dependencies(visit);

        match &self.layer_type {
            LayerType::Tiles(tiles_layer)
            | LayerType::IntGrid(tiles_layer)
            | LayerType::AutoLayer(tiles_layer) => {
                tiles_layer
                    .tileset_definition
                    .iter()
                    .for_each(|handle| handle.visit_dependencies(visit));
            }
            LayerType::Entities(entities_layer) => {
                entities_layer
                    .entities
                    .values()
                    .for_each(|handle| handle.visit_dependencies(visit));
            }
        }
    }
}
