//! This represents the region of a [TilesetDefinition] which is used by an
//! LDtk entity for representing its sprite. Also by field instances.
//!
//! TODO: Improve this doc
use bevy_asset::Handle;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;

use crate::ldtk;
use crate::ldtk_import_error;
use crate::result::LdtkResult;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::UidMap;

/// The visualization for an [crate::entity::Entity] asset.
/// This can also be stored in [crate::field_instance::FieldInstance]s for reference.
///
/// The Anchor and underlying tile set are defined in the entity instance.
#[allow(missing_docs)]
#[derive(Clone, Debug, Reflect)]
pub struct TilesetRectangle {
    pub corner: I64Vec2,
    pub size: I64Vec2,
    pub tileset_definition: Handle<TilesetDefinition>,
}

impl TilesetRectangle {
    pub(crate) fn new(
        value: ldtk::TilesetRectangle,
        tileset_definitions: &UidMap<Handle<TilesetDefinition>>,
    ) -> LdtkResult<Self> {
        let corner = (value.x, value.y).into();
        let size = (value.w, value.h).into();
        let tileset_definition = tileset_definitions
            .get(&value.tileset_uid)
            .ok_or(ldtk_import_error!(
                "Bad tileset definition uid! given: {}",
                value.tileset_uid
            ))?
            .clone();

        Ok(Self {
            corner,
            size,
            tileset_definition,
        })
    }
}
