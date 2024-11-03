use bevy_reflect::Reflect;

use crate::iid::Iid;

#[derive(Debug, Reflect)]
pub struct ProjectAssetPath {
    path: String,
}

impl ProjectAssetPath {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn to_layer_definition_asset_path(
        &self,
        layer_definition_identifier: &str,
    ) -> LayerDefinitionAssetPath {
        LayerDefinitionAssetPath {
            path: self.path.clone(),
            layer_definition_identifier: layer_definition_identifier.to_string(),
        }
    }

    pub fn to_tileset_definition_asset_path(
        &self,
        tileset_definition_identifier: &str,
    ) -> TilesetDefinitionAssetPath {
        TilesetDefinitionAssetPath {
            path: self.path.clone(),
            tileset_definition_identifier: tileset_definition_identifier.to_string(),
        }
    }

    pub fn to_world_asset_path(&self, world_identifier: &str) -> WorldAssetPath {
        WorldAssetPath {
            path: self.path.clone(),
            world_identifier: world_identifier.to_string(),
        }
    }
}

#[derive(Debug, Reflect)]
pub struct LayerDefinitionAssetPath {
    path: String,
    layer_definition_identifier: String,
}

impl LayerDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!(
            "{}#|layer_definition:{}",
            self.path, self.layer_definition_identifier
        )
    }
}

#[derive(Debug, Reflect)]
pub struct TilesetDefinitionAssetPath {
    path: String,
    tileset_definition_identifier: String,
}

impl TilesetDefinitionAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!(
            "{}#|tileset_definition:{}",
            self.path, self.tileset_definition_identifier
        )
    }
}

#[derive(Debug, Reflect)]
pub struct WorldAssetPath {
    path: String,
    world_identifier: String,
}

impl WorldAssetPath {
    pub fn to_level_asset_path(&self, level_identifier: &str) -> LevelAssetPath {
        LevelAssetPath {
            path: self.path.clone(),
            world_identifier: self.world_identifier.clone(),
            level_identifier: level_identifier.to_string(),
        }
    }

    pub fn to_asset_label(&self) -> String {
        self.world_identifier.clone()
    }
}

#[derive(Debug, Reflect)]
pub struct LevelAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
}

impl LevelAssetPath {
    pub fn to_layer_asset_path(&self, layer_identifier: &str) -> LayerAssetPath {
        LayerAssetPath {
            path: self.path.clone(),
            world_identifier: self.world_identifier.clone(),
            level_identifier: self.level_identifier.clone(),
            layer_identifier: layer_identifier.to_string(),
        }
    }
    pub fn to_asset_label(&self) -> String {
        format!("{}/{}", self.world_identifier, self.level_identifier)
    }
}

#[derive(Debug, Reflect)]
pub struct LayerAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
    layer_identifier: String,
}

impl LayerAssetPath {
    pub fn to_entity_asset_path(
        &self,
        entity_identifier: &str,
        entity_iid: Iid,
    ) -> EntityAssetPath {
        EntityAssetPath {
            path: self.path.clone(),
            world_identifier: self.world_identifier.clone(),
            level_identifier: self.level_identifier.clone(),
            layer_identifier: self.layer_identifier.clone(),
            entity_identifier: entity_identifier.to_string(),
            entity_iid,
        }
    }
    pub fn to_asset_label(&self) -> String {
        format!(
            "{}/{}/{}",
            self.world_identifier, self.level_identifier, self.layer_identifier
        )
    }
}

#[derive(Debug, Reflect)]
pub struct EntityAssetPath {
    path: String,
    world_identifier: String,
    level_identifier: String,
    layer_identifier: String,
    entity_identifier: String,
    entity_iid: Iid,
}

impl EntityAssetPath {
    pub fn to_asset_label(&self) -> String {
        format!(
            "{}/{}/{}/{}@{}",
            self.world_identifier,
            self.level_identifier,
            self.layer_identifier,
            self.entity_identifier,
            self.entity_iid
        )
    }
}
