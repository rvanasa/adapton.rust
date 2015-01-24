// #![feature(unsafe_destructor, macro_rules, phase, globs, default_type_params)]
#![feature(unboxed_closures)]
// #![feature(old_orphan_check)]

//! Adapton for Rust

/// A Thunk safe for single-threaded access.
// extern crate lazy;

pub mod name;

#[macro_use]
pub mod art;

pub mod balfp;

pub mod list;

pub mod challengeWhile;

mod adapton {
    pub use super::*;
}
