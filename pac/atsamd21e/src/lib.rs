//! Peripheral access API for ATSAMD21E microcontrollers, generated with
//! `svd2rust`.
//!
//! You can find an overview of the generated API [here](https://docs.rs/svd2rust/0.37.1/svd2rust/#peripheral-api).

#![no_std]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/pac.rs"));

pub use __pac_impl::generic::*;
pub use __pac_impl::*;
