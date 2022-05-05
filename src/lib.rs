#![allow(dead_code)]

#[cfg(feature = "draw2d")]
pub mod draw2d;

/// Parser for the ULTRAKILL cyber grind pattern (cgp) format
pub mod parser;

/// Utilities for drawing patterns (extracting a colour from height information)
pub mod util;
