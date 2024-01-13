#![deny(missing_docs)]

//! `devotee` is small visualization engine based on [winit](https://crates.io/crates/winit) and [rodio](https://crates.io/crates/rodio).
//! It aims to provide minimalist visualization capabilities.
//!
//! To use `devotee` specify `Config`, create `Setup`, initialize `App` and `run` it.

pub use devotee_backend::winit;

/// Application is the visualization core.
/// It provides basic event loop handling.
pub mod app;
/// Set of random useful stuff.
pub mod util;
/// Set of visualization primitives.
pub mod visual;
