//! This module provides target specific integration with the [`mcan`] crate.
//!
//! MCAN is an off-the-shelf peripheral that is synthesized and integrated
//! into, among the others, E51/E54 MCUs.
//!
//! Instance of the [`Dependencies`] struct is necessary in order to make `mcan`
//! abstractions operational.
//!
//! More information regarding the `mcan` API can be found in its
//! documentation.
//!
//! [`mcan`]: https://crates.io/crates/mcan
use crate::{
    clock::v2::{
        ahb::{AhbClk, AhbId},
        gclk::{EnabledGclk0, GclkSourceId},
        pclk::{Pclk, PclkId, PclkSourceId},
        types::Can0,
    },
    gpio::*,
    typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement, Sealed},
};
use atsamd_hal_macros::hal_cfg;

#[hal_cfg("can1")]
use crate::clock::v2::types::Can1;

use mcan_core::CanId;
use mcan_core::fugit::HertzU32;

/// Struct enclosing all the dependencies required to bootstrap `ID` instance of
/// MCAN.
///
/// Its construction means that all platform-specific prerequisites are in place
/// and `mcan` abstractions are operational.
pub struct Dependencies<ID: PclkId + AhbId, PS: PclkSourceId, RX, TX, CAN> {
    pclk: Pclk<ID, PS>,
    host_freq: HertzU32,
    ahbclk: AhbClk<ID>,
    rx: RX,
    tx: TX,
    can: CAN,
}

impl<ID: PclkId + AhbId, PS: PclkSourceId, RX, TX, CAN> Dependencies<ID, PS, RX, TX, CAN> {
    /// Create an instance of `Dependencies` struct.
    ///
    /// This struct implements [`mcan_core::Dependencies`] trait, making it
    /// possible to construct an instance of `mcan::bus::CanConfigurable`.
    pub fn new<I: GclkSourceId, S: Increment>(
        gclk0: EnabledGclk0<I, S>,
        pclk: Pclk<ID, PS>,
        ahbclk: AhbClk<ID>,
        rx: RX,
        tx: TX,
        can: CAN,
    ) -> (Self, EnabledGclk0<I, S::Inc>) {
        let host_freq = pclk.freq();
        (
            Self {
                pclk,
                host_freq,
                ahbclk,
                rx,
                tx,
                can,
            },
            gclk0.inc(),
        )
    }
    /// Destroy an instance of `Dependencies` struct.
    ///
    /// Releases all enclosed objects back to the user.
    #[allow(clippy::type_complexity)]
    pub fn free<I: GclkSourceId, S: Decrement>(
        self,
        gclk0: EnabledGclk0<I, S>,
    ) -> (
        EnabledGclk0<I, S::Dec>,
        Pclk<ID, PS>,
        HertzU32,
        AhbClk<ID>,
        RX,
        TX,
        CAN,
    ) {
        let Self {
            pclk,
            host_freq,
            ahbclk,
            rx,
            tx,
            can,
        } = self;
        (gclk0.dec(), pclk, host_freq, ahbclk, rx, tx, can)
    }
}

unsafe impl<ID, PS, RX, TX, CAN> mcan_core::Dependencies<ID> for Dependencies<ID, PS, RX, TX, CAN>
where
    ID: CanId + PclkId + AhbId,
    PS: PclkSourceId,
    RX: RxPin<ValidFor = ID>,
    TX: TxPin<ValidFor = ID>,
    CAN: OwnedPeripheral<Represents = ID>,
{
    fn host_clock(&self) -> HertzU32 {
        self.host_freq
    }

    fn can_clock(&self) -> HertzU32 {
        self.pclk.freq()
    }

    fn eligible_message_ram_start(&self) -> *const () {
        0x2000_0000 as _
    }
}

unsafe impl CanId for Can0 {
    const ADDRESS: *const () = crate::pac::Can0::PTR as *const _;
}

#[hal_cfg("can1")]
unsafe impl CanId for Can1 {
    const ADDRESS: *const () = crate::pac::Can1::PTR as *const _;
}

/// Trait representing a CAN peripheral
pub trait OwnedPeripheral: Sealed {
    type Represents: CanId;
}

impl Sealed for crate::pac::Can0 {}
impl OwnedPeripheral for crate::pac::Can0 {
    type Represents = Can0;
}

#[hal_cfg("can1")]
impl Sealed for crate::pac::Can1 {}
#[hal_cfg("can1")]
impl OwnedPeripheral for crate::pac::Can1 {
    type Represents = Can1;
}

/// Trait implemented on pins that can be set as RX pins for CAN
pub trait RxPin: Sealed {
    type ValidFor: CanId;
}

/// Trait implemented on pins that can be set as TX pins for CAN
pub trait TxPin: Sealed {
    type ValidFor: CanId;
}

impl RxPin for Pin<PA23, AlternateI> {
    type ValidFor = Can0;
}

impl RxPin for Pin<PA25, AlternateI> {
    type ValidFor = Can0;
}

impl TxPin for Pin<PA22, AlternateI> {
    type ValidFor = Can0;
}

impl TxPin for Pin<PA24, AlternateI> {
    type ValidFor = Can0;
}

#[hal_cfg("can1")]
impl RxPin for Pin<PB13, AlternateH> {
    type ValidFor = Can1;
}

#[hal_cfg("can1")]
impl RxPin for Pin<PB15, AlternateH> {
    type ValidFor = Can1;
}

#[hal_cfg("can1")]
impl TxPin for Pin<PB12, AlternateH> {
    type ValidFor = Can1;
}

#[hal_cfg("can1")]
impl TxPin for Pin<PB14, AlternateH> {
    type ValidFor = Can1;
}
