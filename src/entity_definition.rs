//! The LDtk definition for an entity.
//!
//! This is used by LDtk for creating new entity instances.
//!
//! This is an import of an LDtk
//! [EntityDefinition](https://ldtk.io/json/#ldtk-EntityDefJson).

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;

use crate::Result;
use crate::anchor::bevy_anchor_from_ldtk;
use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_string;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::{ldtk, ldtk_import_error};

/// A nine-slice pattern.
///
/// See [nineSliceBorders](https://ldtk.io/json/#ldtk-EntityDefJson;nineSliceBorders)
/// from the LDtk documentation.
#[derive(Debug, Reflect)]
pub struct NineSlice {
    pub up: i64,
    pub right: i64,
    pub down: i64,
    pub left: i64,
}

impl NineSlice {
    pub(crate) fn new(nine_slice: &[i64]) -> Result<Self> {
        (nine_slice.len() == 4)
            .then(|| Self {
                up: nine_slice[0],
                right: nine_slice[1],
                down: nine_slice[2],
                left: nine_slice[3],
            })
            .ok_or(ldtk_import_error!("bad nine_slice array! {nine_slice:?}"))
    }
}

/// Hint from LDtk on how this entity's sprite should be rendered.
///
/// See [tileRenderMode](https://ldtk.io/json/#ldtk-EntityDefJson;tileRenderMode)
/// from the LDtk documentation.
#[allow(missing_docs)]
#[derive(Debug, Reflect)]
pub enum TileRenderMode {
    Cover,
    FitInside,
    Repeat,
    Stretch,
    FullSizeCropped,
    FullSizeUncropped,
    NineSlice(NineSlice),
}

impl TileRenderMode {
    pub(crate) fn new(value: &ldtk::TileRenderMode, nine_slice: &[i64]) -> Result<Self> {
        match value {
            ldtk::TileRenderMode::Cover => {
                nine_slice
                    .is_empty()
                    .then_some(Self::Cover)
                    .ok_or(ldtk_import_error!(
                        "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                        value
                    ))
            }
            ldtk::TileRenderMode::FitInside => nine_slice
                .is_empty()
                .then_some(Self::FitInside)
                .ok_or(ldtk_import_error!(
                    "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                    value
                )),
            ldtk::TileRenderMode::FullSizeCropped => nine_slice
                .is_empty()
                .then_some(Self::FullSizeCropped)
                .ok_or(ldtk_import_error!(
                    "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                    value
                )),
            ldtk::TileRenderMode::FullSizeUncropped => nine_slice
                .is_empty()
                .then_some(Self::FullSizeUncropped)
                .ok_or(ldtk_import_error!(
                    "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                    value
                )),
            ldtk::TileRenderMode::Repeat => {
                nine_slice
                    .is_empty()
                    .then_some(Self::Repeat)
                    .ok_or(ldtk_import_error!(
                        "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                        value
                    ))
            }
            ldtk::TileRenderMode::Stretch => {
                nine_slice
                    .is_empty()
                    .then_some(Self::Stretch)
                    .ok_or(ldtk_import_error!(
                        "tile_render_mode with non-empty nine_slice array of type: {:?}!",
                        value
                    ))
            }
            ldtk::TileRenderMode::NineSlice => Ok(Self::NineSlice(NineSlice::new(nine_slice)?)),
        }
    }
}

/// The definition for creating a new entity.
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
    pub size: I64Vec2,
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
        let size = (value.width, value.height).into();
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
        let render_mode =
            TileRenderMode::new(&value.tile_render_mode, value.nine_slice_borders.as_slice())?;

        let path = project_asset_path.to_entity_definition_asset_path(&identifier)?;

        let asset = Self {
            identifier,
            color,
            size,
            anchor,
            tile,
            ui_tile,
            render_mode,
        };

        let handle = load_context.add_labeled_asset(path.to_asset_label(), asset);

        Ok((uid, handle))
    }
}
