use bevy_utils::{HashMap, HashSet};

pub type Uid = i64;

pub type UidMap<T> = HashMap<Uid, T>;
pub type UidSet = HashSet<Uid>;
