//! Compile time optimized maps and sets.
//!
//! Keys can be string literals, byte string literals, byte literals, char
//! literals, or any of the fixed-size isizeegral types.
#![doc(html_root_url="https://sfackler.github.io/doc")]
#![feature(core)]
#![warn(missing_docs)]
#![no_std]

#[macro_use]
extern crate core;
extern crate phf_shared;

pub use phf_shared::PhfHash;
#[doc(inline)]
pub use map::Map;
#[doc(inline)]
pub use set::Set;
#[doc(inline)]
pub use ordered_map::OrderedMap;
#[doc(inline)]
pub use ordered_set::OrderedSet;

pub mod map;
pub mod set;
pub mod ordered_map;
pub mod ordered_set;

mod std {
    pub use core::fmt;
}

