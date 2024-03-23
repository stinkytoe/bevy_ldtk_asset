use bevy::{prelude::*, utils::thiserror};
use thiserror::Error;

use crate::prelude::LdtkEntity;

/// A system parameter for querying which entities, if any,
/// contain a given tag in world space.
pub type LdtkEntitiesWithTag<'world, 'state> = Query<'world, 'state, (Entity, &'static LdtkEntity)>;

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum FindSingleError {
    /// No entity fits the query.
    #[error("No entities contain tag {0}")]
    NoEntities(String),
    /// Multiple entities fit the query.
    #[error("Multiple entities contain tag {0}")]
    MultipleEntities(String),
}

#[allow(missing_docs)]
pub trait LdtkEntitiesWithTagTrait {
    fn find(&self, tag: &str) -> Vec<Entity>;
    fn find_single(&self, tag: &str) -> Result<Entity, FindSingleError>;
}

impl LdtkEntitiesWithTagTrait for LdtkEntitiesWithTag<'_, '_> {
    /// Returns a list of all entities which contain LdtkEntity,
    /// and which contain the given tag
    fn find(&self, tag: &str) -> Vec<Entity> {
        self.iter()
            .filter_map(|(entity, ldtk_entity)| {
                if ldtk_entity.has_tag(tag) {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns Ok(Entity) only if there is exactly one entity
    /// which contains the given tag
    fn find_single(&self, tag: &str) -> Result<Entity, FindSingleError> {
        let matches = self.find(tag);
        match matches.len() {
            0 => Err(FindSingleError::NoEntities(tag.to_string())),
            1 => Ok(matches[0]),
            _ => Err(FindSingleError::MultipleEntities(tag.to_string())),
        }
    }
}
