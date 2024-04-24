use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::thiserror;
use std::path::PathBuf;
use thiserror::Error;

use crate::ldtk;

#[derive(Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub enum LayerType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

#[derive(Component, Debug)]
#[cfg_attr(feature = "enable_reflect", derive(Reflect))]
pub struct LayerComponent {
    // Grid size of layer
    // from c_wid, c_hei
    grid_size: I64Vec2,
    // size in pixels of a grid cell
    // from grid_size
    grid_cell_size: i64,
    identifier: String,
    opacity: f64,
    px_total_offset: I64Vec2,
    tileset_def_uid: Option<i64>,
    tileset_rel_path: Option<PathBuf>,
    // from type
    layer_type: LayerType,
    // NOTE: auto_layer_tiles
    // NOTE: entity_instances
    // NOTE: grid_tiles
    iid: String,
    // NOTE: int_grid_csv
    layer_def_uid: i64,
    level_id: i64,
    override_tileset_uid: Option<i64>,
    px_offset: I64Vec2,
    visible: bool,
}

impl LayerComponent {
    pub fn grid_size(&self) -> I64Vec2 {
        self.grid_size
    }

    pub fn grid_cell_size(&self) -> i64 {
        self.grid_cell_size
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    pub fn px_total_offset(&self) -> I64Vec2 {
        self.px_total_offset
    }

    pub fn tileset_def_uid(&self) -> Option<i64> {
        self.tileset_def_uid
    }

    pub fn tileset_rel_path(&self) -> Option<&PathBuf> {
        self.tileset_rel_path.as_ref()
    }

    pub fn layer_type(&self) -> &LayerType {
        &self.layer_type
    }

    pub fn iid(&self) -> &str {
        self.iid.as_ref()
    }

    pub fn layer_def_uid(&self) -> i64 {
        self.layer_def_uid
    }

    pub fn level_id(&self) -> i64 {
        self.level_id
    }

    pub fn override_tileset_uid(&self) -> Option<i64> {
        self.override_tileset_uid
    }

    pub fn px_offset(&self) -> I64Vec2 {
        self.px_offset
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}

#[derive(Debug, Error)]
pub enum LayerComponentError {
    #[error("Unknown LDtk layer type! {0}")]
    UnknownLayerType(String),
}

impl TryFrom<&ldtk::LayerInstance> for LayerComponent {
    type Error = LayerComponentError;

    fn try_from(value: &ldtk::LayerInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            grid_size: (value.c_wid, value.c_hei).into(),
            grid_cell_size: value.grid_size,
            identifier: value.identifier.clone(),
            opacity: value.opacity,
            px_total_offset: (value.px_total_offset_x, value.px_total_offset_y).into(),
            tileset_def_uid: value.tileset_def_uid,
            tileset_rel_path: value
                .tileset_rel_path
                .as_ref()
                .map(|tileset_rel_path| tileset_rel_path.into()),
            layer_type: match value.layer_instance_type.as_str() {
                "IntGrid" => LayerType::IntGrid,
                "Entities" => LayerType::Entities,
                "Tiles" => LayerType::Tiles,
                "AutoLayer" => LayerType::Autolayer,
                _ => {
                    return Err(LayerComponentError::UnknownLayerType(
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
        })
    }
}