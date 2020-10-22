//! The prelude.
//!
//! To use wio-terminal effectively, a number of traits and types need to be
//! imported. Instead of importing them each manually, the prelude contains the
//! most commonly used imports.
//!
//! This can be imported as `use wio_terminal::prelude::*`.

pub use atsamd_hal::prelude::*;
pub use lis3dh::accelerometer::{Accelerometer, RawAccelerometer};
