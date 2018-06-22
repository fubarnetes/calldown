//! This module implements wrappers around the zfs commands

mod zpool;
mod dataset;

pub use self::zpool::*;
pub use self::dataset::*;
