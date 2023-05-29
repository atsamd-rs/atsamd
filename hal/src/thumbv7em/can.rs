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
        gclk::Gclk0Id,
        pclk::{Pclk, PclkId, PclkSourceId},
        types::Can0,
        Source,
    },
    gpio::*,
    typelevel::{Decrement, Increment},
};

#[cfg(feature = "has-can1")]
use crate::clock::v2::types::Can1;

use mcan_core::fugit::HertzU32;
use mcan_core::CanId;

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
    pub fn new<S>(
        gclk0: S,
        pclk: Pclk<ID, PS>,
        ahbclk: AhbClk<ID>,
        rx: RX,
        tx: TX,
        can: CAN,
    ) -> (Self, S::Inc)
    where
        S: Source<Id = Gclk0Id> + Increment,
    {
        (
            Self {
                pclk,
                host_freq: gclk0.freq(),
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
    pub fn free<S>(self, gclk0: S) -> (Pclk<ID, PS>, HertzU32, AhbClk<ID>, RX, TX, CAN, S::Dec)
    where
        S: Source<Id = Gclk0Id> + Decrement,
    {
        let Self {
            pclk,
            host_freq,
            ahbclk,
            rx,
            tx,
            can,
        } = self;
        (pclk, host_freq, ahbclk, rx, tx, can, gclk0.dec())
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
    const ADDRESS: *const () = crate::pac::CAN0::PTR as *const _;
}

#[cfg(feature = "has-can1")]
unsafe impl CanId for Can1 {
    const ADDRESS: *const () = crate::pac::CAN1::PTR as *const _;
}

trait OwnedPeripheral {
    type Represents: CanId;
}

impl OwnedPeripheral for crate::pac::CAN0 {
    type Represents = Can0;
}

#[cfg(feature = "has-can1")]
impl OwnedPeripheral for crate::pac::CAN1 {
    type Represents = Can1;
}

trait RxPin {
    type ValidFor: CanId;
}

trait TxPin {
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

#[cfg(feature = "has-can1")]
impl RxPin for Pin<PB13, AlternateH> {
    type ValidFor = Can1;
}

#[cfg(feature = "has-can1")]
impl RxPin for Pin<PB15, AlternateH> {
    type ValidFor = Can1;
}

#[cfg(feature = "has-can1")]
impl TxPin for Pin<PB12, AlternateH> {
    type ValidFor = Can1;
}

#[cfg(feature = "has-can1")]
impl TxPin for Pin<PB14, AlternateH> {
    type ValidFor = Can1;
}
