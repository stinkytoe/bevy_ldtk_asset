use std::str::FromStr;

use bevy_asset::{Asset, Handle, LoadContext};
use bevy_log::error;
use bevy_math::{I64Vec2, Vec2};
use bevy_reflect::Reflect;
use bevy_render::texture::Image;

use crate::asset_labels::{LayerAssetPath, LevelAssetPath};
use crate::entity::Entity;
use crate::iid::{Iid, IidMap};
use crate::layer_definition::LayerDefinition;
use crate::ldtk;
use crate::ldtk_asset_trait::{LdtkAsset, LdtkAssetWithChildren};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::project_loader::{ProjectContext, ProjectDefinitionContext};
use crate::tile_instance::TileInstance;
use crate::tileset_definition::TilesetDefinition;
use crate::uid::Uid;
use crate::{ldtk_import_error, Result};

#[derive(Debug, Reflect)]
pub struct EntitiesLayer {
    pub entity_handles: IidMap<Handle<Entity>>,
}

impl EntitiesLayer {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definitions_context: &ProjectDefinitionContext,
    ) -> Result<Self> {
        (value.int_grid_csv.is_empty()
            && value.grid_tiles.is_empty()
            && value.auto_layer_tiles.is_empty()
            && value.tileset_rel_path.is_none()
            && value.tileset_def_uid.is_none())
        .then(|| -> Result<_> {
            let entity_handles = value
                .entity_instances
                .iter()
                .map(|value| {
                    Entity::create_handle_pair(
                        value,
                        layer_asset_path,
                        load_context,
                        project_context,
                        project_definitions_context,
                    )
                })
                .collect::<Result<_>>()?;
            Ok(Self { entity_handles })
        })
        .transpose()?
        .ok_or(ldtk_import_error!("Entities layer with Tile data!"))
    }
}

#[derive(Debug, Reflect)]
pub struct TilesLayer {
    pub int_grid: Vec<i64>,
    pub tiles: Vec<TileInstance>,
    pub tileset_definition: Option<Handle<TilesetDefinition>>,
    // This will be filled with the default image handle if the JSON is null.
    pub tileset_image: Handle<Image>,
}

impl TilesLayer {
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<TilesLayer> {
        let tiles: &[_] = match value.layer_instance_type.as_str() {
            "Tiles" => &value.grid_tiles,
            "AutoLayer" | "IntGrid" => &value.auto_layer_tiles,
            // Since this was checked in LayerType::new(..), things are screwy if we reach here and
            // therefore we panic!
            _ => unreachable!(),
        };

        value
            .entity_instances
            .is_empty()
            .then(|| -> Result<_> {
                let int_grid = value.int_grid_csv.clone();
                let tiles = tiles.iter().map(TileInstance::new).collect::<Result<_>>()?;
                let tileset_definition = value
                    .tileset_def_uid
                    .and_then(|uid| project_definition_context.tileset_definitions.get(&uid))
                    .cloned();
                let tileset_image = value
                    .tileset_rel_path
                    .as_deref()
                    .map(|ldtk_path| {
                        ldtk_path_to_bevy_path(project_context.project_directory, ldtk_path)
                    })
                    .map(|bevy_path| load_context.load(bevy_path))
                    .unwrap_or_default();

                // Not a parse error, but should be reported to the user.
                if tileset_definition.is_none() {
                    error!(
                        "tileset_definition is None in layer: {}! This is technically \
                        a valid LDtk file, but the editor will show an error message for the layer \
                        with the missing tileset. Please correct inside of LDtk!",
                        value.identifier
                    );
                }

                Ok(Self {
                    int_grid,
                    tiles,
                    tileset_definition,
                    tileset_image,
                })
            })
            .transpose()?
            .ok_or(ldtk_import_error!("Entities layer with Tile data!"))
    }
}

#[derive(Debug, Reflect)]
pub enum LayerType {
    Entities(EntitiesLayer),
    IntGrid(TilesLayer),
    Tiles(TilesLayer),
    AutoLayer(TilesLayer),
}

impl LayerType {
    pub fn is_tiles_layer(&self) -> bool {
        !matches!(self, Self::Entities(_))
    }

    pub fn is_entities_layer(&self) -> bool {
        matches!(self, Self::Entities(_))
    }

    pub fn get_tiles_layer(&self) -> Option<&TilesLayer> {
        match self {
            LayerType::Entities(_) => None,
            LayerType::IntGrid(tiles_layer)
            | LayerType::Tiles(tiles_layer)
            | LayerType::AutoLayer(tiles_layer) => Some(tiles_layer),
        }
    }

    pub fn get_entities_layer(&self) -> Option<&EntitiesLayer> {
        if let Self::Entities(entities_layer) = self {
            Some(entities_layer)
        } else {
            None
        }
    }
}

impl LayerType {
    fn new(
        value: &ldtk::LayerInstance,
        layer_asset_path: &LayerAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<Self> {
        match value.layer_instance_type.as_str() {
            "Entities" => Ok(Self::Entities(EntitiesLayer::new(
                value,
                layer_asset_path,
                load_context,
                project_context,
                project_definition_context,
            )?)),
            "Tiles" | "AutoLayer" | "IntGrid" => Ok(Self::Tiles(TilesLayer::new(
                value,
                load_context,
                project_context,
                project_definition_context,
            )?)),
            unknown => Err(ldtk_import_error!("Unknown layer type! given: {unknown}")),
        }
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct Layer {
    pub grid_size: I64Vec2,
    pub grid_cell_size: i64,
    pub identifier: String,
    pub opacity: f64,
    pub total_offset: Vec2,
    pub layer_type: LayerType,
    pub iid: Iid,
    pub layer_definition: Handle<LayerDefinition>,
    pub level_id: Uid,
    pub location: Vec2,
    pub index: usize,
}

impl Layer {
    pub(crate) fn create_handle_pair(
        value: &ldtk::LayerInstance,
        index: usize,
        level_asset_path: &LevelAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
        project_definition_context: &ProjectDefinitionContext,
    ) -> Result<(Iid, Handle<Self>)> {
        let grid_size: I64Vec2 = (value.c_wid, value.c_hei).into();
        let grid_cell_size = value.grid_size;
        let identifier = value.identifier.clone();
        let layer_asset_path = level_asset_path.to_layer_asset_path(&identifier)?;
        let opacity = value.opacity;
        let total_offset = (
            value.px_total_offset_x as f32,
            -value.px_total_offset_y as f32,
        )
            .into();
        let layer_type = LayerType::new(
            value,
            &layer_asset_path,
            load_context,
            project_context,
            project_definition_context,
        )?;
        let iid = Iid::from_str(&value.iid)?;
        let layer_definition = project_definition_context
            .layer_definitions
            .get(&value.layer_def_uid)
            .ok_or(ldtk_import_error!(
                "Bad layer definition uid! given: {}",
                value.layer_def_uid
            ))?
            .clone();
        let level_id = value.level_id;
        let location = (
            value.px_total_offset_x as f32,
            -value.px_total_offset_y as f32,
        )
            .into();

        // Sanity check to guarantee that the int_grid size makes sense
        let int_grid_len = value.int_grid_csv.len();
        let total_grids = (grid_size.x * grid_size.y) as usize;
        if int_grid_len != 0 && int_grid_len != total_grids {
            return Err(ldtk_import_error!(
                "Bad length for int_grid_csv in layer {identifier}! length:{int_grid_len}"
            ));
        }

        let layer = Layer {
            grid_size,
            grid_cell_size,
            identifier,
            opacity,
            total_offset,
            layer_type,
            iid,
            layer_definition,
            level_id,
            location,
            index,
        }
        .into();

        let handle =
            load_context.add_loaded_labeled_asset(layer_asset_path.to_asset_label(), layer);

        Ok((iid, handle))
    }
}

impl LdtkAsset for Layer {
    fn get_identifier(&self) -> &str {
        &self.identifier
    }

    fn get_iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkAssetWithChildren<Entity> for Layer {
    fn get_children(&self) -> impl Iterator<Item = &Handle<Entity>> {
        match &self.layer_type {
            LayerType::Entities(entities_layer) => {
                either::Left(entities_layer.entity_handles.values())
            }
            LayerType::IntGrid(_) | LayerType::Tiles(_) | LayerType::AutoLayer(_) => {
                either::Right([].iter())
            }
        }
    }
}
