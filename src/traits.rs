use bevy::prelude::*;

pub trait HasIdentifier {
    fn identifier(&self) -> &String;
}

pub trait SpawnsEntities {
    fn spawn_entities(&self, commands: &mut Commands, entity: Entity);
}
