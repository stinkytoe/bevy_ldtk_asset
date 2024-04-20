use bevy::prelude::*;
use bevy::utils::thiserror;
use thiserror::Error;

use crate::ldtk::{self};
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Debug, Default)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub struct ReferenceToAnEntityInstance {
    entity_iid: String,
    layer_iid: String,
    level_iid: String,
    world_iid: String,
}

impl ReferenceToAnEntityInstance {
    pub fn entity_iid(&self) -> &str {
        self.entity_iid.as_ref()
    }

    pub fn layer_iid(&self) -> &str {
        self.layer_iid.as_ref()
    }

    pub fn level_iid(&self) -> &str {
        self.level_iid.as_ref()
    }

    pub fn world_iid(&self) -> &str {
        self.world_iid.as_ref()
    }
}

impl From<&ldtk::ReferenceToAnEntityInstance> for ReferenceToAnEntityInstance {
    fn from(value: &ldtk::ReferenceToAnEntityInstance) -> Self {
        Self {
            entity_iid: value.entity_iid.clone(),
            layer_iid: value.layer_iid.clone(),
            level_iid: value.level_iid.clone(),
            world_iid: value.world_iid.clone(),
        }
    }
}

#[derive(Debug, Error)]
pub enum FieldInstanceValueError {
    #[error("Given unknown field instance type from LDtk project! {0}")]
    UnknownFieldInstanceType(String),
    #[error("value is None!")]
    ValueIsNone,
    #[error("Unable to parse as an integer!")]
    BadInt,
    #[error("Unable to parse as a float!")]
    BadFloat,
    #[error("Unable to parse as a string?!")]
    BadString,
}

#[derive(Debug)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub enum FieldInstanceValue {
    Int(i64),
    Float(f64),
    String(String),
    // Multilines(String),
    // Enum(String),
    // Bool(bool),
    // // from GridPoint
    // GridPoint(U64Vec2),
    // TilesetRect(TilesetRectangle),
    // EntityReferenceInfo(ReferenceToAnEntityInstance),
    // Array(Vec<FieldInstanceValue>),
}

#[derive(Debug)]
#[cfg_attr(feature = "enable_typepath", derive(TypePath))]
pub struct FieldInstance {
    identifier: String,
    tile: Option<TilesetRectangle>,
    value: FieldInstanceValue,
    def_uid: i64,
}

impl FieldInstance {
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    pub fn tile(&self) -> Option<&TilesetRectangle> {
        self.tile.as_ref()
    }

    pub fn value(&self) -> &FieldInstanceValue {
        &self.value
    }

    pub fn def_uid(&self) -> i64 {
        self.def_uid
    }
}

impl TryFrom<&ldtk::FieldInstance> for FieldInstance {
    type Error = FieldInstanceValueError;

    fn try_from(value: &ldtk::FieldInstance) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::from),
            value: {
                let field_instance_type = value.field_instance_type.as_str();
                let value = value
                    .value
                    .as_ref()
                    .ok_or(FieldInstanceValueError::ValueIsNone)?;
                match field_instance_type {
                    "Int" => FieldInstanceValue::Int(
                        value.as_i64().ok_or(FieldInstanceValueError::BadInt)?,
                    ),
                    "Float" => FieldInstanceValue::Float(
                        value.as_f64().ok_or(FieldInstanceValueError::BadFloat)?,
                    ),
                    "String" => FieldInstanceValue::String(
                        value
                            .as_str()
                            .ok_or(FieldInstanceValueError::BadString)?
                            .to_owned(),
                    ),
                    // TODO: finish me!
                    _ => {
                        return Err(FieldInstanceValueError::UnknownFieldInstanceType(
                            field_instance_type.to_owned(),
                        ))
                    }
                }
            },
            def_uid: value.def_uid,
        })
    }
}
