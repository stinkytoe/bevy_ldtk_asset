use bevy::math::{I64Vec2, Vec2};
use bevy::reflect::Reflect;

use crate::ldtk;

//#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct TilesetDefinition {
//    /// Grid-based height
//    #[serde(rename = "__cHei")]
//    pub c_hei: i64,
//    /// Grid-based width
//    #[serde(rename = "__cWid")]
//    pub c_wid: i64,
//    /// The following data is used internally for various optimizations. It's always synced with
//    /// source image changes.
//    pub cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,
//    /// An array of custom tile metadata
//    pub custom_data: Vec<TileCustomMetadata>,
//    /// If this value is set, then it means that this atlas uses an internal LDtk atlas image
//    /// instead of a loaded one. Possible values: &lt;`null`&gt;, `LdtkIcons`
//    pub embed_atlas: Option<EmbedAtlas>,
//    /// Tileset tags using Enum values specified by `tagsSourceEnumId`. This array contains 1
//    /// element per Enum value, which contains an array of all Tile IDs that are tagged with it.
//    pub enum_tags: Vec<EnumTagValue>,
//    /// User defined unique identifier
//    pub identifier: String,
//    /// Distance in pixels from image borders
//    pub padding: i64,
//    /// Image height in pixels
//    pub px_hei: i64,
//    /// Image width in pixels
//    pub px_wid: i64,
//    /// Path to the source file, relative to the current project JSON file<br/>  It can be null
//    /// if no image was provided, or when using an embed atlas.
//    pub rel_path: Option<String>,
//    /// Array of group of tiles selections, only meant to be used in the editor
//    pub saved_selections: Vec<HashMap<String, Option<serde_json::Value>>>,
//    /// Space in pixels between all tiles
//    pub spacing: i64,
//    /// An array of user-defined tags to organize the Tilesets
//    pub tags: Vec<String>,
//    /// Optional Enum definition UID used for this tileset meta-data
//    pub tags_source_enum_uid: Option<i64>,
//    pub tile_grid_size: i64,
//    /// Unique Intidentifier
//    pub uid: i64,
//}

#[derive(Debug, Reflect)]
pub struct TileCustomMetadata {
    pub data: String,
    pub tile_id: i64,
}

impl From<ldtk::TileCustomMetadata> for TileCustomMetadata {
    fn from(value: ldtk::TileCustomMetadata) -> Self {
        Self {
            data: value.data,
            tile_id: value.tile_id,
        }
    }
}

#[derive(Debug, Reflect)]
pub struct EnumTagValue {
    pub enum_value_id: String,
    pub tile_ids: Vec<i64>,
}

impl From<ldtk::EnumTagValue> for EnumTagValue {
    fn from(value: ldtk::EnumTagValue) -> Self {
        Self {
            enum_value_id: value.enum_value_id,
            tile_ids: value.tile_ids,
        }
    }
}

#[derive(Debug, Reflect)]
pub struct TilesetDefinition {
    pub cell_size: I64Vec2,
    pub custom_data: Vec<TileCustomMetadata>,
    pub enum_tags: Vec<EnumTagValue>,
    pub identifier: String,
    pub padding: i64,
    pub size: Vec2,
    pub rel_path: Option<String>,
    pub tags: Vec<String>,
    pub tags_source_enum_uid: Option<i64>,
    pub tile_grid_size: i64,
}

impl From<ldtk::TilesetDefinition> for (i64, TilesetDefinition) {
    fn from(value: ldtk::TilesetDefinition) -> Self {
        let uid = value.uid;

        let cell_size = (value.c_wid, value.c_hei).into();
        let custom_data = value
            .custom_data
            .into_iter()
            .map(TileCustomMetadata::from)
            .collect();
        let enum_tags = value
            .enum_tags
            .into_iter()
            .map(EnumTagValue::from)
            .collect();
        let identifier = value.identifier;
        let padding = value.padding;
        let size = (value.px_wid as f32, value.px_hei as f32).into();
        let rel_path = value.rel_path;
        let tags = value.tags;
        let tags_source_enum_uid = value.tags_source_enum_uid;
        let tile_grid_size = value.tile_grid_size;

        (
            uid,
            TilesetDefinition {
                cell_size,
                custom_data,
                enum_tags,
                identifier,
                padding,
                size,
                rel_path,
                tags,
                tags_source_enum_uid,
                tile_grid_size,
            },
        )
    }
}
