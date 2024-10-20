use std::str::FromStr;

use bevy::asset::{Asset, Handle};
use bevy::math::{I64Vec2, Vec2};
use bevy::reflect::Reflect;

use crate::entity::Entity;
use crate::iid::Iid;
use crate::ldtk;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::tile_instance::TileInstance;

#[derive(Debug, Reflect)]
pub struct EntitiesLayer {
    entity_handles: Vec<Handle<Entity>>,
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
        layer_type: &str,
        //layer_path: &str,
        entities: &[ldtk::EntityInstance],
        grid_tiles: &[ldtk::TileInstance],
        auto_layer_tiles: &[ldtk::TileInstance],
        int_grid_csv: &[i64],
    ) -> crate::Result<Self> {
        match (
            layer_type,
            entities.len(),
            grid_tiles.len(),
            auto_layer_tiles.len(),
            int_grid_csv.len(),
        ) {
            ("Entities", _, g, a, i) if g != 0 || a != 0 || i != 0 => {
                Err(crate::Error::LdtkImportError("Entity layer type can only have entity instance data!".to_string()))
            }
            ("Entities", _, _, _, _) => Ok(Self::Entities(EntitiesLayer {
                entity_handles: Vec::default(),
            })),
            ("Tiles", e, _, a, i) if e != 0 || a != 0 || i != 0 => Err(crate::Error::LdtkImportError(
                "Tiles layer type can only have grid tile data!".to_string(),
            )),
            ("Tiles", _, _, _, _) => Ok(Self::Tiles(TilesLayer {
                int_grid: int_grid_csv.to_vec(),
                tiles: grid_tiles
                    .iter()
                    .map(TileInstance::new)
                    .collect::<Result<_, _>>()?,
            })),
            ("AutoLayer", e, g, _, _) | ("IntGrid", e, g, _, _) if e != 0 || g != 0 => {
                Err(crate::Error::LdtkImportError("AutoLayer/IntGrid layer types can only have auto layer tile with optional int_grid data!".to_string()))
            }
            ("AutoLayer", _, _, _, _) | ("IntGrid", _, _, _, _) => Ok(Self::Tiles(TilesLayer {
                int_grid: int_grid_csv.to_vec(),
                tiles: auto_layer_tiles
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
    //pub parent_path: String,
}

impl Layer {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        index: usize,
        //parent_path: &str,
        entities: Vec<Entity>,
    ) -> crate::Result<Self> {
        let grid_size = (value.c_wid, value.c_hei).into();
        let grid_cell_size = value.grid_size;
        let identifier = value.identifier.clone();
        let opacity = value.opacity;
        let total_offset = (
            value.px_total_offset_x as f32,
            -value.px_total_offset_y as f32,
        )
            .into();
        let tileset_def_uid = value.tileset_def_uid;
        let tileset_rel_path = value.tileset_rel_path.clone();
        let layer_type = LayerType::new(
            &value.layer_instance_type,
            //&format!("{parent_path}/{}", value.identifier),
            &value.entity_instances,
            &value.grid_tiles,
            &value.auto_layer_tiles,
            &value.int_grid_csv,
        )?;
        let iid = Iid::from_str(&value.iid)?;
        let layer_def_uid = value.layer_def_uid;
        let level_id = value.level_id;
        let location = (value.px_offset_x as f32, -value.px_total_offset_y as f32).into();
        //let parent_path = parent_path.to_string();

        Ok(Layer {
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
            //parent_path,
        })
    }
}

impl LdtkAsset for Layer {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    //fn parent_path(&self) -> bevy::asset::AssetPath {
    //    AssetPath::from(&self.parent_path)
    //}
    //
    //fn children_paths(&self) -> impl Iterator<Item = bevy::asset::AssetPath> {
    //    if let LayerType::Entities(entities_layer) = &self.layer_type {
    //        return entities_layer
    //            .entity_paths
    //            .iter()
    //            .map(AssetPath::from)
    //            .collect::<Vec<_>>()
    //            .into_iter();
    //    } else {
    //        vec![].into_iter()
    //    }
    //}
}
