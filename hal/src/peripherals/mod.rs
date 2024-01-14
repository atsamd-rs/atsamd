#[cfg(all(feature = "unproven", feature = "has-adc-variant1"))]
#[path = "adc/variant1.rs"]
pub mod adc;

#[cfg(all(feature = "unproven", feature = "has-adc-variant2"))]
#[path = "adc/variant2.rs"]
pub mod adc;

mod serial_number;
pub use serial_number::*;
