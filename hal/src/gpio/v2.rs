//! # Version 2 of the GPIO module
//!
//! The new API provides two different submodules, [`pin`] and [`dynpin`],
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
