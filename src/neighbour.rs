use std::str::FromStr;

use bevy::reflect::Reflect;

use crate::error::Error;
use crate::iid::Iid;
use crate::ldtk;

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
    fn new(dir: &str) -> Result<Self, Error> {
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
            _ => Err(Error::LdtkImportError(format!(
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
    pub(crate) fn new(value: &ldtk::NeighbourLevel) -> Result<Self, Error> {
        let dir = NeighbourDir::new(&value.dir)?;
        let level_iid = Iid::from_str(&value.level_iid)?;

        Ok(Self { dir, level_iid })
    }
}
