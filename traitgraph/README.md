# Traitgraph

[![](https://docs.rs/traitgraph/badge.svg)](https://docs.rs/traitgraph)
![](https://github.com/sebschmi/abstract-datastructures-rs/workflows/Tests%20and%20Lints/badge.svg?branch=main)

A Rust crate to represent and operate on graphs.

The basic principle of this crate is to define all methods on traits, and then implement these for concrete graph representations.
The crate mainly builds on top of [petgraph](https://crates.io/crates/petgraph), and might hopefully be included into petgraph at some point.
