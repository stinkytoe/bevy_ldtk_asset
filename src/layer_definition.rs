//! The LDtk definition for a layer.
//!
//! This is used by LDtk for creating new layer instances.
//!
//! This is an import of an LDtk
//! [LDtk: LayerDefinition](https://ldtk.io/json/#ldtk-LayerDefJson)

use bevy_asset::{Asset, Handle};
use bevy_color::Color;
use bevy_math::{DVec2, I64Vec2};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::color::bevy_color_from_ldtk_string;
use crate::ldtk;
use crate::ldtk_import_error;
use crate::result::Result;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};

/// Represents the type of layer.
#[derive(Debug, Reflect)]
#[allow(missing_docs)]
pub enum LayerDefinitionType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

impl LayerDefinitionType {
    pub(crate) fn new(ldtk_type: &str) -> Result<LayerDefinitionType> {
        Ok(match ldtk_type {
            "IntGrid" => LayerDefinitionType::IntGrid,
            "Entities" => LayerDefinitionType::Entities,
            "Tiles" => LayerDefinitionType::Tiles,
            "AutoLayer" => LayerDefinitionType::Autolayer,
            _ => {
                return Err(ldtk_import_error!(
                    "Could not build LayerDefinitionType from input! given: {ldtk_type}"
                ));
            }
        })
    }
}

#[derive(Asset, Debug, Reflect)]
/// An element of [LayerDefinition] is used by LDtk when constructing new layers
/// and levels, and represents defaults which they are initialized to.
pub struct LayerDefinition {
    /// Type of layer. See [LayerDefinitionType].
    pub layer_definition_type: LayerDefinitionType,
    /// The source IntGrid layer Uid, used for marking.
    ///
    /// Only for AutoLayer types.
    pub auto_source_layer_def_uid: Option<Uid>,
    /// Display Opacity [0.0 to 1.0]
    pub display_opacity: f64,
    /// Width and height of the grid in pixels.
    pub grid_cell_size: i64,
    /// User defined unique identifier.
    pub identifier: String,
    /// For IntGrid types, defines extra metadata for a given IntGrid value.
    pub int_grid_values: UidMap<IntGridValue>,
    /// Goup Information for IntGridValues.
    pub int_grid_values_groups: UidMap<IntGridValuesGroup>,
    /// Parallax factor. Each value (x, y) ranging from -1.0 to 1.0 .
    ///
    /// This affects the scrolling speed of this layer.
    ///
    /// From the `parallaxFactorX` and `parallaxFactorX` LDtk JSON fields.
    pub parallax_factor: DVec2,
    /// Parallax also effects Scaling.
    pub parallax_scaling: bool,
    /// Offset in relation to the level origin.
    ///
    /// From the `pxOffsetX` and `pxOffsetY` LDtk JSON fields.
    pub offset: I64Vec2,
    /// Handle, if any, to the [TilesetDefinition] which is assigned to this
    /// layer.
    ///
    /// This can either be because this is an IntGrid layer with no tiles,
    /// an Entity layer, or any layer with no default tileset assigned.
    pub tileset_definition: Option<Handle<TilesetDefinition>>,
}

impl LayerDefinition {
    pub(crate) async fn new(
        value: ldtk::LayerDefinition,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<Self> {
        let identifier = value.identifier;
        let layer_definition_type = LayerDefinitionType::new(&value.layer_definition_type)?;
        let auto_source_layer_def_uid = value.auto_source_layer_def_uid;
        let display_opacity = value.display_opacity;
        let grid_cell_size = value.grid_size;
        let int_grid_values = value
            .int_grid_values
            .into_iter()
            .map(|value| Ok((value.value, IntGridValue::new(value, tileset_definitions)?)))
            .collect::<Result<_>>()?;
        let int_grid_values_groups = value
            .int_grid_values_groups
            .into_iter()
            .map(|value| Ok((value.uid, IntGridValuesGroup::new(value)?)))
            .collect::<Result<_>>()?;
        let parallax_factor = (value.parallax_factor_x, value.parallax_factor_y).into();
        let parallax_scaling = value.parallax_scaling;
        let offset = (value.px_offset_x, value.px_offset_y).into();
        let tileset_definition = value
            .tileset_def_uid
            .map(|tileset_def_uid| {
                tileset_definitions
                    .get(&tileset_def_uid)
                    .ok_or(ldtk_import_error!("Bad uid! {tileset_def_uid}"))
            })
            .transpose()?
            .cloned();

        Ok(LayerDefinition {
            layer_definition_type,
            auto_source_layer_def_uid,
            display_opacity,
            grid_cell_size,
            identifier,
            int_grid_values,
            int_grid_values_groups,
            parallax_factor,
            parallax_scaling,
            offset,
            tileset_definition,
        })
    }
}

/// An int grid value. IntGrid type layers will contain an array of integers,
/// whose value will correlate with a value in one of these elements.
#[derive(Clone, Debug, Reflect)]
#[allow(missing_docs)]
pub struct IntGridValue {
    pub color: Color,
    pub group_uid: Uid,
    pub identifier: Option<String>,
    pub tile: Option<TilesetRectangle>,
    pub value: i64,
}

impl IntGridValue {
    pub(crate) fn new(
        int_grid_value: ldtk::IntGridValueDefinition,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<Self> {
        let color = bevy_color_from_ldtk_string(&int_grid_value.color)?;
        let group_uid = int_grid_value.group_uid;
        let identifier = int_grid_value.identifier;
        let tile = int_grid_value
            .tile
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;

        let value = int_grid_value.value;

        Ok(Self {
            color,
            group_uid,
            identifier,
            tile,
            value,
        })
    }
}

/// ItGridValues can be collected into groups. The [IntGridValue::group_uid]
/// field will act as the key into this map to get metadata about their
/// associated group.
pub type IntGridValuesGroups = HashMap<i64, IntGridValuesGroup>;

/// Metadata for a given IntGrid group.
#[derive(Debug, Reflect)]
#[allow(missing_docs)]
pub struct IntGridValuesGroup {
    pub color: Option<Color>,
    pub identifier: Option<String>,
}

impl IntGridValuesGroup {
    pub(crate) fn new(value: ldtk::IntGridValueGroupDefinition) -> Result<Self> {
        let color = value
            .color
            .as_deref()
            .map(bevy_color_from_ldtk_string)
            .transpose()?;
        let identifier = value.identifier;

        Ok(Self { color, identifier })
    }
}
