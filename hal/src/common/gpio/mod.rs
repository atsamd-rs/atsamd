//! GPIO pins
//!
//! # Versions
//!
//! There are currently two versions of the GPIO module. The inital GPIO API
//! was based on a macro-heavy implementation. The discussion in issue
//! [#214](https://github.com/atsamd-rs/atsamd/issues/214) spurred the creation
//! of a new module with less macro-use and a refactored API.
//!
//! The new module is provided in `v2`. The old module was removed, but a
//! compatibility shim is provided in `v1` to support existing code.
//!
//! # Migration
//!
//! The `v2` module will eventually replace `v1`. New users are encouraged to
//! use `v2` instead of `v1`. Existing code should expect to migrate to `v2`
//! before this crate reaches `1.0`.
//!
//! # `gpio::v2`
//!
//! The new API provides two different submodules, `v2::pin` and `v2::dynpin`,
//! representing two different ways to handle GPIO pins. The default, `v2::pin`,
//! is a type-level API that tracks the state of each pin at compile-time. The
//! alternative, `v2::dynpin` is a type-erased, value-level API that tracks the
//! state of each pin at run-time.
//!
//! The type-level API is strongly preferred. By representing the state of each
//! pin within the type system, the compiler can detect logic errors at
//! compile-time. Furthermore, the type-level API has absolutely zero run-time
//! cost. As a result, `v2::pin` is re-exported as the default `v2` API.
//!
//! If needed, `v2::dynpin` can be used to erase the type-level differences
//! between pins. However, by doing so, pins must now be tracked at run-time,
//! and each pin has a non-zero memory footprint.

pub mod v1;
pub use v1::*;

pub mod v2;
