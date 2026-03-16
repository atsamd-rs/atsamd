use super::{AdcInstance, channel::*};
use crate::{gpio::AnyPin, typelevel::Sealed};
use atsamd_hal_macros::hal_cfg;
use core::marker::PhantomData;
use num_traits::int::PrimInt;

#[hal_cfg(any("adc-d11", "adc-d21"))]
use crate::pac::adc as adc0;
#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

/// Trait for positive ADC channels
pub trait PosChannel<I: AdcInstance>: Sealed {
    const MUXVAL: adc0::inputctrl::Muxposselect;

    fn get_channel() -> Self;
}

/// Trait for negative ADC channels
pub trait NegChannel<I: AdcInstance>: Sealed {
    const MUXVAL: adc0::inputctrl::Muxnegselect;

    fn get_channel() -> Self;
}

/// Marker trait for ADC pins which can be used as positive ADC inputs
pub trait PosAdcPin<I: AdcInstance>: AnyPin<Mode = crate::gpio::AlternateB> + Sealed {
    type Channel: PosChannel<I>;
}

/// Marker trait for ADC pins which can be used as negative ADC inputs
pub trait NegAdcPin<I: AdcInstance>: AnyPin<Mode = crate::gpio::AlternateB> + Sealed {
    type Channel: NegChannel<I>;
}

/// Marker trait representing [`PosChannel`]'s which measures one of the various
/// CPU voltages
pub trait CpuVoltageSource<I: AdcInstance>: PosChannel<I> {}

/// Sampling mode for the ADC
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SampleMode {
    SingleEnded,
    Differential,
}

