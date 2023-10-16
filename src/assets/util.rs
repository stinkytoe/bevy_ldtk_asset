use std::path::Path;

use bevy::asset::AssetPath;

pub fn ldtk_file_to_asset_path<'a>(ldtk_file: &str, ldtk_path: &Path) -> AssetPath<'a> {
    AssetPath::new(ldtk_path.join(ldtk_file), None)
}
