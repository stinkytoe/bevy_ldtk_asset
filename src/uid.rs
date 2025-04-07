//! A unique identifier used in LDtk to reference other assets.
//!
//! This is being phased out and being replaced by [crate::iid::Iid].
//!
//! See: [Unique Identifiers](https://ldtk.io/docs/game-dev/json-overview/unique-identifiers/)

#![allow(missing_docs)]

use bevy_platform_support::collections::{HashMap, HashSet};

pub type Uid = i64;

pub type UidMap<T> = HashMap<Uid, T>;
pub type UidSet = HashSet<Uid>;
