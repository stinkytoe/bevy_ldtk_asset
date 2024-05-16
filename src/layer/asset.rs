use bevy::math::I64Vec2;
use bevy::prelude::*;
use std::path::PathBuf;
use thiserror::Error;

use crate::ldtk;
use crate::project::ProjectAsset;

#[derive(Debug, Error)]
pub enum LayerTypeError {
    #[error("Unknown LDtk layer type! {0}")]
    UnknownLayerType(String),
}

#[derive(Debug, Reflect)]
pub enum LayerType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

#[derive(Asset, Debug, Reflect)]
pub struct LayerAsset {
    // Grid size of layer
    // from c_wid, c_hei
    pub grid_size: I64Vec2,
    // size in pixels of a grid cell
    // from grid_size
    pub grid_cell_size: i64,
    pub identifier: String,
    pub opacity: f64,
    pub px_total_offset: I64Vec2,
    pub tileset_def_uid: Option<i64>,
    pub tileset_rel_path: Option<PathBuf>,
    // from type
    pub layer_type: LayerType,
    pub iid: String,
    pub layer_def_uid: i64,
    pub level_id: i64,
    pub override_tileset_uid: Option<i64>,
    pub px_offset: I64Vec2,
    pub visible: bool,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
}

impl LayerAsset {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        project: Handle<ProjectAsset>,
    ) -> Result<Self, LayerTypeError> {
        Ok(Self {
            grid_size: (value.c_wid, value.c_hei).into(),
            grid_cell_size: value.grid_size,
            identifier: value.identifier.clone(),
            opacity: value.opacity,
            px_total_offset: (value.px_total_offset_x, value.px_total_offset_y).into(),
            tileset_def_uid: value.tileset_def_uid,
            tileset_rel_path: value.tileset_rel_path.as_ref().map(PathBuf::from),
            layer_type: match value.layer_instance_type.as_str() {
                "IntGrid" => LayerType::IntGrid,
                "Entities" => LayerType::Entities,
                "Tiles" => LayerType::Tiles,
                "AutoLayer" => LayerType::Autolayer,
                _ => {
                    return Err(LayerTypeError::UnknownLayerType(
                        value.layer_instance_type.clone(),
                    ))
                }
            },
            iid: value.iid.clone(),
            layer_def_uid: value.layer_def_uid,
            level_id: value.level_id,
            override_tileset_uid: value.override_tileset_uid,
            px_offset: (value.px_offset_x, value.px_offset_y).into(),
            visible: value.visible,
            project,
        })
    }
}
