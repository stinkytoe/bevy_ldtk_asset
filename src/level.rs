use std::str::FromStr;

use bevy::asset::{Asset, Handle, LoadContext};
use bevy::color::Color;
use bevy::math::{Rect, Vec2};
use bevy::reflect::Reflect;
use bevy::render::texture::Image;

use crate::color::bevy_color_from_ldtk_string;
use crate::field_instance::FieldInstance;
use crate::iid::Iid;
use crate::label::WorldAssetPath;
use crate::layer::Layer;
use crate::ldtk;
use crate::ldtk_asset_traits::{HasChildren, LdtkAsset};
use crate::ldtk_path::ldtk_path_to_bevy_path;
use crate::prelude::IidMap;
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
#[derive(Clone, Debug, Reflect)]
pub struct LevelBackground {
    pub image: Handle<Image>,
    pub crop_rect: Rect,
    pub scale: Vec2,
    pub corner: Vec2,
}

impl LevelBackground {
    pub(crate) fn new(
        value: &ldtk::LevelBackgroundPosition,
        image: Handle<Image>,
    ) -> crate::Result<Self> {
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
            image,
            crop_rect,
            scale,
            corner,
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct Level {
    pub bg_color: Color,
    pub neighbours: Vec<Neighbour>,
    pub background: Option<LevelBackground>,
    pub field_instances: Vec<FieldInstance>,
    pub identifier: String,
    pub iid: Iid,
    pub size: Vec2,
    pub uid: Uid, // TODO: do we need this?
    pub world_depth: i64,
    pub location: Vec2,
    pub layers: IidMap<Handle<Layer>>,
    pub index: usize,
}

impl Level {
    pub(crate) fn create_handle_pair(
        value: &ldtk::Level,
        index: usize,
        world_asset_path: &WorldAssetPath,
        load_context: &mut LoadContext,
        project_context: &ProjectContext,
    ) -> crate::Result<(Iid, Handle<Self>)> {
        let bg_color = bevy_color_from_ldtk_string(&value.bg_color)?;
        let neighbours = value
            .neighbours
            .iter()
            .map(Neighbour::new)
            .collect::<Result<_, _>>()?;
        let background = match (value.bg_pos.as_ref(), value.bg_rel_path.as_ref()) {
            (None, None) => None,
            (None, Some(_)) => {
                return Err(crate::Error::LdtkImportError(
                    "bg_pos is None while bg_rel_path is Some(_)!".to_string(),
                ))
            }
            (Some(_), None) => {
                return Err(crate::Error::LdtkImportError(
                    "bg_pos is Some(_) while bg_rel_path is None!".to_string(),
                ))
            }
            (Some(bg_pos), Some(bg_rel_path)) => {
                let path = ldtk_path_to_bevy_path(project_context.project_directory, bg_rel_path);
                let image = load_context.load(path);
                let background = LevelBackground::new(bg_pos, image)?;
                Some(background)
            }
        };
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

        let level_asset_path = world_asset_path.to_level_asset_path(&identifier);

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
            .rev()
            .enumerate()
            .map(|(index, ldtk_layer_instance)| {
                Layer::create_handle_pair(
                    ldtk_layer_instance,
                    index,
                    &level_asset_path,
                    load_context,
                    project_context,
                )
            })
            .collect::<crate::Result<_>>()?;

        let level = Level {
            bg_color,
            neighbours,
            background,
            field_instances,
            identifier,
            iid,
            size,
            uid,
            world_depth,
            location,
            layers,
            index,
        }
        .into();

        let handle =
            load_context.add_loaded_labeled_asset(level_asset_path.to_asset_label(), level);

        Ok((iid, handle))
    }
}

impl LdtkAsset for Level {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn iid(&self) -> Iid {
        self.iid
    }
}

impl HasChildren for Level {
    type Child = Layer;

    fn children(&self) -> impl Iterator<Item = &Handle<Self::Child>> {
        self.layers.values()
    }
}
