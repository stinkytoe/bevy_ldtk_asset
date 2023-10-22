use crate::ldtk_json;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;

// #[derive(Debug, TypePath, TypeUuid)]
// #[uuid = "4010265b-c425-412f-9fa3-21fc89d1f250"]
#[derive(Asset, TypePath, Debug, Deserialize)]
pub(crate) struct LdtkLevel {
    pub(crate) _level: ldtk_json::Level,
}
