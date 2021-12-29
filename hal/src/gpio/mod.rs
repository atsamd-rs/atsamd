//! # GPIO
//!
//! The GPIO module is used to configure GPIO pins through the
//! [PORT](crate::pac::PORT) interface.
//!
//! ## Versions
//!
//! The inital GPIO API used a macro-heavy implementation, and it contained a
//! few mistakes. The discussion in issue [#214](https://github.com/atsamd-rs/atsamd/issues/214)
//! spurred the creation of a new module with fewer macros and a corrected,
//! refactored API.
//!
//! The GPIO module has been completely rewritten (the `v2` module in
//! pre-`0.15.0` HAL versions). The old module (`v1`) was removed in HAL version
//! `0.15.0`.
//!
//! ## Features
//!
//! - Converting between pin modes no longer requires access to the [`Port`]
//!   type.
//!
//! As a consequence, pin mode conversions can now be implemented using
//! [`From`]/[`Into`].
//! ```
//! let pins = Pins::new(peripherals.PORT);
//! let pa08: Pin<PA08, PushPullOutput> = pins.pa08.into();
//! ```
//!
//! - Defines a new [`AnyPin`] trait that can be used to simplify function
//!   arguments and reduce the number of type parameters required when dealing
//!   with GPIO pins. See the [`AnyPin`] documentation for more details.
//!
//! - Offers a type-erased, [`DynPin`] type, for run-time tracking of pins.
//! - Provides a new [`bsp_pins`] macro to help BSP authors provide meaningful
//!   names and type aliases for their GPIO pins.
//!
//! [`bsp_pins`]: crate::bsp_pins
//!
//! # Pin modules
//!
//! The API provides two different submodules, [`pin`] and [`dynpin`],
//! representing two different ways to handle GPIO pins. The default, [`pin`],
//! is a type-level API that tracks the state of each pin at compile-time. The
//! alternative, [`dynpin`] is a type-erased, value-level API that tracks the
//! state of each pin at run-time.
//!
//! The type-level API is strongly preferred. By representing the state of each
//! pin within the type system, the compiler can detect logic errors at
//! compile-time. Furthermore, the type-level API has absolutely zero run-time
//! cost.
//!
//! If needed, [`dynpin`] can be used to erase the type-level differences
//! between pins. However, by doing so, pins must now be tracked at run-time,
//! and each pin has a non-zero memory footprint.

pub mod pin;
pub use pin::*;

pub mod dynpin;
pub use dynpin::*;

mod reg;
