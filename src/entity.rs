use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_math::{I64Vec2, Vec2};
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;

use crate::anchor::bevy_anchor_from_ldtk;
use crate::asset_labels::LayerAssetPath;
use crate::color::bevy_color_from_ldtk_string;
use crate::entity_definition::EntityDefinition;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::ldtk_asset_trait::LdtkAsset;
use crate::project_loader::{ProjectContext, ProjectDefinitionContext};
use crate::tileset_rectangle::TilesetRectangle;
use crate::{ldtk, ldtk_import_error};

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
    pub entity_definition: Handle<EntityDefinition>,
    pub field_instances: Vec<FieldInstance>,
    pub size: Vec2,
    pub location: Vec2,
}

impl Entity {
    pub(crate) fn create_handle_pair(
        value: &ldtk::EntityInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        _project_context: &ProjectContext,
        project_definitions_context: &ProjectDefinitionContext,
    ) -> crate::Result<(Iid, Handle<Self>)> {
        let identifier = value.identifier.clone();
        let iid = Iid::from_str(&value.iid)?;
        let grid = (value.grid.len() == 2)
            .then(|| (value.grid[0], value.grid[1]).into())
            .ok_or(ldtk_import_error!(
                "Bad value for grid! given: {:?}",
                value.grid
            ))?;
        let anchor = bevy_anchor_from_ldtk(&value.pivot)?;
        let smart_color = bevy_color_from_ldtk_string(&value.smart_color)?;
        let tags = value.tags.clone();
        //let tile: Option<TilesetRectangle> = value.tile.map(|tile| tile.into());
        let tile = value
            .tile
            .as_ref()
            .map(|value| {
                TilesetRectangle::new(value, project_definitions_context.tileset_definitions)
            })
            .transpose()?;
        let world_location = match (value.world_x, value.world_y) {
            (None, None) => None,
            (None, Some(y)) => {
                return Err(ldtk_import_error!(
                    "When constructing an entity, world_x was None but world_y was Some({y})!",
                ))
            }
            (Some(x), None) => {
                return Err(ldtk_import_error!(
                    "When constructing an entity, world_x was Some({x}) but world_y was None!",
                ))
            }
            (Some(x), Some(y)) => Some((x as f32, -y as f32).into()),
        };
        let entity_definition = project_definitions_context
            .entity_definitions
            .get(&value.def_uid)
            .ok_or(ldtk_import_error!(
                "bad entity definition uid! given: {}",
                value.def_uid
            ))?
            .clone();
        let field_instances = value
            .field_instances
            .iter()
            .map(|value| FieldInstance::new(value, project_definitions_context.tileset_definitions))
            .collect::<Result<_, _>>()?;
        let size = (value.width as f32, value.height as f32).into();
        let location = (value.px.len() == 2)
            .then(|| (value.px[0] as f32, -value.px[1] as f32).into())
            .ok_or(ldtk_import_error!(
                "Unable to parse Vec2 from entity px field! given: {:?}",
                value.grid
            ))?;

        let entity_asset_path = layer_asset_path.to_entity_asset_path(&identifier, iid)?;

        let entity = Self {
            identifier,
            iid,
            grid,
            anchor,
            smart_color,
            tags,
            tile,
            world_location,
            entity_definition,
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
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}
