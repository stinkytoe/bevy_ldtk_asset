//! Tileset Definitions
//!
//! Information about a tileset.
//!
//! See [Tileset Definition: LDtk docs](https://ldtk.io/json/#ldtk-TilesetDefJson)

use bevy_asset::{Asset, Handle};
use bevy_image::Image;
use bevy_math::I64Vec2;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::ldtk_asset_trait::LdtkAssetWithTags;
use crate::result::LdtkResult;
use crate::uid::Uid;
use crate::{ldtk, ldtk_import_error};

/// A tileset definition for use in visualizations.
///
/// See [TilesetDefinition](https://ldtk.io/json/#ldtk-TilesetDefJson)
#[derive(Asset, Debug, Reflect)]
pub struct TilesetDefinition {
    /// Grid based cell size.
    ///
    /// From the `__cHei` and `__cWid` LDtk JSON fields.
    pub tile_grid_size: I64Vec2,
    /// Optional Custom data for tiles.
    ///
    /// User provided data. Indexed by the tile id.
    pub custom_data: HashMap<i64, String>,
    /// A collection of enum value ids, and a list of tileset ids which are
    /// marked with the given enum value id.
    pub enum_tags: HashMap<String, Vec<i64>>,
    /// User defined unique identifier.
    pub identifier: String,
    /// Distance in pixels from image borders.
    pub padding: i64,
    /// Size of the tileset image, in pixels.
    ///
    /// From the `pixHei` and `pixWid` LDtk JSON fields.
    pub tileset_image_size: I64Vec2,
    /// Optional handle to the linked tile set image's object.
    ///
    /// This is passed along from the LDtk project, and it is not guaranteed
    /// that the image actually exists, or can be successfully loaded. If the
    /// asset fails to load, then we will hold onto the failed asset handle.
    ///
    /// From the `relPath` LDtk JSON fields.
    pub tileset_image: Option<Handle<Image>>,
    /// An array of user defined tags.
    pub tags: Vec<String>,
    /// Optional [Uid] for an associated enum definition.
    pub tags_source_enum_uid: Option<Uid>,
    /// Size of tiles, in pixels. Only square tiles supported.
    ///
    /// From the `tileGridSize` LDtk JSON fields.
    pub tile_grid_pixel_size: i64,
}

impl TilesetDefinition {
    pub(crate) async fn new(
        value: ldtk::TilesetDefinition,
        tileset_definition_images: &HashMap<String, Handle<Image>>,
    ) -> LdtkResult<Self> {
        // see https://github.com/stinkytoe/bevy_ldtk_asset/issues/35
        if value.embed_atlas.is_some() {
            return Err(ldtk_import_error!(
                "This LDtk project contains an embedded atlas!\
                 Licensing prevents us from loading this asset. At this time we are considering\
                 it to be an error to attempt to load an LDtk project with an embedded atlas!\
                 See https://github.com/stinkytoe/bevy_ldtk_asset/issues/35 for a discussion\
                 relating to this decision."
            ));
        }

        let identifier = value.identifier;
        let tile_grid_size = (value.c_wid, value.c_hei).into();
        let custom_data = value
            .custom_data
            .into_iter()
            .map(|tile| (tile.tile_id, tile.data))
            .collect();
        let enum_tags = value
            .enum_tags
            .into_iter()
            .map(|enum_tag| (enum_tag.enum_value_id, enum_tag.tile_ids))
            .collect();
        let padding = value.padding;
        let tileset_image_size = (value.px_wid, value.px_hei).into();
        let tileset_image = value
            .rel_path
            .map(|rel_path| {
                tileset_definition_images
                    .get(&rel_path)
                    .ok_or(ldtk_import_error!("Bad rel path! {rel_path}"))
            })
            .transpose()?
            .cloned();

        let tags = value.tags;
        let tags_source_enum_uid = value.tags_source_enum_uid;
        let tile_grid_pixel_size = value.tile_grid_size;

        Ok(TilesetDefinition {
            identifier,
            tile_grid_size,
            custom_data,
            enum_tags,
            padding,
            tileset_image_size,
            tileset_image,
            tags,
            tags_source_enum_uid,
            tile_grid_pixel_size,
        })
    }
}

impl LdtkAssetWithTags for TilesetDefinition {
    fn get_tags(&self) -> &[String] {
        &self.tags
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
