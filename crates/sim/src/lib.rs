//! The deterministic sim every server runs: engine-free (D3), no clock, I/O, threads, or floats.

mod fixed;
mod rng;
pub use fixed::{Fixed, FixedError, SCALE};
pub use rng::Rng;
