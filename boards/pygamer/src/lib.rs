#![no_std]
#![recursion_limit = "1024"]

pub mod pins;
use atsamd_hal as hal;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use pins::Pins;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

pub mod util {
    /// Analogous to Arduinos map function
    pub fn map_from(input: i16, from_range: (i16, i16), to_range: (i16, i16)) -> i16 {
        debug_assert!(from_range.0 < from_range.1);
        debug_assert!(to_range.0 < to_range.1);
        debug_assert!(input >= from_range.0);
        debug_assert!(input <= from_range.1);

        let from: f32 = (from_range.1 - from_range.0).into();
        let to: f32 = (to_range.1 - to_range.0).into();
        ((input - from_range.0) as f32 / from * to + to_range.0 as f32) as i16
    }
}
