//! # GPIO
//!
//! The GPIO module is used to configure GPIO pins through the
//! [PORT](crate::target_device::PORT) interface.
//!
//! ## Versions
//!
//! There are currently two versions of the GPIO module. The inital GPIO API
//! used a macro-heavy implementation, and it contained a few mistakes. The
//! discussion in issue [#214](https://github.com/atsamd-rs/atsamd/issues/214)
//! spurred the creation of a new module with fewer macros and a corrected,
//! refactored API.
//!
//! The new module is provided in [`v2`]. The old module was removed, but a
//! compatibility shim is provided in [`v1`] to support existing code. Users
//! should expect to eventually migrate to [`v2`].
//!
//! ## Errors in [`v1`]
//!
//! [`v2`] fixes a number of errors in [`v1`]:
//!
//! - [`v1`] implements an open-drain output mode, but SAMD chips do not have
//!   open-drain outputs. There is (almost) no mention of "open-drain" anywhere
//!   in the datasheets. In fact, the SAMD21 datasheet notes a removal of the
//!   term in Rev. E. Open-drain outputs have been has been removed in [`v2`].
//! - [`v1`] allows users to enable a pull-up resistor while in an output mode,
//!   but this is not possible for SAMD chips. There is no corresponding entry
//!   in the "Pin Configurations Summary" table in any of the three datasheets.
//!   Moreover, when a pull resistor is enabled for inputs, its direction is set
//!   using the `OUT` bit, which would not be possible in an output mode.
//! - [`v1`] does not implement any of the disabled pin modes, i.e. when both
//!   `DIR` and `INEN` are `0`. As a consequence, the state of [`v1`] pins at
//!   reset is incorrect. [`v1`] assumes they are in floating input mode, but
//!   they are actually in floating disabled mode.
//!
//! ## New features
//!
//! The [`v2`] module has several new features:
//!
//! - Converting between pin modes no longer requires access to the [`Port`]
//!   type.
//!
//! For example, the follow code in [`v1`],
//! ```
//! let pins = peripherals.PORT.split();
//! let pa8 = pins.pa8.into_push_pull_output(&mut pins.port);
//! ```
//! would look like this in [`v2`].
//! ```
//! let pins = v2::Pins::new(peripherals.PORT);
//! let pa08 = pins.pa08.into_push_pull_output();
//! ```
//! As a consequence, pin mode conversions can now be implemented using
//! [`From`]/[`Into`].
//! ```
//! let pins = Pins::new(peripherals.PORT);
//! let pa08: Pin<PA08, PushPullOutput> = pins.pa08.into();
//! ```
//!
//! - [`v2`] defines a new [`AnyPin`] trait that can be used to simplify
//!   function arguments and reduce the number of type parameters required when
//!   dealing with GPIO pins.
//! - [`v2`] offers a type-erased, [`DynPin`] type, for run-time tracking of
//!   pins.
//! - [`v2`] provides a new [`bsp_pins`] macro to help BSP authors provide
//!   meaningful names and type aliases for their GPIO pins.
//!
//! ## Compatibility
//!
//! The original [`v1`] module has been removed. It has been replaced with a
//! compatibility shim to support existing code. The shim implements all [`v1`]
//! pin types in terms of [`v2::Pin`]. In fact, it declares its own [`v1::Pin`]
//! type as a newtype wrapper around a [`v2::Pin`], and it defines the
//! individual `Pa0`, `Pa1`, etc. pin types as type aliases of the new
//! [`v1::Pin`] type.
//!
//! As a consequence, it is easy to define the conversion between a [`v1::Pin`]
//! and its corresponding [`v2::Pin`] using [`From`]/[`Into`].
//! ```
//! let pins = peripherals.PORT.split();
//! let pa8: v1::Pa8<_> = pins.pa8;
//! let pa08 = v2::Pin<_, _> = pa8.into();
//! ```
//! Moreover, all [`v1::Pin`] and [`v2::Pin`] types implement the [`AnyPin`]
//! trait, which is particularly useful for supporting both module versions
//! simultaneously. See the [`AnyPin`] documentation for more details.
//! ```
//! /// Take the v1 or v2 representation of pin PA08, in any mode, then convert
//! /// it to a push-pull output and set it high
//! fn example(pin: impl AnyPin<Id = PA08>) {
//!     let mut pin = pin.into().into_push_pull_output();
//!     pin.set_high().ok();
//! }
//! ```
//! [`AnyPin`]: v2::AnyPin
//! [`DynPin`]: v2::DynPin
//! [`bsp_pins`]: crate::bsp_pins

pub mod v1;
pub use v1::*;

pub mod v2;
