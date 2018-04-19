#![no_std]
#![feature(never_type)]

pub extern crate atsamd21g18a;
extern crate cortex_m;
pub extern crate embedded_hal as hal;
extern crate nb;

mod calibration;
pub mod clock;
pub mod gpio;
pub mod prelude;
pub mod time;
pub mod timer;
