//! Interface for handling Iid fields in LDtk
//!
//! This is a thin wrapper over the [uuid](https://crates.io/crates/uuid) crate, with
//! [Iid] being a re-export of [uuid::Uuid], and the [iid]! macro being a re-export of of
//! [uuid::uuid].
//!
//! I chose to wrap these values for two reasons:
//! * To match the nomenclature of LDtk (using Iid instead of Uuid as the name).
//! * I had originally written my own implementation, but switched to the superior [uuid] crate
//!   after I realized it offered better features while remaining compatable with LDtk.

#![allow(missing_docs)]

use bevy_platform::collections::HashMap;
use bevy_platform::collections::HashSet;

/// A re-export of the [uuid::uuid] macro.
pub use uuid::uuid as iid;

/// A re-export of the [uuid::Uuid] type.
pub use uuid::Uuid as Iid;

/// A [bevy_utils::HashMap] indexed by an [Iid], for convenience.
pub type IidMap<T> = HashMap<Iid, T>;

/// A [bevy_utils::HashSet] of [Iid]s, for convenience.
pub type IidSet = HashSet<Iid>;
