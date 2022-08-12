#![deny(missing_docs)]

//! `devotee` is small visualization engine based on [pixels](https://crates.io/crates/pixels), [winit](https://crates.io/crates/winit) and [rodio](https://crates.io/crates/rodio).
//! It aims to provide minimalist visualization platform.

/// Application is the visualization core.
/// It provides basic event loop handling.
pub mod app;
/// Math is dedicated to basic concepts like Vectors.
pub mod math;
/// Node is the block of the `devotee` project.
pub mod node;
/// Set of visualization primitives.
pub mod visual;
