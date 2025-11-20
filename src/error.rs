#![allow(missing_docs)]

use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum LdtkError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    ReadAssetBytesError(#[from] bevy_asset::ReadAssetBytesError),

    // TODO: can this be improved?
    #[error("poison error!")]
    PoisonError,

    #[error("Failure importing ldtk file! {0}")]
    LdtkImportError(String),

    #[error("Duplicate Iid error! {0}")]
    DuplicateIidError(crate::iid::Iid),
}

impl<T> From<PoisonError<T>> for LdtkError {
    fn from(_value: PoisonError<T>) -> Self {
        LdtkError::PoisonError
    }
}

#[macro_export]
macro_rules! ldtk_import_error {
    ($($args:tt)*) => {
        $crate::error::LdtkError::LdtkImportError(format!($($args)*))
    };
}
