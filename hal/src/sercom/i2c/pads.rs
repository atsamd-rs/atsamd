//! Generic I²C Pad Definitions
//!
//! This module provides a type‐level way to specify I²C pads for any SERCOM peripheral.
//! It defines an `InternalPads` struct that pairs a set of pad types (SDA and SCL) with
//! their corresponding I²C roles. You can start from `InternalPads::default()` and then
//! call `.sda(...).scl(...)` to assign each pad in turn. The compiler enforces that the
//! chosen SDA must be Pad0 and SCL must be Pad1 for a given SERCOM, ensuring valid I²C
//! pin assignments.
//!
//! The public alias `Pads<SDA, SCL>` is a shorthand for assigning SDA then SCL to an
//! all‐`NoneT` initial container. A convenience `InternalPads::new(sda, scl)` constructs
//! this in one step.

use crate::roles_type;

use crate::sercom::pad::{IsPad, OptionalPad};
use crate::sercom::pads::ReplacePad;
use crate::sercom::{pads, IsI2cPad, Sercom};
use crate::typelevel::NoneT;
use core::marker::PhantomData;

//==============================================================================
// Type Aliases and Role Definitions
//==============================================================================

/// `DefaultPads` is the starting point: no pads assigned (all four slots are `NoneT`).
/// Internally, it is `InternalPads<pads::Pads, Roles>`, where:
/// - `pads::Pads` is the raw four‐slot container for optional pads.
/// - `Roles` is the placeholder for role types (both default to `NoneT`).
type DefaultPads = InternalPads<pads::Pads, Roles>;

/// `AddRole<P, R, NP>` represents “take an existing `InternalPads<P, R>` and assign one more role
/// `R` to pad type `NP`. Internally, this uses `ReplacePad<NP>` to fill one slot in `P::Pads`,
/// and `ReplaceRole<R>` to fill one slot in `R::Roles`.
type AddRole<P, R, NP> = InternalPads<
    <<P as IsPads>::Pads as ReplacePad<NP>>::NewPads,
    <<P as IsPads>::Roles as ReplaceRole<R>>::NewRoles<NP>,
>;

/// Public alias: assign SDA (Pad0) then SCL (Pad1) to an all‐`NoneT` `DefaultPads`.
/// Equivalent to `InternalPads::default().sda(SDA).scl(SCL)`.
pub type Pads<SDA = NoneT, SCL = NoneT> = AddRole<AddRole<DefaultPads, Sda, SDA>, Scl, SCL>;

// Define the two I²C roles: SDA (data line) and SCL (clock line).
roles_type!(Sda, Scl);

//==============================================================================
// InternalPads Definition
//==============================================================================

/// `InternalPads<P, R>` pairs:
/// - `P: pads::ValidPads`: a low‐level four‐slot pad container ensuring all chosen pins share one SERCOM.
/// - `R: IsRoles`: a two‐slot role container indicating which slot corresponds to `Sda` or `Scl`.
///
/// The default for `R` is `Roles<NoneT, NoneT>`, meaning neither SDA nor SCL has been assigned yet.
pub struct InternalPads<
    P: pads::ValidPads = pads::Pads<NoneT, NoneT, NoneT, NoneT>,
    R: IsRoles = Roles<NoneT, NoneT>,
> {
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

/// `ValidPads` for `InternalPads<P, Roles<SDA, SCL>>` ensures:
/// - `SDA: IsI2cPad<PadNum = Pad0>`
/// - `SCL: IsI2cPad<PadNum = Pad1>`
///
/// In other words, SDA must be pad 0 and SCL must be pad 1 on the same SERCOM.
pub trait ValidPads {
    type Sercom: Sercom;
}

use crate::sercom::{Pad0, Pad1};

impl<P: pads::ValidPads, SDA: IsI2cPad<PadNum = Pad0>, SCL: IsI2cPad<PadNum = Pad1>> ValidPads
    for InternalPads<P, Roles<SDA, SCL>>
{
    type Sercom = P::Sercom;
}

impl<P: pads::ValidPads, R: IsRoles> InternalPads<P, R> {
    /// Assign an SDA pin `I` (role `Sda`)
    /// *Requires:* `P: ReplacePad<I>` (slot exists and yields ValidPads) and `R: ReplaceRole<Sda>`.
    /// Returns a new `InternalPads<P::NewPads, R::NewRoles<I>>`.
    pub fn sda<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Sda>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }

    /// Assign an SCL pin `I` (role `Scl`)
    pub fn scl<I: IsPad>(self, pin: I) -> InternalPads<P::NewPads, R::NewRoles<I>>
    where
        P: ReplacePad<I>,
        R: ReplaceRole<Scl>,
    {
        InternalPads {
            pads: self.pads.replace(pin),
            roles: self.roles.replace::<I>(),
        }
    }
}

impl InternalPads {
    /// Convenience constructor: build `Pads<SDA, SCL>` in one go.
    /// *Requires:* Underlying `pads::Pads` supports `.replace(SDA)` then `.replace(SCL)`.
    pub fn new<SDA: IsPad, SCL: IsPad>(sda: SDA, scl: SCL) -> Pads<SDA, SCL>
    where
        pads::Pads: ReplacePad<SDA, NewPads: ReplacePad<SCL>>,
    {
        InternalPads::default().sda(sda).scl(scl)
    }
}
