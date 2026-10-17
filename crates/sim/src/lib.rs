//! The deterministic sim every server runs: engine-free (D3), no clock, I/O, threads, or floats.

mod fixed;
mod rng;
mod state;
pub use fixed::{Fixed, FixedError, SCALE};
pub use rng::Rng;
pub use state::{tick, Event, Input, State, System, HEARTH_ORDER, SYSTEM_ORDER};
