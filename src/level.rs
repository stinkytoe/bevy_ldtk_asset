use crate::{ldtk_json, util};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Level {
    pub bg_color: Color,
    pub bg_pos: Option<ldtk_json::LevelBackgroundPosition>,
    pub bg_rel_path: Option<String>,
    pub identifier: String,
    pub iid: String,
    pub neighbors: Vec<ldtk_json::NeighbourLevel>,
}

impl From<&ldtk_json::Level> for Level {
    fn from(value: &ldtk_json::Level) -> Self {
        Level {
            bg_color: util::get_bevy_color_from_ldtk(&value.bg_color).unwrap_or_else(|e| {
                debug!("Failed to parse level's bg_color: {e}");
                debug!("Using maroon instead");
                Color::MAROON
            }),
            bg_pos: value.bg_pos.clone(),
            bg_rel_path: value.bg_rel_path.clone(),
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            neighbors: value.neighbours.clone(),
        }
    }
}
