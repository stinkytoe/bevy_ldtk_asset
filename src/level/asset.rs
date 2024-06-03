use bevy::prelude::*;
use bevy::utils::HashMap;
use std::path::PathBuf;
use thiserror::Error;

use crate::field_instance::FieldInstance;
use crate::field_instance::FieldInstanceValueParseError;
use crate::field_instance::FieldInstances;
use crate::layer::LayerAsset;
use crate::ldtk;
use crate::level::LevelBackgroundPosition;
use crate::level::Neighbour;
use crate::level::NeighbourError;
use crate::level::Neighbours;
use crate::project::ProjectAsset;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum NewLevelAssetError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    NeighbourError(#[from] NeighbourError),
    #[error(transparent)]
    FieldInstanceValueParseError(#[from] FieldInstanceValueParseError),
}

#[derive(Asset, Debug, Reflect)]
pub struct LevelAsset {
    pub bg_color: Color,
    pub bg_pos: Option<LevelBackgroundPosition>,
    pub neighbours: Neighbours,
    pub bg_rel_path: Option<PathBuf>,
    pub field_instances: FieldInstances,
    pub identifier: String,
    pub iid: String,
    pub size: Vec2,
    // (worldX, worldY, and worldDepth)
    // In Bevy coordinate system, not necessarily the same as Bevy transform!
    pub world_location: Vec3,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
    pub layer_assets_by_identifier: HashMap<String, Handle<LayerAsset>>,
    pub layer_assets_by_iid: HashMap<String, Handle<LayerAsset>>,
}

impl LevelAsset {
    pub(crate) fn new(
        value: &ldtk::Level,
        project: Handle<ProjectAsset>,
        layer_assets_by_identifier: HashMap<String, Handle<LayerAsset>>,
        layer_assets_by_iid: HashMap<String, Handle<LayerAsset>>,
    ) -> Result<Self, NewLevelAssetError> {
        Ok(Self {
            bg_color: bevy_color_from_ldtk(&value.bg_color)?,
            bg_pos: value.bg_pos.as_ref().map(LevelBackgroundPosition::from),
            neighbours: Neighbours {
                neighbours: value
                    .neighbours
                    .iter()
                    .map(Neighbour::try_from)
                    .collect::<Result<_, _>>()?,
            },
            bg_rel_path: value.bg_rel_path.as_ref().map(PathBuf::from),
            field_instances: FieldInstances {
                field_instances: value
                    .field_instances
                    .iter()
                    .map(FieldInstance::try_from)
                    .collect::<Result<_, _>>()?,
            },
            identifier: value.identifier.clone(),
            iid: value.iid.clone(),
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            world_location: Vec3::new(
                value.world_x as f32,
                -value.world_y as f32,
                value.world_depth as f32,
            ),
            project,
            layer_assets_by_identifier,
            layer_assets_by_iid,
        })
    }
}
