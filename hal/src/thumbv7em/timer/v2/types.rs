use core::ops::Deref;

use crate::clock::v2::apb::ApbId;
use crate::clock::v2::pclk::PclkId;
use crate::clock::v2::types;
use crate::pac;

/// Any timer identifier (TimerIds + CombinedTimerIds)
pub trait AbstractTimerId {
    type Reg: core::ops::Deref<Target = crate::pac::tc0::RegisterBlock>;
}

/// "non-combined" timer identifier (Tc0, Tc1, ..., Tc7)
///
/// TimerId matches 1:1 ApbId; thus subtrait relation
pub trait TimerId: ApbId + AbstractTimerId {}

/// "combined" timer identifiers (Tc0Tc1, Tc2Tc3, ...)
/// CombinedTimerIds matches 1:1 PclkIds; thus subtrait relation
pub trait CombinedTimerId: PclkId + AbstractTimerId {
    type PrimaryTimer: PrimaryTimerId<CombinedTimer = Self>;
    type SecondaryTimer: TimerId;
}

/// "primary" timers are the even-numbered ones (Tc0, Tc2, Tc4, Tc6)
///
/// they usually drive the timer in a "paired" mode
pub trait PrimaryTimerId: TimerId {
    type CombinedTimer: CombinedTimerId<PrimaryTimer = Self>;
}

/// Marker trait for "primary" timers which support "paired" mode
pub trait PairableTimerId: PrimaryTimerId {}

macro_rules! impl_timer_traits {
    (
        $(
            $( #[$( $cfg:tt )+] )?
            (($t1:ty, $t1_reg:ty), ($t2:ty, $t2_reg:ty), $t3:ident)
        ),+$(,)?
    ) => {
        $(
        paste::paste! {
            $( #[$( $cfg )+] )?
            mod [<impls_for_pair_ $t3:snake >] {
                use super::{
                    AbstractTimerId, PrimaryTimerId,
                    CombinedTimerId, TimerId, CombinedTimerReg,
                    pac, types
                };
                use pac::{$t1_reg, $t2_reg};
                use types::{$t1, $t2, $t3};

                impl AbstractTimerId for $t1 {
                    type Reg = $t1_reg;
                }
                impl TimerId for $t1 {}

                impl PrimaryTimerId for $t1 {
                    type CombinedTimer = $t3;
                }

                impl AbstractTimerId for $t2 {
                    type Reg = $t2_reg;
                }
                impl TimerId for $t2 {
                }

                impl AbstractTimerId for $t3 {
                    // Allegedly when in a "paired" mode,
                    // HW reg access should be done through the primary timer
                    type Reg = CombinedTimerReg<$t1_reg, $t2_reg>;
                }
                impl CombinedTimerId for $t3 {
                    type PrimaryTimer = $t1;
                    type SecondaryTimer = $t2;
                }
            }
        }
        )+
    }
}

impl_timer_traits! {
    #[cfg(all(feature = "has-tc0", feature = "has-tc1"))]
    ((Tc0, TC0), (Tc1, TC1), Tc0Tc1),
    #[cfg(all(feature = "has-tc2", feature = "has-tc3"))]
    ((Tc2, TC2), (Tc3, TC3), Tc2Tc3),
    #[cfg(all(feature = "has-tc4", feature = "has-tc5"))]
    ((Tc4, TC4), (Tc5, TC5), Tc4Tc5),
    #[cfg(all(feature = "has-tc6", feature = "has-tc7"))]
    ((Tc6, TC6), (Tc7, TC7), Tc6Tc7),
}

#[cfg(all(feature = "has-tc0", feature = "has-tc1"))]
impl PairableTimerId for types::Tc0 {}
#[cfg(all(feature = "has-tc2", feature = "has-tc3"))]
impl PairableTimerId for types::Tc2 {}

/// Wrapper for a combined timer (Timer<T: CombinedTimerId, ..>) to hold
/// registers of inner timers.
///
/// Main purpose of it is to be able to return both of them on
/// deconstruction. Did not find a way to `core::mem::transmute(())` into
/// `T::Reg` because size of a type is not known
pub struct CombinedTimerReg<P, S> {
    pub primary_timer_reg: P,
    pub secondary_timer_reg: S,
}

impl<P, S> From<(P, S)> for CombinedTimerReg<P, S> {
    fn from(value: (P, S)) -> Self {
        Self {
            primary_timer_reg: value.0,
            secondary_timer_reg: value.1,
        }
    }
}

impl<P, S> From<CombinedTimerReg<P, S>> for (P, S) {
    fn from(value: CombinedTimerReg<P, S>) -> Self {
        (value.primary_timer_reg, value.secondary_timer_reg)
    }
}

impl<P: Deref, S> Deref for CombinedTimerReg<P, S> {
    type Target = P::Target;

    // HW register from the primary timer is used when in "paired" mode
    fn deref(&self) -> &Self::Target {
        self.primary_timer_reg.deref()
    }
}

pub mod state {
    pub trait State {}

    pub enum Disabled {}
    pub enum Enabled {}

    impl State for Disabled {}
    impl State for Enabled {}
}

pub mod timer_width {
    pub trait TimerWidth: Copy + Clone + num_traits::AsPrimitive<u32> {
        fn from_u32(v: u32) -> Self;
    }

    impl TimerWidth for u8 {
        fn from_u32(v: u32) -> Self {
            v as Self
        }
    }
    impl TimerWidth for u16 {
        fn from_u32(v: u32) -> Self {
            v as Self
        }
    }
    impl TimerWidth for u32 {
        fn from_u32(v: u32) -> Self {
            v as Self
        }
    }
}

#[derive(Copy, Clone)]
pub enum TimerDirection {
    Increment,
    Decrement,
}

pub type TimerPrescaler = crate::pac::tc0::count8::ctrla::PRESCALER_A;

#[derive(Copy, Clone, Debug)]
pub enum TimerError {
    NoValidPrescaler(TimerPrescalerError),
}

#[derive(Copy, Clone, Debug)]
pub enum TimerPrescalerError {
    InvalidPrescaler,
    FrequenciesNotDivisible,
    OutputFrequencyIsZero,
}

pub(super) type TimerCommand = crate::pac::tc0::count8::ctrlbset::CMD_A;

pub(super) trait WriteClosure<T: crate::pac::generic::Writable>:
    FnOnce(&mut T::Writer) -> &mut crate::pac::generic::W<T>
{
}

impl<T, C> WriteClosure<T> for C
where
    T: crate::pac::generic::Writable,
    C: FnOnce(&mut T::Writer) -> &mut crate::pac::generic::W<T>,
{
}

pub(super) type CtrlBSet = crate::pac::tc0::count8::ctrlbset::CTRLBSET_SPEC;
pub(super) type CtrlBClr = crate::pac::tc0::count8::ctrlbclr::CTRLBCLR_SPEC;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum TimerCompareRegister {
    Zero = 0,
    One = 1,
}

pub(super) type SecondaryTimer<T> =
    <<T as PrimaryTimerId>::CombinedTimer as CombinedTimerId>::SecondaryTimer;
pub(super) type Reg<T> = <T as AbstractTimerId>::Reg;
