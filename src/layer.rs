use std::str::FromStr;

use bevy::asset::{Asset, Handle, LoadContext};
use bevy::math::{I64Vec2, Vec2};
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

use crate::entity::Entity;
use crate::iid::{Iid, IidMap};
use crate::label::{LayerAssetPath, LevelAssetPath};
use crate::ldtk;
use crate::ldtk_asset_traits::{HasChildren, LdtkAsset};
use crate::project_loader::ProjectContext;
use crate::tile_instance::TileInstance;

#[derive(Debug, Reflect)]
pub struct EntitiesLayer {
    entity_handles: IidMap<Handle<Entity>>,
}

#[derive(Debug, Reflect)]
pub struct TilesLayer {
    int_grid: Vec<i64>,
    tiles: Vec<TileInstance>,
}

#[derive(Debug, Reflect)]
pub enum LayerType {
    Entities(EntitiesLayer),
    IntGrid(TilesLayer),
    Tiles(TilesLayer),
    AutoLayer(TilesLayer),
}

impl LayerType {
    fn new(
        ldtk_layer_instance: &ldtk::LayerInstance,
        layer_asset_label: &LayerAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
    ) -> crate::Result<Self> {
        match (
            ldtk_layer_instance.layer_instance_type.as_str(),
            ldtk_layer_instance.entity_instances.len(),
            ldtk_layer_instance.grid_tiles.len(),
            ldtk_layer_instance.auto_layer_tiles.len(),
            ldtk_layer_instance.int_grid_csv.len(),
        ) {
            ("Entities", _, g, a, i) if g != 0 || a != 0 || i != 0 => {
                Err(crate::Error::LdtkImportError(
                    "Entity layer type can only have entity instance data!".to_string(),
                ))
            }
            ("Entities", _, _, _, _) => Ok(Self::Entities(EntitiesLayer {
                entity_handles: ldtk_layer_instance
                    .entity_instances
                    .iter()
                    .map(|ldtk_entity_instance| {
                        Entity::create_handle_pair(
                            ldtk_entity_instance,
                            layer_asset_label,
                            load_context,
                            project_context,
                        )
                    })
                    .collect::<crate::Result<_>>()?,
            })),

            ("Tiles", e, _, a, i) if e != 0 || a != 0 || i != 0 => {
                Err(crate::Error::LdtkImportError(
                    "Tiles layer type can only have grid tile data!".to_string(),
                ))
            }
            ("Tiles", _, _, _, _) => Ok(Self::Tiles(TilesLayer {
                int_grid: ldtk_layer_instance.int_grid_csv.to_vec(),
                tiles: ldtk_layer_instance
                    .grid_tiles
                    .iter()
                    .map(TileInstance::new)
                    .collect::<Result<_, _>>()?,
            })),

            ("AutoLayer", e, g, _, _) | ("IntGrid", e, g, _, _) if e != 0 || g != 0 => {
                Err(crate::Error::LdtkImportError(
                    "AutoLayer/IntGrid layer types \
                        can only have auto layer tile with optional int_grid data!"
                        .to_string(),
                ))
            }
            ("AutoLayer", _, _, _, _) | ("IntGrid", _, _, _, _) => Ok(Self::Tiles(TilesLayer {
                int_grid: ldtk_layer_instance.int_grid_csv.to_vec(),
                tiles: ldtk_layer_instance
                    .auto_layer_tiles
                    .iter()
                    .map(TileInstance::new)
                    .collect::<Result<_, _>>()?,
            })),

            (unknown, _, _, _, _) => Err(crate::Error::LdtkImportError(format!(
                "Unknown layer type! given: {unknown}"
            ))),
        }
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct Layer {
    pub grid_size: I64Vec2,
    pub grid_cell_size: i64,
    pub identifier: String,
    pub opacity: f64,
    pub total_offset: Vec2,
    pub tileset_def_uid: Option<i64>,
    pub tileset_rel_path: Option<String>,
    pub layer_type: LayerType,
    pub iid: Iid,
    pub layer_def_uid: i64,
    pub level_id: i64,
    pub location: Vec2,
    pub index: usize,
    // TODO: hackhackhack! This is meant to always remain empty, so that we have something
    // to generate a Values iterator with if the layer_type is such that it doesn't contain
    // entities!
    pub stub: IidMap<Handle<Entity>>,
}

impl Layer {
    pub(crate) fn create_handle_pair(
        value: &ldtk::LayerInstance,
        index: usize,
        level_asset_path: &LevelAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
    ) -> crate::Result<(Iid, Handle<Self>)> {
        let grid_size = (value.c_wid, value.c_hei).into();
        let grid_cell_size = value.grid_size;
        let identifier = value.identifier.clone();
        let layer_asset_path = level_asset_path.to_layer_asset_path(&identifier);
        let opacity = value.opacity;
        let total_offset = (
            value.px_total_offset_x as f32,
            -value.px_total_offset_y as f32,
        )
            .into();
        let tileset_def_uid = value.tileset_def_uid;
        let tileset_rel_path = value.tileset_rel_path.clone();
        let layer_type = LayerType::new(value, &layer_asset_path, load_context, project_context)?;
        let iid = Iid::from_str(&value.iid)?;
        let layer_def_uid = value.layer_def_uid;
        let level_id = value.level_id;
        let location = (value.px_offset_x as f32, -value.px_total_offset_y as f32).into();
        let stub = HashMap::default();

        let layer = Layer {
            grid_size,
            grid_cell_size,
            identifier,
            opacity,
            total_offset,
            tileset_def_uid,
            tileset_rel_path,
            layer_type,
            iid,
            layer_def_uid,
            level_id,
            location,
            index,
            stub,
        }
        .into();

        let handle =
            load_context.add_loaded_labeled_asset(layer_asset_path.to_asset_label(), layer);

        Ok((iid, handle))
    }
}

impl LdtkAsset for Layer {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn iid(&self) -> Iid {
        self.iid
    }
}

impl HasChildren for Layer {
    type Child = Entity;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        match &self.layer_type {
            LayerType::Entities(entities) => entities.entity_handles.values(),
            LayerType::IntGrid(_) | LayerType::Tiles(_) | LayerType::AutoLayer(_) => {
                self.stub.values()
            }
        }
    }
}
