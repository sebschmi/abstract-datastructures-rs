# Traitgraph

[![](https://docs.rs/traitgraph/badge.svg)](https://docs.rs/traitgraph)
![](https://github.com/sebschmi/abstract-datastructures-rs/workflows/Tests%20and%20Lints/badge.svg?branch=main)

A Rust crate to represent graphs.

The basic principle of this crate is to define all methods on traits, and then implement these for concrete graph representations.
Currently, only [petgraph](https://crates.io/crates/petgraph) is supported as a representation.
