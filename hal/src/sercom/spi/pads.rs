//! Generalized SPI Pad Definitions
//!
//! This module provides a flexible, role‐based way to specify SPI pads for any SERCOM
//! peripheral. It defines an `InternalPads` struct that pairs a set of pad types (MOSI/MISO/SCLK/SS)
//! with their corresponding roles. You can start from `InternalPads::default()` and then call
//! `.data_in(...).data_out(...).sclk(...).ss(...)` to assign each pad in turn. The compiler
//! enforces that all chosen pads share a valid SERCOM (and correct DIPO/DOPO settings per target).
//!
//! Finally, the `roles_type!` macro is used to generate role markers (`Di`, `Do`, `Ck`, `Ss`) and
//! an associated `ReplaceRole` implementation, so that you can fill in roles one by one.

use atsamd_hal_macros::hal_cfg;

use crate::sercom::pad::{IsPad, OptionalPad};
use crate::sercom::pads::ReplacePad;
use crate::sercom::spi::{self, Capability};
use crate::sercom::{pads, Sercom};
use crate::typelevel::NoneT;
use crate::{impl_const, roles_type};
use core::marker::PhantomData;

//=============================================================================
// Type Aliases and Role Definitions
//=============================================================================

/// `DefaultPads` is the starting point: no pads assigned in any of the four slots.
///
/// Internally, it is `InternalPads<pads::Pads, Roles>`, where:
/// - `pads::Pads` is the raw 4‐slot container for optional pads.
/// - `Roles` is the placeholder for role types (all default to `NoneT`).
type DefaultPads = InternalPads<pads::Pads, Roles>;

/// `AddRole<P, R, NP>` represents “take an existing `InternalPads<P, R>` and assign one more role
/// `R` to pad type `NP`. Internally, it uses `ReplacePad<NP>` to fill one slot in `P::Pads`,
/// and `ReplaceRole<R>` to fill one slot in `R::Roles`.
type AddRole<P, R, NP> = InternalPads<
    <<P as IsPads>::Pads as ReplacePad<NP>>::NewPads,
    <<P as IsPads>::Roles as ReplaceRole<R>>::NewRoles<NP>,
>;

/// Public alias: assign MISO (`Di`), then MOSI (`Do`), then SCLK (`Ck`) to `DefaultPads`.
pub type Pads<DI = NoneT, DO = NoneT, CK = NoneT> =
    AddRole<AddRole<AddRole<DefaultPads, Di, DI>, Do, DO>, Ck, CK>;

// Define the four SPI roles: Data In (MISO), Data Out (MOSI), Clock (SCLK), and Slave Select (SS).
roles_type!(Di, Do, Ck, Ss);

//=============================================================================
// InternalPads Definition
//=============================================================================

/// `InternalPads<P, R>` packages:
/// - `P: pads::ValidPads`: a low‐level 4‐slot pad container ensuring all chosen pins share one SERCOM.
/// - `R: IsRoles`: a 4‐slot role container indicating which slot corresponds to `Di`, `Do`, `Ck`, or `Ss`.
///
/// The default for `R` is `Roles<NoneT, NoneT, NoneT, NoneT>`, meaning no roles assigned yet.
pub struct InternalPads<P: pads::ValidPads, R: IsRoles = Roles<NoneT, NoneT, NoneT, NoneT>> {
    pads: P,
    roles: R,
}

/// Type class: any type implementing `IsPads` exposes two associated types:
/// - `Pads`: the underlying pad container (`pads::ValidPads`)
/// - `Roles`: the associated roles (`IsRoles`)
pub trait IsPads {
    type Pads: pads::ValidPads;
    type Roles: IsRoles;
}

/// Blanket implementation of `IsPads` for `InternalPads<P, R>`.
impl<P: pads::ValidPads, R: IsRoles> IsPads for InternalPads<P, R> {
    type Pads = P;
    type Roles = R;
}

/// `Default` for `InternalPads` produces an all‐`NoneT` pad/role combination.
/// That is, no pad and no role has been assigned yet.
impl Default for InternalPads<pads::Pads<NoneT, NoneT, NoneT, NoneT>> {
    fn default() -> Self {
        InternalPads {
            pads: pads::Pads::default(),
            roles: Roles::default(),
        }
    }
}

impl<P: pads::ValidPads, R: IsRoles> InternalPads<P, R> {
    /// Assign a MISO pin `I` (role `Di`)
    /// *Requires:* `P: ReplacePad<I>` (slot exists and yields ValidPads) and `R: ReplaceRole<Di>`.
    /// Returns a new `InternalPads<P::NewPads, R::NewRoles<I>>`.
    pub fn data_in<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Di>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a MOSI pin `I` (role `Do`)
    pub fn data_out<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Do>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a SCLK pin `I` (role `Ck`)
    pub fn sclk<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Ck>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a Slave‐Select pin `I` (role `Ss`)
    pub fn ss<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Ss>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }
}

//=============================================================================
// CapabilityRxTx Trait
//=============================================================================

/// Maps a pair `(RX, TX)` of pad roles (where either may be `NoneT`) to an SPI `Capability`:
/// - `(RX, NoneT)` ⇒ `spi::Rx` (receive‐only)
/// - `(NoneT, TX)` ⇒ `spi::Tx` (transmit‐only)
/// - `(RX, TX)` ⇒ `spi::Duplex` (full‐duplex)
pub trait CapabilityRxTx {
    type Capability: Capability;
}

impl<RX: IsPad> CapabilityRxTx for (RX, NoneT) {
    type Capability = spi::Rx;
}

impl<TX: IsPad> CapabilityRxTx for (NoneT, TX) {
    type Capability = spi::Tx;
}

impl<RX: IsPad, TX: IsPad> CapabilityRxTx for (RX, TX) {
    type Capability = spi::Duplex;
}

//=============================================================================
// DIPO / DOPO Implementations
//=============================================================================

/// `Dipo` is a marker trait providing the DIPO constant
trait Dipo {
    const DIPO: u8;
}

use crate::sercom::{Pad0, Pad1, Pad2, Pad3};

impl_const! {
    trait = Dipo;
    field = const DIPO: u8;
    NoneT => 0,
    Pad0  => 0,
    Pad1  => 1,
    Pad2  => 2,
    Pad3  => 3,
}

/// `Dopo` is a marker trait providing the DOPO constant
trait Dopo {
    const DOPO: u8;
}

#[hal_cfg("sercom0-d5x")]
impl_const! {
    trait = Dopo;
    field = const DOPO: u8;
    (NoneT, NoneT, NoneT) => 0,
    (Pad0,  Pad1,  NoneT)  => 0,
    (Pad0,  Pad1,  Pad2)   => 0,
    (Pad3,  Pad1,  NoneT)  => 2,
    (Pad3,  Pad1,  Pad2)   => 2,
}

#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
impl_const! {
    trait = Dopo;
    field = const DOPO: u8;
    (Pad0, Pad1, Pad2) => 0,
    (Pad0, Pad1, NoneT) => 0,
    (Pad2, Pad3, Pad1) => 0,
    (Pad2, Pad3, NoneT) => 0,
    (Pad3, Pad1, Pad2) => 0,
    (Pad3, Pad1, NoneT) => 0,
    (Pad0, Pad3, Pad1) => 0,
    (Pad0, Pad3, NoneT) => 0,
}

//=============================================================================
// ValidPads for InternalPads
//=============================================================================

/// `ValidPads` for `InternalPads<P, Roles<DI, DO, CK, SS>>` ensures:
/// - `(DI, DO): CapabilityRxTx` ⇒ at least one of RX/TX is present
/// - `DI::PadNum: Dipo` ⇒ a valid DIPO value
/// - `(DO::PadNum, CK::PadNum, SS::PadNum): Dopo` ⇒ a valid DOPO value
///
/// This consolidates pad‐role validity and computes:
/// - `Sercom = P::Sercom`
/// - `Capability = <(DI, DO) as CapabilityRxTx>::Capability`
/// - `SS` type as the chosen `OptionalPad` for SS
/// - `DIPO_DOPO` as `(DIPO, DOPO)`
pub trait ValidPads {
    type Sercom: Sercom;
    type Capability: Capability;
    type SS: OptionalPad;
    const DIPO_DOPO: (u8, u8);
}

impl<P: pads::ValidPads, DI: OptionalPad, DO: OptionalPad, CK: OptionalPad, SS: OptionalPad>
    ValidPads for InternalPads<P, Roles<DI, DO, CK, SS>>
where
    (DI, DO): CapabilityRxTx,
    DI::PadNum: Dipo,
    (DO::PadNum, CK::PadNum, SS::PadNum): Dopo,
{
    type Sercom = P::Sercom;
    type Capability = <(DI, DO) as CapabilityRxTx>::Capability;
    type SS = SS;
    const DIPO_DOPO: (u8, u8) = (
        DI::PadNum::DIPO,
        <(DO::PadNum, CK::PadNum, SS::PadNum)>::DOPO,
    );
}
