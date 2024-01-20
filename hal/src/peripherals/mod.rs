#[cfg(feature = "unproven")]
atsamd_hal_macros::hal_module_mapping!(
    pub adc,
    "adc-d11 adc-d21" => "adc/d11.rs",
    "adc-d5x" => "adc/d5x.rs",
);

mod serial_number;
pub use serial_number::*;
