use bevy_asset::AssetPath;
use path_clean::PathClean;
use std::path::{Path, PathBuf};

pub fn ldtk_path_to_bevy_path(
    base_directory: &AssetPath<'_>,
    ldtk_path: impl AsRef<Path>,
) -> PathBuf {
    base_directory.path().join(ldtk_path).clean()
}
