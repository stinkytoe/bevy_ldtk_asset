use crate::assets::structs::layer::{Layer, LayerError};
use crate::ldtk_json;
use crate::util::{get_bevy_color_from_ldtk, ColorParseError};
use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

#[derive(Clone, Debug)]
pub(crate) struct Level {
    pub(crate) background: Option<LevelBackground>,
    pub(crate) bg_color: Color,
    pub(crate) _neighbors: Vec<ldtk_json::NeighbourLevel>,
    pub(crate) _field_instances: Vec<ldtk_json::FieldInstance>,
    pub(crate) _identifier: String,
    pub(crate) _iid: String,
    pub(crate) _layers: Vec<Layer>,
    pub(crate) px_height: i64,
    pub(crate) px_width: i64,
    pub(crate) _uid: i64,
    pub(crate) world_depth: i64,
    pub(crate) world_x: i64,
    pub(crate) world_y: i64,
}

#[derive(Clone, Debug)]
pub(crate) struct LevelBackground {
    pub(crate) _position: ldtk_json::LevelBackgroundPosition,
    pub(crate) _path: String,
}

#[derive(Debug, Error)]
pub(crate) enum LevelError {
    #[error("Bad Color Value: {0}")]
    BadColor(#[from] ColorParseError),
    #[error("Failed to construct Layer: {0}")]
    BadLayer(#[from] LayerError),
    #[error("bg_pos without bg_rel_path")]
    BgPosWithoutBgRelPath,
    #[error("bg_rel_path without bg_pos")]
    BgRelPathWithoutBgPos,
    #[error("Parsing the internal version of a level, in an external level files context")]
    ExternalLevel,
}

impl TryFrom<&ldtk_json::Level> for Level {
    type Error = LevelError;

    fn try_from(value: &ldtk_json::Level) -> Result<Self, Self::Error> {
        trace!("Loading level: {}", value.identifier);

        Ok(Self {
            background: match (value.bg_pos.as_ref(), value.bg_rel_path.as_ref()) {
                (None, None) => None,
                (None, Some(_)) => {
                    error!("We got a background rel path but no background position!");
                    return Err(LevelError::BgRelPathWithoutBgPos);
                }
                (Some(_), None) => {
                    error!("We got a background position but no background relative path!");
                    return Err(LevelError::BgPosWithoutBgRelPath);
                }
                (Some(bg_pos), Some(bg_rel_path)) => Some(LevelBackground {
                    _position: bg_pos.clone(),
                    _path: bg_rel_path.clone(),
                }),
            },
            bg_color: get_bevy_color_from_ldtk(value.bg_color.clone())?,
            _neighbors: value.neighbours.clone(),
            _field_instances: value.field_instances.clone(),
            _identifier: value.identifier.clone(),
            _iid: value.iid.clone(),
            _layers: value
                .layer_instances
                .as_ref()
                .ok_or(LevelError::ExternalLevel)?
                .iter()
                .map(Layer::try_from)
                .collect::<Result<_, _>>()?,
            px_height: value.px_hei,
            px_width: value.px_wid,
            _uid: value.uid,
            world_depth: value.world_depth,
            world_x: value.world_x,
            world_y: value.world_y,
        })
    }
}
