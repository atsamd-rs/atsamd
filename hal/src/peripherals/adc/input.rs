use core::marker::PhantomData;
use atsamd_hal_macros::hal_cfg;
use crate::{
    gpio::AnyPin,
    typelevel::Sealed,
};
use super::{
    AdcInstance,
    channel::*,
};

#[hal_cfg(any("adc-d11", "adc-d21"))]
use crate::pac::adc as adc0;
#[hal_cfg("adc-d5x")]
use crate::pac::adc0;


/// Trait for positive ADC channels
pub trait PosChannel<I: AdcInstance>: Sealed {
    const MUXVAL: adc0::inputctrl::Muxposselect;
}

/// Trait for negative ADC channels
pub trait NegChannel<I: AdcInstance>: Sealed {
    const MUXVAL: adc0::inputctrl::Muxnegselect;
}

/// Marker trait for ADC pins which can be used as positive ADC inputs
pub trait PosAdcPin<I: AdcInstance, P: PosChannel<I>>: AnyPin<Mode = crate::gpio::AlternateB> + Sealed {}

/// Marker trait for ADC pins which can be used as negative ADC inputs
pub trait NegAdcPin<I: AdcInstance, N: NegChannel<I>>: AnyPin<Mode = crate::gpio::AlternateB> + Sealed {}

/// Sampling mode for the ADC
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SampleMode {
    SingleEnded,
    Differential,
}

/// Trait for ADC inputs
pub trait AdcInput<I: AdcInstance> {
    const SAMPLE_MODE: SampleMode;
    type Pos: PosChannel<I>;
    type Neg: NegChannel<I>;
}

/// Type representing a single ended input.
pub struct SingleEndedInput<I, P>
where
    I: AdcInstance,
    P: PosChannel<I>,
{
    adc: PhantomData<I>,
    pos: PhantomData<P>,
}

impl<I, P> AdcInput<I> for SingleEndedInput<I, P>
where
    I: AdcInstance,
    P: PosChannel<I>,
{
    const SAMPLE_MODE: SampleMode = SampleMode::SingleEnded;
    type Pos = P;
    /// Single ended inputs always referenced to internal ADC GND
    type Neg = GND<I>;
}

impl<I, P> SingleEndedInput<I, P>
where
    I: AdcInstance,
    P: PosChannel<I>,
{

    fn new() -> Self {
        Self {
            adc: PhantomData,
            pos: PhantomData,
        }
    }

    /// Creates a [`SingleEndedInput`] from a [`PosChannel`]
    pub fn from_channel(_pos: P) -> Self {
        Self::new()
    }

    /// Creates a [`SingleEndedInput`] from a [`PosAdcPin`]
    pub fn from_pin<PP: PosAdcPin<I, P>>(_pin: PP) -> Self {
        Self::new()
    }
}

/// Type representing a differential input
pub struct DifferentialInput<I, P, N>
where
    I: AdcInstance,
    P: PosChannel<I>,
    N: NegChannel<I>,
{
    adc: PhantomData<I>,
    pos: PhantomData<P>,
    neg: PhantomData<N>,
}

impl<I, P, N> AdcInput<I> for DifferentialInput<I, P, N>
where
    I: AdcInstance,
    P: PosChannel<I>,
    N: NegChannel<I>,
{
    const SAMPLE_MODE: SampleMode = SampleMode::Differential;
    type Pos = P;
    type Neg = N;
}

impl<I, P, N> DifferentialInput<I, P, N>
where
    I: AdcInstance,
    P: PosChannel<I>,
    N: NegChannel<I>,
{

    fn new() -> Self {
        Self {
            adc: PhantomData,
            pos: PhantomData,
            neg: PhantomData,
        }
    }

    /// Create a [`DifferentialInput`] from two ADC channels
    pub fn from_channels(_pos: P, _neg: N) -> Self {
        Self::new()
    }

    /// Create a [`DifferentialInput`] from two GPIO pins
    pub fn from_pins<PP, PN>(_pos: PP, _neg: PN) -> Self
    where
        PP: PosAdcPin<I, P>,
        PN: NegAdcPin<I, N>,
    {
        Self::new()
    }

    /// Create a [`DifferentialInput`] from a single GPIO pin and
    /// a negative ADC channel
    pub fn from_pos_pin<PP>(_pos: PP, _neg: N) -> Self
    where
        PP: PosAdcPin<I, P>,
    {
        Self::new()
    }
}
