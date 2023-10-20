use crate::{ldtk_json, util};
use bevy::{asset::LoadContext, prelude::*};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Level {
    pub(crate) _bg_color: Color,
    pub(crate) _bg_pos: Option<ldtk_json::LevelBackgroundPosition>,
    pub(crate) _bg_rel_path: Option<String>,
    pub(crate) _identifier: String,
    pub(crate) _iid: String,
    pub(crate) _neighbors: Vec<ldtk_json::NeighbourLevel>,
}

// impl From<&ldtk_json::Level> for Level {
//     fn from(value: &ldtk_json::Level) -> Self {
impl Level {
    pub(crate) fn new(value: &ldtk_json::Level, _load_context: &LoadContext) -> Self {
        debug!("Loading level: {}", value.identifier);
        debug!("     with iid: {}", value.iid);
        Level {
            _bg_color: util::get_bevy_color_from_ldtk(&value.bg_color).unwrap_or_else(|e| {
                debug!("Failed to parse level's bg_color: {e}");
                debug!("Using maroon instead");
                Color::MAROON
            }),
            _bg_pos: value.bg_pos.clone(),
            _bg_rel_path: value.bg_rel_path.clone(),
            _identifier: value.identifier.clone(),
            _iid: value.iid.clone(),
            _neighbors: value.neighbours.clone(),
        }
    }
}
