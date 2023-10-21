use crate::ldtk_json;
use bevy::reflect::{TypePath, TypeUuid};

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "4010265b-c425-412f-9fa3-21fc89d1f250"]
pub(crate) struct LdtkLevel {
    pub(crate) _level: ldtk_json::Level,
}
