use crate::{
    ldtk_json,
    util::{self, ColorParseError},
};
use bevy::prelude::*;

/// Representation of an ldtk IntGridValueDefinition.
/// This defines the field associated with an int_grid_csv.
/// See [ldtk_json::IntGridValueDefinition]
#[derive(Debug)]
pub struct IntGridValue<'a> {
    pub(crate) value: &'a ldtk_json::IntGridValueDefinition,
}

impl IntGridValue<'_> {
    /// Returns the optional identifier for this int grid value definition.
    /// LDtk treats this is an optional field, so we will too.
    pub fn identifier(&self) -> &Option<String> {
        &self.value.identifier
    }
    /// Returns an Ok(Color) Bevy color object, or a ColorParseError on error.
    pub fn color(&self) -> Result<Color, ColorParseError> {
        util::get_bevy_color_from_ldtk(&self.value.color)
    }

    /// Returns the group uid associated with this int grid definition
    pub fn group_uid(&self) -> i64 {
        self.value.group_uid
    }

    // todo: implement TilesetRectangle then implement this
    // pub fn tile(&self) -> Option<

    /// The actual value of this int grid. This is what will be references in an int_grid_csv
    pub fn value(&self) -> i64 {
        self.value.value
    }
}
