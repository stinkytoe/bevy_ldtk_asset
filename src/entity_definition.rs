use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;

use crate::anchor::bevy_anchor_from_ldtk;
use crate::asset_labels::ProjectAssetPath;
use crate::color::bevy_color_from_ldtk_string;
use crate::tileset_definition::TilesetDefinition;
use crate::tileset_rectangle::TilesetRectangle;
use crate::uid::{Uid, UidMap};
use crate::{ldtk, Result};

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

#[derive(Asset, Debug, Reflect)]
pub struct EntityDefinition {
    pub identifier: String,
    pub color: Color,
    pub size: Vec2,
    // TODO: add nine_slice_borders field!
    pub anchor: Anchor,
    pub tile: Option<TilesetRectangle>,
    pub ui_tile: Option<TilesetRectangle>,
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
