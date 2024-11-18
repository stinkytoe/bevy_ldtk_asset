//! The LDtk definition for an entity.
//!
//! This is used by LDtk for creating new entity instances.
//!
//! This is an import of an LDtk
//! [EntityDefinition](https://ldtk.io/json/#ldtk-EntityDefJson)

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;

use crate::anchor::bevy_anchor_from_ldtk;
use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_string;
use crate::ldtk;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::Result;

/// Hint from LDtk on how this entity's sprite should be rendered.
///
/// See [LDtk documentation on tileRenderMode](https://ldtk.io/json/#ldtk-EntityDefJson;tileRenderMode)
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub enum TileRenderMode {
    Cover,
    FitInside,
    Repeat,
    Stretch,
    FullSizeCropped,
    FullSizeUncropped,
    NineSlice,
}

impl From<ldtk::TileRenderMode> for TileRenderMode {
    fn from(value: ldtk::TileRenderMode) -> Self {
        match value {
            ldtk::TileRenderMode::Cover => Self::Cover,
            ldtk::TileRenderMode::FitInside => Self::FitInside,
            ldtk::TileRenderMode::FullSizeCropped => Self::FullSizeCropped,
            ldtk::TileRenderMode::FullSizeUncropped => Self::FullSizeUncropped,
            ldtk::TileRenderMode::NineSlice => Self::NineSlice,
            ldtk::TileRenderMode::Repeat => Self::Repeat,
            ldtk::TileRenderMode::Stretch => Self::Stretch,
        }
    }
}

/// The definition for creating a new entity.
///
/// TODO: add nine_slice_borders field! [#21](https://github.com/stinkytoe/bevy_ldtk_asset/issues/21)
/// TODO: how are we going to handle aseprite targets? [#20](https://github.com/stinkytoe/bevy_ldtk_asset/issues/20)
#[derive(Asset, Debug, Reflect)]
pub struct EntityDefinition {
    /// The identifier for this definition.
    ///
    /// Unlike [crate::entity::Entity] instances, the identifier for the definition is unique.
    pub identifier: String,
    /// Base color for the entity.
    pub color: Color,
    /// Size of the region for this entity.
    ///
    /// Not necessarily the size of the visualization.
    pub size: Vec2,
    /// The relative `center` of the entity.
    pub anchor: Anchor,
    /// Optional [TilesetRectangle], representing a potential visualization of the entity.
    pub tile: Option<TilesetRectangle>,
    /// Optional [TilesetRectangle] for use in LDtk's GUI.
    ///
    /// Potentially an icon or portrait representation?
    pub ui_tile: Option<TilesetRectangle>,
    /// The render mode for an [crate::entity::Entity] instance's visualization.
    pub render_mode: TileRenderMode,
}

impl EntityDefinition {
    pub(crate) fn create_handle_pair(
        value: &ldtk::EntityDefinition,
        project_asset_path: &ProjectAssetPath,
        load_context: &mut LoadContext,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> Result<(Uid, Handle<Self>)> {
        let identifier = value.identifier.clone();
        let uid = value.uid;
        let color = bevy_color_from_ldtk_string(&value.color)?;
        let size = (value.width as f32, value.height as f32).into();
        let anchor = bevy_anchor_from_ldtk(&[value.pivot_x, value.pivot_y])?;
        let tile = value
            .tile_rect
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;
        let ui_tile = value
            .ui_tile_rect
            .as_ref()
            .map(|value| TilesetRectangle::new(value, tileset_definitions))
            .transpose()?;
        let render_mode = value.tile_render_mode.clone().into();

        let path = project_asset_path.to_entity_definition_asset_path(&identifier)?;

        let asset = Self {
            identifier,
            color,
            size,
            anchor,
            tile,
            ui_tile,
            render_mode,
        }
        .into();

        let handle = load_context.add_loaded_labeled_asset(path.to_asset_label(), asset);

        Ok((uid, handle))
    }
}
