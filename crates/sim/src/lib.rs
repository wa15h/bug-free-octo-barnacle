//! The deterministic simulation every server runs (D3): no engine, clock, I/O, threads, or floats.

mod fixed;
mod rng;
pub use fixed::{Fixed, FixedError, SCALE};
pub use rng::Rng;
