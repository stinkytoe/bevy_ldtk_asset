use bevy::prelude::*;

pub trait HasIdentifier {
    fn identifier(&self) -> &String;
}

pub trait Spawn {
    fn spawn(&self, commands: &mut Commands, entity: Entity);
}
