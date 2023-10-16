use crate::level::Level;
use bevy::reflect::{TypePath, TypeUuid};

#[derive(Debug, TypePath, TypeUuid)]
#[uuid = "4010265b-c425-412f-9fa3-21fc89d1f250"]
pub struct LdtkLevel {
    pub level: Level,
}
