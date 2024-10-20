use std::str::FromStr;

use bevy::asset::{Asset, LoadContext};
use bevy::color::Color;
use bevy::math::{Rect, Vec2};
use bevy::reflect::Reflect;

use crate::color::bevy_color_from_ldtk_string;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::layer::Layer;
use crate::ldtk;
use crate::ldtk_asset_traits::{HasIdentifier, HasIid, LdtkAsset};
use crate::project_loader::ProjectContext;
use crate::uid::Uid;

#[derive(Debug, Reflect)]
pub enum NeighbourDir {
    North,
    South,
    East,
    West,
    Lower,
    Greater,
    Overlap,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl NeighbourDir {
    fn new(dir: &str) -> crate::Result<Self> {
        match dir {
            "n" => Ok(Self::North),
            "s" => Ok(Self::South),
            "w" => Ok(Self::West),
            "e" => Ok(Self::East),
            "<" => Ok(Self::Lower),
            ">" => Ok(Self::Greater),
            "o" => Ok(Self::Overlap),
            "nw" => Ok(Self::NorthWest),
            "ne" => Ok(Self::NorthEast),
            "sw" => Ok(Self::SouthWest),
            "se" => Ok(Self::SouthEast),
            _ => Err(crate::Error::LdtkImportError(format!(
                "Bad direction from LDtk neighbor! given: {dir}"
            ))),
        }
    }
}

#[derive(Debug, Reflect)]
pub struct Neighbour {
    pub dir: NeighbourDir,
    pub level_iid: Iid,
}

impl Neighbour {
    pub(crate) fn new(value: &ldtk::NeighbourLevel) -> crate::Result<Self> {
        let dir = NeighbourDir::new(&value.dir)?;
        let level_iid = Iid::from_str(&value.level_iid)?;

        Ok(Self { dir, level_iid })
    }
}
#[derive(Debug, Reflect)]
pub struct LevelBackgroundPosition {
    pub crop_rect: Rect,
    pub scale: Vec2,
    pub corner: Vec2,
}

impl LevelBackgroundPosition {
    pub(crate) fn new(value: &ldtk::LevelBackgroundPosition) -> crate::Result<Self> {
        let crop_rect = (value.crop_rect.len() == 4)
            .then(|| {
                let p0 = (value.crop_rect[0] as f32, value.crop_rect[1] as f32).into();
                let size = Vec2::new(value.crop_rect[2] as f32, value.crop_rect[3] as f32);
                let p1 = p0 + size;
                Rect::from_corners(p0, p1)
            })
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad value for crop_rect! given: {:?}",
                value.crop_rect
            )))?;
        let scale = (value.scale.len() == 2)
            .then(|| (value.scale[0] as f32, value.scale[1] as f32).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad value for scale! given: {:?}",
                value.crop_rect
            )))?;
        let corner = (value.top_left_px.len() == 2)
            .then(|| (value.top_left_px[0] as f32, value.top_left_px[1] as f32).into())
            .ok_or(crate::Error::LdtkImportError(format!(
                "Bad value for corner! given: {:?}",
                value.crop_rect
            )))?;

        Ok(Self {
            crop_rect,
            scale,
            corner,
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct Level {
    pub bg_color: Color,
    pub bg_pos: Option<LevelBackgroundPosition>,
    pub neighbours: Vec<Neighbour>,
    pub bg_rel_path: Option<String>,
    pub field_instances: Vec<FieldInstance>,
    pub identifier: String,
    pub iid: Iid,
    pub size: Vec2,
    pub uid: Uid, // TODO: do we need this?
    pub world_depth: i64,
    pub location: Vec2,
    pub layers: Vec<Layer>,
}

impl Level {
    pub(crate) fn new(
        value: &ldtk::Level,
        _load_context: &mut LoadContext,
        _project_context: &ProjectContext,
    ) -> crate::Result<Self> {
        let bg_color = bevy_color_from_ldtk_string(&value.bg_color)?;
        let bg_pos: Option<LevelBackgroundPosition> = match value.bg_pos.as_ref() {
            Some(bg_pos) => Some(LevelBackgroundPosition::new(bg_pos)?),
            None => None,
        };
        let neighbours = value
            .neighbours
            .iter()
            .map(Neighbour::new)
            .collect::<Result<_, _>>()?;
        let bg_rel_path = value.bg_rel_path.clone();
        let field_instances = value
            .field_instances
            .iter()
            .map(FieldInstance::new)
            .collect::<Result<_, _>>()?;
        let identifier = value.identifier.clone();
        let iid = Iid::from_str(&value.iid)?;
        let size = (value.px_wid as f32, value.px_hei as f32).into();
        let uid = value.uid;
        let world_depth = value.world_depth;
        let location = (value.world_x as f32, -value.world_y as f32).into();

        let layer_instances =
            value
                .layer_instances
                .as_ref()
                .ok_or(crate::Error::LdtkImportError(
                    "layer_instances is None? \
                    Are we opening the local layer definition instead of the external one?"
                        .to_string(),
                ))?;

        let layers = layer_instances
            .iter()
            .map(|ldtk_layer_instance| {
                //let level_iid = Iid::from_str(&ldtk_level.iid)?;
                //let level_label = format!("{}/{}", ldtk_world.identifier, ldtk_level.identifier);
                //let level = Level::new(ldtk_level, load_context, project_context)?.into();
                //let handle = load_context.add_loaded_labeled_asset(level_label, level);
                //Ok((level_iid, handle))
                let layer_iid = Iid::from_str(&ldtk_layer_instance.iid)?;
                //let layer_label = format!("{}/{}", )
                //
                todo!()
            })
            .collect::<crate::Result<_>>()?;

        Ok(Level {
            bg_color,
            bg_pos,
            neighbours,
            bg_rel_path,
            field_instances,
            identifier,
            iid,
            size,
            uid,
            world_depth,
            location,
            layers,
        })
    }
}

impl LdtkAsset for Level {}

impl HasIid for Level {
    fn iid(&self) -> Iid {
        self.iid
    }
}

impl HasIdentifier for Level {
    fn identifier(&self) -> &str {
        &self.identifier
    }
}
