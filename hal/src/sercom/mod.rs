//! # Configure the SERCOM peripherals
//!
//! The SERCOM module is used to configure the SERCOM peripherals as USART, SPI
//! or I2C interfaces.
//!
//! ## Versions
//!
//! There are currently two versions of the SERCOM module. The inital SERCOM API
//! was based on a macro-heavy implementation. The discussion in issue
//! [#214](https://github.com/atsamd-rs/atsamd/issues/214) spurred the creation
//! of a new module with less macro-use and a refactored API.
//!
//! The new module is provided in [v2]. The old module was removed, but a
//! compatibility shim is provided in [v1] to support existing code.
//!
//! ## Migration
//!
//! The [v2] module will eventually replace [v1]. New users are encouraged to
//! use [v2] instead of [v1].
//!
//! The new [`v2::spi`] and [`v2::uart`] modules are substantially more
//! configurable and safe than the existing, [`v1::spi`] and [`v1::uart`]
//! modules.
//!
//! [`Pad`]: v2::pads::Pad

pub mod v1;
pub use v1::*;

pub mod v2;
