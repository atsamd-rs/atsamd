#![no_std]
#![recursion_limit = "1024"]

#[cfg(feature = "unproven")]
mod buttons;

// Re-export the HAL and the PAC to give the user lower-level access to the
// device should they need it.
pub use atsamd_hal::{self as hal, target_device as pac};

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

mod pins;
pub use pins::*;

pub mod prelude {
    pub use atsamd_hal::prelude::*;
}
