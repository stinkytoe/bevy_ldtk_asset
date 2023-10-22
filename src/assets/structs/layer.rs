use crate::assets::structs::tile::Tile;
use crate::ldtk_json;
use bevy::{prelude::*, utils::thiserror};
use thiserror::Error;

#[derive(Clone, Debug)]
pub(crate) struct Layer {
    pub(crate) _c_height: i64,
    pub(crate) _c_width: i64,
    pub(crate) _grid_size: i64,
    pub(crate) _identifier: String,
    pub(crate) _iid: String,
    pub(crate) _opacity: f32,
    pub(crate) _data: LayerData,
}

#[derive(Clone, Debug)]
pub(crate) enum LayerData {
    IntGrid {},
    Entities {},
    Tiles {
        _tiles: Vec<Tile>,
        _tileset_rel_path: String,
    },
    AutoLayer {},
}

#[derive(Debug, Error)]
pub(crate) enum LayerError {
    #[error("Unknown layer type given: {0}")]
    BadLayerType(String),
    #[error("Missing tileset for tile or auto layer")]
    MissingTileset,
}

impl TryFrom<&ldtk_json::LayerInstance> for Layer {
    type Error = LayerError;

    fn try_from(value: &ldtk_json::LayerInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            _c_height: value.c_hei,
            _c_width: value.c_wid,
            _grid_size: value.grid_size,
            _identifier: value.identifier.clone(),
            _iid: value.iid.clone(),
            _opacity: value.opacity as f32,
            _data: match value.layer_instance_type.as_str() {
                "IntGrid" => LayerData::IntGrid {},
                "Entities" => LayerData::Entities {},
                "Tiles" => LayerData::Tiles {
                    _tiles: value.grid_tiles.iter().map(Tile::from).collect(),
                    _tileset_rel_path: value
                        .tileset_rel_path
                        .as_ref()
                        .ok_or(LayerError::MissingTileset)?
                        .clone(),
                },
                "AutoLayer" => LayerData::AutoLayer {},
                _ => {
                    error!(
                        "Bad layer type from LDtk project: {}",
                        value.layer_instance_type
                    );
                    return Err(LayerError::BadLayerType(value.layer_instance_type.clone()));
                }
            },
        })
    }
}
