//! Generalized UART Pad Definitions
//!
//! This module provides a type‐level way to specify UART pads (RX, TX, optional RTS/CTS) for any SERCOM.
//! It defines an `InternalPads` struct that pairs a four‐slot pad container with role markers.
//! You can start from `InternalPads::default()` and then call `.rx(...).tx(...).io(...).clk(...).rts(...).cts(...)`
//! to assign each pad in turn. The compiler enforces valid RX/TX/RTS/CTS combinations (DIPO/TXPO) per target.
//!
//! Public aliases simplify common patterns:
//! - `Pads<RX, TX, RTS, CTS>` assigns RX→TX→RTS→CTS in that order.
//! - `PadsFromIds<S, RX, TX>` (for thumbv6m or thumbv7em) infers pad types from pin IDs for RX/TX.
//!
//! Internally, `ValidPads` for `InternalPads<P, Roles<RX, TX, CLK, RTS, CTS>>` computes:
//! - `RXPO` and `TXPO` constants (pin positions) via `Rxpo`/`Txpo`.
//! - The SPI/UART capability (Rx/Tx/Duplex).
//! - The associated `Sercom` type and optional `CTS` pad.

use crate::sercom::pads;
use crate::sercom::pads::ReplacePad;
use crate::sercom::uart::{AnyConfig, Capability, CharSize, Config};
use crate::sercom::{uart, IsPad, OptionalPad, Sercom};
use crate::typelevel::NoneT;
use crate::{impl_const, roles_type};

use atsamd_hal_macros::hal_cfg;
use core::marker::PhantomData;

//==============================================================================
// Type Aliases and Role Definitions
//==============================================================================

/// `DefaultPads` is the starting point: no pads assigned in any of the four slots.
/// Internally, it is `InternalPads<pads::Pads, Roles>`, where:
/// - `pads::Pads` is the raw four‐slot container for optional pads.
/// - `Roles` is the placeholder for role types (all default to `NoneT`).
type DefaultPads = InternalPads<pads::Pads, Roles>;

/// `AddRole<P, R, NP>` represents “take an existing `InternalPads<P, R>` and assign one more role
/// `R` to pad type `NP`. Internally, this uses `ReplacePad<NP>` to fill one slot in `P::Pads`,
/// and `ReplaceRole<R>` to fill one slot in `R::Roles`.
type AddRole<P, R, NP> = InternalPads<
    <<P as IsPads>::Pads as ReplacePad<NP>>::NewPads,
    <<P as IsPads>::Roles as ReplaceRole<R>>::NewRoles<NP>,
>;

/// Public alias: assign RX, then TX, then optional RTS, then optional CTS to `DefaultPads`.
/// Equivalent to chaining `.rx(RX).tx(TX).rts(RTS).cts(CTS)`.
pub type Pads<RX = NoneT, TX = NoneT, RTS = NoneT, CTS = NoneT> =
    AddRole<AddRole<AddRole<AddRole<DefaultPads, Rx, RX>, Tx, TX>, Rts, RTS>, Cts, CTS>;

/// For thumbv6m or thumbv7em targets, infer pad types from pin IDs for RX and TX.
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
pub type PadsFromIds<S, RX, TX> = Pads<
    <RX as crate::sercom::GetOptionalPad<S>>::Pad,
    <TX as crate::sercom::GetOptionalPad<S>>::Pad,
>;

// Define the five UART roles: Rx, Tx, Clk (unused), Rts, Cts.
roles_type!(Rx, Tx, Clk, Rts, Cts);

//==============================================================================
// InternalPads Definition
//==============================================================================

/// `InternalPads<P, R>` pairs:
/// - `P: pads::ValidPads`: a low‐level four‐slot pad container ensuring all chosen pins share a SERCOM.
/// - `R: IsRoles`: a five‐slot role container indicating which slot corresponds to `Rx`, `Tx`, `Clk`, `Rts`, or `Cts`.
///
/// The default for `R` is `Roles<NoneT, NoneT, NoneT, NoneT, NoneT>`, meaning no roles assigned yet.
pub struct InternalPads<P: pads::ValidPads, R: IsRoles = Roles<NoneT, NoneT, NoneT, NoneT, NoneT>> {
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
    /// Assign an RX pin `I` (role `Rx`)
    /// *Requires:* `P: ReplacePad<I>` (slot exists and yields ValidPads) and `R: ReplaceRole<Rx>`.
    pub fn rx<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Rx>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a TX pin `I` (role `Tx`)
    pub fn tx<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Tx>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a pin `I` to both `Rx` and `Tx` (half‐duplex)
    pub fn io<I: IsPad>(
        self,
        pin: I,
    ) -> InternalPads<P::NewPads, <R::NewRoles<I> as ReplaceRole<Tx>>::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Rx>,
        R::NewRoles<I>: ReplaceRole<Tx>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>().replace::<I>(),
        }
    }

    /// Assign a CLk pin `I` (role `Clk`)
    pub fn clk<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Clk>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign an RTS pin `I` (role `Rts`)
    pub fn rts<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Rts>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign a CTS pin `I` (role `Cts`)
    pub fn cts<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Cts>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }
}

//==============================================================================
// RXPO / TXPO Implementations
//==============================================================================

use crate::sercom::{Pad0, Pad1, Pad2, Pad3};

/// `Rxpo` trait provides the RXPO constant for a given pad number.
trait Rxpo {
    const RXPO: u8;
}

impl_const! {
    trait = Rxpo;
    field = const RXPO: u8;
    NoneT => 0,
    Pad0  => 0,
    Pad1  => 1,
    Pad2  => 2,
    Pad3  => 3,
}

/// `Txpo` trait provides the TXPO constant (which depends on RX/TX/CLK/RTS/CTS combination).
trait Txpo {
    const TXPO: u8;
}

#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
impl_const! {
    trait = Txpo;
    field = const TXPO: u8;
    (NoneT, NoneT, NoneT, NoneT) => 0,
    (NoneT, Pad1, NoneT, NoneT) => 0,
    (Pad0, NoneT, NoneT, NoneT) => 0,
    (Pad0, Pad1, NoneT, NoneT) => 0,
    (NoneT, Pad3, NoneT, NoneT) => 1,
    (Pad2, NoneT, NoneT, NoneT) => 1,
    (Pad2, Pad3, NoneT, NoneT) => 1,
    (NoneT, NoneT, Pad2, Pad3) => 2,
    (Pad0, NoneT, NoneT, Pad3) => 2,
    (Pad0, NoneT, Pad2, NoneT) => 2,
    (NoneT, NoneT, NoneT, Pad3) => 2,
    (NoneT, NoneT, Pad2, NoneT) => 2,
    (Pad0, NoneT, Pad2, Pad3) => 2,
}

#[hal_cfg("sercom0-d5x")]
impl_const! {
    trait = Txpo;
    field = const TXPO: u8;
    (NoneT, NoneT, NoneT, NoneT) => 0,
    (NoneT, Pad1, NoneT, NoneT) => 0,
    (Pad0, NoneT, NoneT, NoneT) => 0,
    (Pad0, Pad1, NoneT, NoneT) => 0,
    (NoneT, NoneT, NoneT, Pad3) => 2,
    (Pad0, NoneT, NoneT, Pad3) => 2,
    (NoneT, NoneT, Pad2, Pad3) => 2,
    (Pad0, NoneT, Pad2, Pad3) => 2,
    (Pad0, NoneT, Pad2, NoneT) => 3,
    (NoneT, NoneT, Pad2, NoneT) => 3,
    (NoneT, Pad1, Pad2, NoneT) => 3,
    (Pad0, Pad1, Pad2, NoneT) => 3,
}

//==============================================================================
// CapabilityRxTx Trait
//==============================================================================

/// Maps a pair `(RX, TX)` of pad roles (where either may be `NoneT`) to a UART `Capability`:
/// - `(RX, NoneT)` ⇒ `uart::Rx` (receive‐only)
/// - `(NoneT, TX)` ⇒ `uart::Tx` (transmit‐only)
/// - `(RX, TX)` ⇒ `uart::Duplex` (full‐duplex)
pub trait CapabilityRxTx {
    type Capability: Capability;
}

impl<RX: IsPad> CapabilityRxTx for (RX, NoneT) {
    type Capability = uart::Rx;
}

impl<TX: IsPad> CapabilityRxTx for (NoneT, TX) {
    type Capability = uart::Tx;
}

impl<RX: IsPad, TX: IsPad> CapabilityRxTx for (RX, TX) {
    type Capability = uart::Duplex;
}

//==============================================================================
// ValidPads for InternalPads
//==============================================================================

/// `ValidPads` for `InternalPads<P, Roles<RX, TX, CLK, RTS, CTS>>` ensures:
/// - `RX::PadNum: Rxpo` provides a valid RXPO value.
/// - `(TX::PadNum, CLK::PadNum, RTS::PadNum, CTS::PadNum): Txpo` provides a valid TXPO value.
/// - `(RX, TX): CapabilityRxTx` yields a valid UART capability.
/// - `P: pads::ValidPads` ensures all pads share one SERCOM.
///
/// Computes:
/// - `const RXPO`
/// - `const TXPO`
/// - `type Capability`
/// - `type Sercom`
/// - `type CTS`
///
/// Downstream code uses this to instantiate `Config<P, C>` for UART.
pub trait ValidPads {
    const RXPO: u8;
    const TXPO: u8;
    type Capability: Capability;
    type Sercom: Sercom;
    type CTS: OptionalPad;
}

impl<
        P: pads::ValidPads,
        RX: OptionalPad,
        TX: OptionalPad,
        CLK: OptionalPad,
        RTS: OptionalPad,
        CTS: OptionalPad,
    > ValidPads for InternalPads<P, Roles<RX, TX, CLK, RTS, CTS>>
where
    RX::PadNum: Rxpo,
    (TX::PadNum, CLK::PadNum, RTS::PadNum, CTS::PadNum): Txpo,
    (RX, TX): CapabilityRxTx,
{
    const RXPO: u8 = RX::PadNum::RXPO;
    const TXPO: u8 = <(TX::PadNum, CLK::PadNum, RTS::PadNum, CTS::PadNum)>::TXPO;
    type Capability = <(RX, TX) as CapabilityRxTx>::Capability;
    type Sercom = P::Sercom;
    type CTS = CTS;
}

//==============================================================================
// ValidConfig for UART Configurations
//==============================================================================

/// Marker trait for valid UART `Config<P, C>` instances.
/// A functional UART requires at least RX or TX present.
pub trait ValidConfig: AnyConfig {}

impl<P: ValidPads, C: CharSize> ValidConfig for Config<P, C> {}
