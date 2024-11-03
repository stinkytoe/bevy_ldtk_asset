use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_math::DVec2;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;

use crate::color::bevy_color_from_ldtk_string;
use crate::label::ProjectAssetPath;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::{ldtk, Error, Result};

#[derive(Debug, Reflect)]
pub enum LayerDefinitionType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

impl LayerDefinitionType {
    pub fn new(ldtk_type: &str) -> Result<LayerDefinitionType> {
        Ok(match ldtk_type {
            "IntGrid" => LayerDefinitionType::IntGrid,
            "Entities" => LayerDefinitionType::Entities,
            "Tiles" => LayerDefinitionType::Tiles,
            "AutoLayer" => LayerDefinitionType::Autolayer,
            _ => {
                return Err(Error::LdtkImportError(format!(
                    "Could not build LayerDefinitionType from input! given: {ldtk_type}"
                )))
            }
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct LayerDefinition {
    pub layer_definition_type: LayerDefinitionType,
    pub auto_source_layer_def_uid: Option<i64>,
    pub display_opacity: f64,
    pub grid_cell_size: i64,
    pub identifier: String,
    pub int_grid_values: Vec<IntGridValue>,
    pub int_grid_values_groups: Vec<IntGridValuesGroup>,
    pub parallax_factor: DVec2,
    pub parallax_scaling: bool,
    pub offset: Vec2,
    pub tileset_definition: Option<Handle<TilesetDefinition>>,
}

impl LayerDefinition {
    pub(crate) fn create_handle_pair(
        value: &ldtk::LayerDefinition,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<(Uid, Handle<Self>)> {
        let identifier = value.identifier.clone();
        let uid = value.uid;

        let layer_definition_asset_path =
            project_asset_path.to_layer_definition_asset_path(&identifier);

        let layer_definition_type = LayerDefinitionType::new(&value.layer_definition_type)?;
        let auto_source_layer_def_uid = value.auto_source_layer_def_uid;
        let display_opacity = value.display_opacity;
        let grid_cell_size = value.grid_size;
        let int_grid_values = value
            .int_grid_values
            .iter()
            .map(IntGridValue::new)
            .collect::<Result<_>>()?;
        let int_grid_values_groups = value
            .int_grid_values_groups
            .iter()
            .map(IntGridValuesGroup::new)
            .collect::<Result<_>>()?;
        let parallax_factor = (value.parallax_factor_x, value.parallax_factor_y).into();
        let parallax_scaling = value.parallax_scaling;
        let offset = (value.px_offset_x as f32, value.px_offset_y as f32).into();
        let tileset_definition = value
            .tileset_def_uid
            .map(|tileset_def_uid| {
                tileset_definitions
                    .get(&tileset_def_uid)
                    .ok_or(Error::LdtkImportError(format!(
                        "Bad uid! {tileset_def_uid}"
                    )))
            })
            .transpose()?
            .cloned();

        let layer_definition = LayerDefinition {
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
        }
        .into();

        let handle = load_context.add_loaded_labeled_asset(
            layer_definition_asset_path.to_asset_label(),
            layer_definition,
        );

        Ok((uid, handle))
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct IntGridValue {
    pub color: Color,
    pub group_uid: i64,
    pub identifier: Option<String>,
    pub tile: Option<TilesetRectangle>,
    pub value: i64,
}

impl IntGridValue {
    pub(crate) fn new(int_grid_value: &ldtk::IntGridValueDefinition) -> Result<Self> {
        let color = bevy_color_from_ldtk_string(&int_grid_value.color)?;
        let group_uid = int_grid_value.group_uid;
        let identifier = int_grid_value.identifier.clone();
        let tile = int_grid_value.tile.as_ref().map(TilesetRectangle::new);
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

pub type IntGridValuesGroups = HashMap<i64, IntGridValuesGroup>;

#[derive(Debug, Reflect)]
pub struct IntGridValuesGroup {
    pub color: Option<Color>,
    pub identifier: Option<String>,
}

impl IntGridValuesGroup {
    pub(crate) fn new(value: &ldtk::IntGridValueGroupDefinition) -> Result<Self> {
        let color = value
            .color
            .as_deref()
            .map(bevy_color_from_ldtk_string)
            .transpose()?;
        let identifier = value.identifier.clone();

        Ok(Self { color, identifier })
    }
}
