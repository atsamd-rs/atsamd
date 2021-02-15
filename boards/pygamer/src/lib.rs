//! `pygamer` is a Board Support Package (BSP) which provides a type-safe API
//! for the Adafruit [Pygamer].
//!
//! This crate is essentially a thin wrapper for [atsamd-hal], and re-exports it
//! along with some of its members.
//!
//! Instead of interacting with the hal pin numbers, [Pins] lets you use
//! descriptive labels printed on the board like d13 or neopixel.
//! ```ignore
//! let mut pins = Pins::new(peripherals.PORT);
//! let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
//! ```
//!
//! The `split()` fn goes further and returns a [Sets] struct which you interact
//! with groups of pins like a bus or peripheral and offers an `init()` fn.
//! ```ignore
//! let mut sets = Pins::new(peripherals.PORT).split();
//! let timer = SpinTimer::new(4);
//! let mut neopixel = sets.neopixel.init(timer, &mut sets.port);
//! ```
//!
//! Visit Adafruit for an [overview] of pins and connectors and check out the
//! [examples] to get an idea of how applications are structured and how the API
//! works. Finally, install [hf2] to enable `cargo run` support for single
//! command build and upload.
//!
//! [Pygamer]: https://www.adafruit.com/product/4242
//! [atsamd-hal]: https://docs.rs/atsamd-hal/latest/atsamd_hal/
//! [overview]: https://learn.adafruit.com/adafruit-pygamer
//! [examples]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/examples
//! [hf2]: https://github.com/jacobrosenthal/hf2-rs/tree/master/hf2-cli

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

#[cfg(feature = "panic_led")]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use embedded_hal::digital::v2::OutputPin;

    let peripherals = unsafe { crate::pac::Peripherals::steal() };
    let mut pins = Pins::new(peripherals.PORT);
    let _ = pins.d13.into_open_drain_output(&mut pins.port).set_high();

    loop {
        cortex_m::asm::udf()
    }
}
