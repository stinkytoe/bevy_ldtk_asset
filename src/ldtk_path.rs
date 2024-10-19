use path_clean::PathClean;
use std::path::{Path, PathBuf};

pub(crate) fn ldtk_path_to_bevy_path(
    base_directory: &Path,
    ldtk_path: impl AsRef<Path>,
) -> PathBuf {
    base_directory.join(ldtk_path).clean()
}
