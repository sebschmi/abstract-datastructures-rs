#![warn(missing_docs)]
//! This crate offers traits for graph datastructures as well as implementations of these traits.

pub use traitsequence;

/// Different implementations of the graph traits.
pub mod implementation;
/// Traits and a default implementation for graph indices.
pub mod index;
pub mod interface;
/// Traits and implementations of node- and edge-centric walks.
pub mod walks;
