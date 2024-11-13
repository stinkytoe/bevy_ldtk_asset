//! The LDtk entity, represented as a Bevy asset.
//!
//! This is an import of an LDtk
//! [EntityInstance](https://ldtk.io/json/#ldtk-EntityInstanceJson)

use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_color::Color;
use bevy_log::error;
use bevy_math::{I64Vec2, Vec2};
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;
use bevy_utils::HashMap;

use crate::anchor::bevy_anchor_from_ldtk;
use crate::asset_labels::LayerAssetPath;
use crate::color::bevy_color_from_ldtk_string;
use crate::entity_definition::EntityDefinition;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithFieldInstances};
use crate::project_loader::{ProjectContext, ProjectDefinitionContext};
use crate::tileset_rectangle::TilesetRectangle;
use crate::Result;
use crate::{ldtk, ldtk_import_error};

/// An asset representing an [LDtk Entity Instance](https://ldtk.io/json/#ldtk-EntityInstanceJson)
///
/// See [crate::asset_labels] for a description of the label format.
#[derive(Asset, Debug, Reflect)]
pub struct Entity {
    /// The identifier for this specific entity.
    ///
    /// Unlike other identifiers, there is no guarantee that this is unique.
    pub identifier: String,
    /// The Iid. This will likely always be unique, even across projects.
    pub iid: Iid,
    /// The grid location of the entity in the containing layer
    pub grid: I64Vec2,
    /// The anchor point of the entity.
    ///
    /// This represents where the 'center' of the entity is. Both the spatial location and the
    /// graphical representation of the entity use this anchor.
    pub anchor: Anchor,
    /// A color representing the entity, calculated by LDtk. Not normally used for visualization.
    pub smart_color: Color,
    /// A list of tags assigned to this specific entity.
    ///
    /// These are assigned in the entity definition, but copied to the instance for convenience.
    /// This allows the user to designate properties and intents about this entity to the game,
    /// such as: is it a player/npc/enemy etc.
    pub tags: Vec<String>,
    /// An optional [TilesetRectangle].
    ///
    /// This is used by the editor as the default visualization, but could be used by a game as the
    /// visualization as well.
    pub tile: Option<TilesetRectangle>,
    /// The entity's location in world space, as defined in the LDtk project.
    ///
    /// This is converted from LDtk's coordinate space to Bevy's pixel coordinate space by negating
    /// the y value.
    pub world_location: Option<Vec2>,
    /// A handle pointing to the [EntityDefinition] asset.
    pub entity_definition: Handle<EntityDefinition>,
    /// A hash map of [FieldInstance] entries, indexed by their identifier.
    ///
    /// These can be defined either in the LDtk [EntityDefinition](https://ldtk.io/json/#ldtk-EntityDefJson),
    /// or the [EntityInstance](https://ldtk.io/json/#ldtk-EntityInstanceJson) itself.
    pub field_instances: HashMap<String, FieldInstance>,
    /// The size of the entity object.
    ///
    /// Note: this does not nesessarily correlate with the size of the entity's visualization, if
    /// it defines one.
    pub size: Vec2,
    /// The entity's location in the space defined by its containing [Layer].
    ///
    /// This is converted from LDtk's coordinate space to Bevy's pixel coordinate space by negating
    /// the y value.
    pub location: Vec2,
}

impl Entity {
    pub(crate) fn create_handle_pair(
        value: &ldtk::EntityInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        _project_context: &ProjectContext,
        project_definitions_context: &ProjectDefinitionContext,
    ) -> Result<(Iid, Handle<Self>)> {
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
            .filter(|value| {
                let ret = value.value.is_some();
                if !ret {
                    error!("Skipping field instance {value:?} because inner value is None!");
                }
                ret
            })
            .map(|value| -> Result<(String, FieldInstance)> {
                Ok((
                    value.identifier.clone(),
                    FieldInstance::new(
                        value,
                        project_definitions_context.tileset_definitions,
                        project_definitions_context.enum_definitions,
                    )?,
                ))
            })
            .collect::<Result<_>>()?;
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

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|inner_tag| inner_tag == tag)
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

impl LdtkAssetWithFieldInstances for Entity {
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.field_instances.get(identifier)
    }
}
