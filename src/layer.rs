//! The LDtk layer, represented as a Bevy asset.
//!
//! This is an import of an LDtk
//! [LayerInstance](https://ldtk.io/json/#ldtk-LayerInstanceJson).

use std::str::FromStr;
use std::sync::{Arc, RwLock};

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_image::Image;
use bevy_log::debug;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use futures::future::try_join_all;
use futures::lock::Mutex;

use crate::entity::EntityInstance;
use crate::iid::{Iid, IidMap};
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren};
use crate::ldtk_import_error;
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project::ProjectContext;
use crate::result::LdtkResult;
use crate::tile_instance::TileInstance;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::Uid;

/// A layer instance which contains [Entity] children.
///
/// See [LayerType].
#[derive(Debug, Reflect)]
pub struct EntitiesLayer {
    /// Handles pointing to the [Entity] instances which belong to this layer.
    pub entities: IidMap<Handle<EntityInstance>>,
}

impl EntitiesLayer {
    async fn new(
        entities_layer_json: ldtk::LayerInstance,
        layer_label: &str,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
    ) -> LdtkResult<Self> {
        macro_rules! should_be {
            ($field:ident, $discr:ident) => {
                (entities_layer_json.$field.$discr())
                    .then(|| ())
                    .ok_or(ldtk_import_error!(
                        "Entity Layer with values in {}?",
                        stringify!($field)
                    ))
            };
        }

        should_be!(int_grid_csv, is_empty)?;
        should_be!(grid_tiles, is_empty)?;
        should_be!(tileset_rel_path, is_none)?;
        should_be!(tileset_def_uid, is_none)?;

        let entity_handles_iter = entities_layer_json
            .entity_instances
            .into_iter()
            .map(|value| {
                let project_context = project_context.clone();
                let load_context = load_context.clone();
                async move {
                    let entity = EntityInstance::new(value, project_context).await?;

                    let iid = entity.iid;

                    let entity_label = format!("{layer_label}/{}@{}", entity.identifier, iid);
                    debug!("constructing entity asset: {entity_label}");

                    let handle = load_context
                        .lock()
                        .await
                        .add_labeled_asset(entity_label, entity);

                    LdtkResult::Ok((iid, handle))
                }
            });

        let entities = try_join_all(entity_handles_iter)
            .await?
            .into_iter()
            .collect();

        Ok(Self { entities })
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
    pub tileset_image: Option<Handle<Image>>,
}

impl TilesLayer {
    async fn new(
        layer_instance_json: ldtk::LayerInstance,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
    ) -> LdtkResult<TilesLayer> {
        // Check that the entities array is empty.
        if !layer_instance_json.entity_instances.is_empty() {
            Err(ldtk_import_error!("Entities layer with Tile data!"))?;
        }

        let int_grid = layer_instance_json.int_grid_csv.clone();

        let layer_instance_type = layer_instance_json.layer_instance_type.as_str();

        let tiles = match (
            layer_instance_type,
            layer_instance_json.grid_tiles.len(),
            layer_instance_json.auto_layer_tiles.len(),
        ) {
            // Failure cases.
            ("Tiles", _, a) if a != 0 => {
                Err(ldtk_import_error!("auto layer tiles in a Tiles layer?"))?
            }
            ("AutoLayer" | "IntGrid", g, _) if g != 0 => Err(ldtk_import_error!(
                "grid tiles in a {} layer?",
                layer_instance_type
            ))?,

            // Good cases.
            ("Tiles", _, _) => layer_instance_json.grid_tiles.into_iter(),
            ("AutoLayer" | "IntGrid", _, _) => layer_instance_json.auto_layer_tiles.into_iter(),
            _ => unreachable!(),
        }
        .map(TileInstance::new)
        .collect::<LdtkResult<_>>()?;

        let tileset_definition = layer_instance_json
            .tileset_def_uid
            .map(|uid| {
                project_context
                    .read()?
                    .tileset_definitions
                    .get(&uid)
                    .cloned()
                    .ok_or(ldtk_import_error!(
                        "could not find a tileset_definition with uid {uid}!"
                    ))
            })
            .transpose()?;

        let tileset_image = if let Some(path) = layer_instance_json.tileset_rel_path {
            let path = ldtk_path_to_bevy_path(&project_context.read()?.project_directory, path);

            let handle = load_context.lock().await.load(path);

            Some(handle)
        } else {
            None
        };

        Ok(Self {
            int_grid,
            tiles,
            tileset_definition,
            tileset_image,
        })
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
    async fn new(
        layer_instance_json: ldtk::LayerInstance,
        layer_label: &str,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
    ) -> LdtkResult<Self> {
        match layer_instance_json.layer_instance_type.as_str() {
            "Entities" => Ok(Self::Entities(
                EntitiesLayer::new(
                    layer_instance_json,
                    layer_label,
                    project_context,
                    load_context,
                )
                .await?,
            )),

            "Tiles" | "AutoLayer" | "IntGrid" => Ok(Self::Tiles(
                TilesLayer::new(layer_instance_json, project_context, load_context).await?,
            )),

            unknown => Err(ldtk_import_error!("Unknown layer type! given: {unknown}")),
        }
    }
}

/// An asset representing an [LDtk Layer Instance](https://ldtk.io/json/#ldtk-LayerInstanceJson).
#[derive(Debug, Asset, Reflect)]
pub struct LayerInstance {
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
    /// The Iid. This will likely always be unique, even across projects.
    pub iid: Iid,
    /// The [Uid] of the containing level.
    pub level_id: Uid,
    /// Location of this layer in relation to its containing [crate::level::Level].
    ///
    /// Derived from [pdTotalOffsetX](https://ldtk.io/json/#ldtk-LayerInstanceJson;__pxTotalOffsetX)
    /// and [pdTotalOffsetY](https://ldtk.io/json/#ldtk-LayerInstanceJson;__pxTotalOffsetY), and is
    /// the cumulative offset of both the definition and the specific instance.
    pub location: I64Vec2,
    /// The layer type of this specific layer.
    pub layer_type: LayerType,
    /// A handle pointing to the [LayerDefinition] asset.
    pub layer_definition: Handle<LayerDefinition>,
    /// Index from 0 to (number of layers - 1), in ascending order. When developing a
    /// visualization, higher index values should be above lower ones.
    pub index: usize,
}

impl LayerInstance {
    pub(crate) async fn new(
        layer_instance_json: ldtk::LayerInstance,
        index: usize,
        project_context: Arc<RwLock<ProjectContext>>,
        load_context: Arc<Mutex<&mut LoadContext<'_>>>,
        layer_label: &str,
    ) -> LdtkResult<Self> {
        let identifier = layer_instance_json.identifier.clone();

        let grid_size: I64Vec2 = (layer_instance_json.c_wid, layer_instance_json.c_hei).into();

        let grid_cell_size = layer_instance_json.grid_size;

        let opacity = layer_instance_json.opacity;

        let iid = Iid::from_str(&layer_instance_json.iid)?;

        let level_id = layer_instance_json.level_id;

        let location = (
            layer_instance_json.px_total_offset_x,
            layer_instance_json.px_total_offset_y,
        )
            .into();

        let layer_definition = project_context
            .read()?
            .layer_definitions
            .get(&layer_instance_json.layer_def_uid)
            .ok_or(ldtk_import_error!(
                "Bad layer definition uid! given: {}",
                layer_instance_json.layer_def_uid
            ))?
            .clone();

        // Sanity check to guarantee that the int_grid size makes sense
        let int_grid_len = layer_instance_json.int_grid_csv.len();
        let total_grids = (grid_size.x * grid_size.y) as usize;
        if int_grid_len != 0 && int_grid_len != total_grids {
            return Err(ldtk_import_error!(
                "Bad length for int_grid_csv in layer {identifier}! length:{int_grid_len}"
            ));
        }

        let layer_type = LayerType::new(
            layer_instance_json,
            layer_label,
            project_context,
            load_context,
        )
        .await?;

        Ok(Self {
            grid_size,
            grid_cell_size,
            identifier,
            opacity,
            iid,
            level_id,
            location,
            layer_definition,
            layer_type,
            index,
        })
    }
}

impl LdtkAsset for LayerInstance {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<EntityInstance> for LayerInstance {
    fn get_children(&self) -> impl Iterator<Item = &Handle<EntityInstance>> {
        match &self.layer_type {
            LayerType::Entities(entities_layer) => either::Left(entities_layer.entities.values()),
            LayerType::IntGrid(_) | LayerType::Tiles(_) | LayerType::AutoLayer(_) => {
                either::Right([].iter())
            }
        }
    }
}
