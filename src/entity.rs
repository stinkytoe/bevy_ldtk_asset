use std::str::FromStr;

use bevy::asset::{Asset, Handle, LoadContext};
use bevy::color::Color;
use bevy::math::{I64Vec2, Vec2};
use bevy::reflect::Reflect;
use bevy::sprite::Anchor;

use crate::anchor::bevy_anchor_from_ldtk;
use crate::color::bevy_color_from_ldtk_string;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::label::LayerAssetPath;
use crate::ldtk;
use crate::ldtk_asset_traits::LdtkAsset;
use crate::project_loader::ProjectContext;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Asset, Debug, Reflect)]
pub struct Entity {
    pub identifier: String,
    pub iid: Iid,
    pub grid: I64Vec2,
    pub anchor: Anchor,
    pub smart_color: Color,
    pub tags: Vec<String>,
    pub tile: Option<TilesetRectangle>,
    pub world_location: Option<Vec2>,
    pub def_uid: i64,
    pub field_instances: Vec<FieldInstance>,
    pub size: Vec2,
    pub location: Vec2,
    //pub parent_path: String,
}

impl Entity {
    pub(crate) fn create_handle_pair(
        value: &ldtk::EntityInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        _project_context: &ProjectContext,
    ) -> crate::Result<(Iid, Handle<Self>)> {
        let identifier = value.identifier.clone();
        let iid = Iid::from_str(&value.iid)?;
        let grid = (value.grid.len() == 2)
            .then(|| (value.grid[0], value.grid[1]).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad value for grid! given: {:?}",
                value.grid
            )))?;
        let anchor = bevy_anchor_from_ldtk(&value.pivot)?;
        let smart_color = bevy_color_from_ldtk_string(&value.smart_color)?;
        let tags = value.tags.clone();
        //let tile: Option<TilesetRectangle> = value.tile.map(|tile| tile.into());
        let tile = value.tile.as_ref().map(TilesetRectangle::new);
        let world_location = match (value.world_x, value.world_y) {
            (None, None) => None,
            (None, Some(y)) => {
                return Err(crate::Error::LdtkImportError(format!(
                    "When constructing an entity, world_x was None but world_y was Some({y})!",
                )))
            }
            (Some(x), None) => {
                return Err(crate::Error::LdtkImportError(format!(
                    "When constructing an entity, world_x was Some({x}) but world_y was None!",
                )))
            }
            (Some(x), Some(y)) => Some((x as f32, -y as f32).into()),
        };
        let def_uid = value.def_uid;
        let field_instances = value
            .field_instances
            .iter()
            .map(FieldInstance::new)
            .collect::<Result<_, _>>()?;
        let size = (value.width as f32, value.height as f32).into();
        let location = (value.px.len() == 2)
            .then(|| (value.px[0] as f32, -value.px[1] as f32).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Unable to parse Vec2 from entity px field! given: {:?}",
                value.grid
            )))?;

        let entity_asset_path = layer_asset_path.to_entity_asset_path(&identifier, iid);

        let entity = Self {
            identifier,
            iid,
            grid,
            anchor,
            smart_color,
            tags,
            tile,
            world_location,
            def_uid,
            field_instances,
            size,
            location,
        }
        .into();

        let handle =
            load_context.add_loaded_labeled_asset(entity_asset_path.to_asset_label(), entity);

        Ok((iid, handle))
    }
}

impl LdtkAsset for Entity {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn iid(&self) -> Iid {
        self.iid
    }
}
