//! A unique identifier used in LDtk to reference other assets.
//!
//! This is being phased out and being replaced by [crate::iid::Iid].

#![allow(missing_docs)]

use bevy_utils::{HashMap, HashSet};

pub type Uid = i64;

pub type UidMap<T> = HashMap<Uid, T>;
pub type UidSet = HashSet<Uid>;
