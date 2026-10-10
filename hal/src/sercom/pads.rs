//! Generic SERCOM Pads
//!
//! This module defines a four‐slot, type‐level abstraction for SERCOM pads.
//! Each SERCOM peripheral has up to 4 associated pads (Pad0 to Pad3).
//! Putting the storage of actual Pads objects (GPIO pins with alternate functions)
//! into this struct always fulfills the invariant that each SERCOM pad can only be
//! assigned to one GPIO pin. The struct can be used as a base for all SERCOM peripherals
//! (UART, SPI, I2C). It uses a builder‐pattern style via `replace(...)` calls to assign
//! individual fields. The module also defines a `roles_type!` macro to link peripheral
//! roles (like Rx, Tx, Clk, Rts, Cts for UART) with the base pads type.

use crate::sercom::*;
use crate::typelevel::NoneT;

//==============================================================================
// Pads
//==============================================================================

/// `Pads<P0, P1, P2, P3>` is a generic container for up to four optional SERCOM pads.
/// By default, all four slots are `NoneT`, allowing you to call `Pads::default()`
/// and then fill in slots one at a time via `replace(...)`.
pub struct Pads<
    P0: OptionalPad = NoneT,
    P1: OptionalPad = NoneT,
    P2: OptionalPad = NoneT,
    P3: OptionalPad = NoneT,
>(P0, P1, P2, P3);

/// `Default` is implemented only for the “all-`NoneT`” case.
impl Default for Pads<NoneT, NoneT, NoneT, NoneT> {
    fn default() -> Self {
        Pads(NoneT, NoneT, NoneT, NoneT)
    }
}

/// Marker trait: `ReplacePad<I>` is shorthand for `ReplacePadNum<I, I::PadNum>`.
/// If a type implements `ReplacePadNum<I, I::PadNum>`, it automatically implements `ReplacePad<I>`.
/// Skip if trying to replace no pad (NoneT)
pub trait ReplacePad<I: OptionalPad> {
    type NewPads: ValidPads;
    fn replace(self, pin: I) -> Self::NewPads;
}
impl<I: IsPad, R: ReplacePadNum<I, I::PadNum>> ReplacePad<I> for R {
    type NewPads = R::NewPads;
    fn replace(self, pin: I) -> Self::NewPads {
        <R as ReplacePadNum<I, I::PadNum>>::replace(self, pin)
    }
}
impl<V: ValidPads> ReplacePad<NoneT> for V {
    type NewPads = V;
    fn replace(self, _: NoneT) -> Self::NewPads {
        self
    }
}

/// Core trait for “filling” exactly one slot in a `Pads<...>` tuple.
/// - `I: IsPad` is the chosen pad type (e.g., a concrete `Pad<SercomX, PAxx>`).
/// - `P: PadNum` is the numeric index of the pad (Pad0, Pad1, Pad2, or Pad3).
///
/// This trait requires that the original `Pads<...>` be `ValidPads`, and that
/// the resulting `NewPads` also be `ValidPads`. This ensures compile‐time checks
/// that all chosen pads share the same SERCOM (and, on M4 targets, a shared IO set).
///
/// # Associated Types
/// - `type NewPads: ValidPads`: the resulting `Pads<...>` type after replacing one slot.
///
/// # Method
/// - `fn replace(self, pin: I) -> Self::NewPads`: consumes the old `Pads<...>`
///   and returns a new `Pads<...>` with the specified slot filled by `pin`.
pub trait ReplacePadNum<I: IsPad, P: PadNum>: ValidPads {
    type NewPads: ValidPads;
    fn replace(self, pin: I) -> Self::NewPads;
}

/// Fills slot 0 if it is `NoneT`.
impl<P1: OptionalPad, P2: OptionalPad, P3: OptionalPad, I: IsPad<PadNum = Pad0>>
    ReplacePadNum<I, Pad0> for Pads<NoneT, P1, P2, P3>
where
    Pads<NoneT, P1, P2, P3>: ValidPads,
    Pads<I, P1, P2, P3>: ValidPads,
{
    type NewPads = Pads<I, P1, P2, P3>;
    fn replace(self, pin: I) -> Self::NewPads {
        Pads(pin, self.1, self.2, self.3)
    }
}

/// Fills slot 1 if it is `NoneT`.
impl<P0: OptionalPad, P2: OptionalPad, P3: OptionalPad, I: IsPad<PadNum = Pad1>>
    ReplacePadNum<I, Pad1> for Pads<P0, NoneT, P2, P3>
where
    Pads<P0, NoneT, P2, P3>: ValidPads,
    Pads<P0, I, P2, P3>: ValidPads,
{
    type NewPads = Pads<P0, I, P2, P3>;
    fn replace(self, pin: I) -> Self::NewPads {
        Pads(self.0, pin, self.2, self.3)
    }
}

/// Fills slot 2 if it is `NoneT`.
impl<P0: OptionalPad, P1: OptionalPad, P3: OptionalPad, I: IsPad<PadNum = Pad2>>
    ReplacePadNum<I, Pad2> for Pads<P0, P1, NoneT, P3>
where
    Pads<P0, P1, NoneT, P3>: ValidPads,
    Pads<P0, P1, I, P3>: ValidPads,
{
    type NewPads = Pads<P0, P1, I, P3>;
    fn replace(self, pin: I) -> Self::NewPads {
        Pads(self.0, self.1, pin, self.3)
    }
}

/// Fills slot 3 if it is `NoneT`.
impl<P0: OptionalPad, P1: OptionalPad, P2: OptionalPad, I: IsPad<PadNum = Pad3>>
    ReplacePadNum<I, Pad3> for Pads<P0, P1, P2, NoneT>
where
    Pads<P0, P1, P2, NoneT>: ValidPads,
    Pads<P0, P1, P2, I>: ValidPads,
{
    type NewPads = Pads<P0, P1, P2, I>;
    fn replace(self, pin: I) -> Self::NewPads {
        Pads(self.0, self.1, self.2, pin)
    }
}

/// The `ValidPads` trait marks a `Pads<P0, P1, P2, P3>` type as “all four pads
/// (ignoring `NoneT`) belong to exactly one SERCOM peripheral.” Downstream code
/// uses this to constrain valid pad combinations.
pub trait ValidPads {
    /// The associated `Sercom` type that all four pads map to.
    type Sercom: Sercom;
}

/// On thumbv6m (M0) targets (`sercom0-d11` or `sercom0-d21`), we only need to
/// check that all pads share the same SERCOM (`ShareSercom`).
#[hal_cfg(any("sercom0-d11", "sercom0-d21"))]
impl<P0: OptionalPad, P1: OptionalPad, P2: OptionalPad, P3: OptionalPad> ValidPads
    for Pads<P0, P1, P2, P3>
where
    // `ShareSercom` ensures any non‐`NoneT` pad among P0..P3 maps to the same SERCOM.
    (P0, P1, P2, P3): ShareSercom,
{
    /// The `Sercom` type is obtained from the `ShareSercom` implementation.
    type Sercom = <(P0, P1, P2, P3) as ShareSercom>::Sercom;
}

/// On thumbv7em (M4) targets (`sercom0-d5x`), each SERCOM can be mapped to multiple IO sets.
/// We must ensure both:
/// 1. All pads share the same SERCOM (`ShareSercom`).
/// 2. All pads lie in one common IO set (`ShareIoSet`).
#[hal_cfg("sercom0-d5x")]
impl<P0: OptionalPad, P1: OptionalPad, P2: OptionalPad, P3: OptionalPad> ValidPads
    for Pads<P0, P1, P2, P3>
where
    (P0, P1, P2, P3): ShareSercom,
    (P0, P1, P2, P3): ShareIoSet,
{
    /// The `Sercom` type is still obtained from `ShareSercom`.
    type Sercom = <(P0, P1, P2, P3) as ShareSercom>::Sercom;
}

//==============================================================================
// roles_type! macro
//==============================================================================

/// `roles_type!` generates a `Roles<…>` struct and implements a `ReplaceRole<R>` trait
/// for each role. Used by peripheral-specific pad modules (e.g., UART, SPI, I2C) to
/// fill role slots in order.
///
/// Example:
/// ```ignore
/// roles_type!(Rx, Tx, Rts, Cts);
/// ```
/// Expands to:
/// - `pub struct Roles<Rx: OptionalPad = NoneT, Tx: OptionalPad = NoneT, Rts: OptionalPad = NoneT, Cts: OptionalPad = NoneT>(PhantomData<Rx>, PhantomData<Tx>, PhantomData<Rts>, PhantomData<Cts>);`
/// - `impl Default for Roles<…> { /* sets all to NoneT */ }`
/// - `pub trait IsRoles {}`
/// - `impl<…> IsRoles for Roles<…> {} // any combination of OptionalPad`
/// - `pub trait IsRole {}`
/// - `pub trait ReplaceRole<R: IsRole>: IsRoles { type NewRoles<I: OptionalPad>: IsRoles; fn replace<I: OptionalPad>(self) -> Self::NewRoles<I>; }`
/// - For each `role` (e.g. `Rx`), an empty `pub struct Rx; impl IsRole for Rx {}` and an impl block:
///   `impl<other roles> ReplaceRole<Rx> for Roles<NoneT, _, _, _> { type NewRoles<I> = Roles<I, _, _, _>; fn replace(self, pin) { /* set Rx slot to I, keep others same */ } }`
#[macro_export]
macro_rules! roles_type {
    ($($role:ident),+ $(,)?) => {
        // Define the `Roles<…>` struct, with each `role: OptionalPad = NoneT` default.
        pub struct Roles<$($role: OptionalPad = NoneT,)*> ($(PhantomData<$role>,)*);

        // Implement `Default` for `Roles<…>` by setting every slot to `NoneT`.
        impl<$($role: OptionalPad,)*> Default for Roles<$($role,)*> {
            fn default() -> Self {
                Roles($(PhantomData::<$role>,)*)
            }
        }

        // Blanket trait to mark a type as having “role slots.”
        pub trait IsRoles {}

        // Implement IsRoles for any Roles type.
        impl<$($role: OptionalPad,)*> IsRoles for Roles<$($role,)*> {}

        // Marker trait for each specific role name.
        pub trait IsRole {}

        // Allows replacing one role slot at a time.
        pub trait ReplaceRole<R: IsRole>: IsRoles {
            type NewRoles<I: OptionalPad>: IsRoles;
            fn replace<I: OptionalPad>(self) -> Self::NewRoles<I>;
        }

        // Recurse into the helper arm to generate per-role impls.
        roles_type!(@internal (), $($role),+);
    };
    (@internal ($($done:ident,)*), $head:ident$(, $tail:ident)*) => {
        paste::paste! {
            // Define the marker type for this role (e.g., `pub struct Rx;`).
            pub struct $head;
            impl IsRole for $head {}

            // Implement `ReplaceRole<$head>` for `Roles<…>` where that role slot is `NoneT`.
            impl<$($done: OptionalPad,)* $($tail: OptionalPad),*> ReplaceRole<$head>
                for Roles<$($done,)* NoneT, $($tail,)*>
            {
                // The new roles type replaces `NoneT` at position `$head` with a concrete `I`.
                type NewRoles<I: OptionalPad> = Roles<$($done,)* I, $($tail,)*>;
                fn replace<I: OptionalPad>(self) -> Self::NewRoles<I> {
                    #[allow(non_snake_case)]
                    // Destructure `self` as `Roles(done_var, _skip, tail_var, …)`
                    let Roles($([<$done _var>],)* _skip, $([<$tail _var>],)*) = self;
                    Roles($([<$done _var>],)* PhantomData::<I>, $([<$tail _var>],)*)
                }
            }

            // Recurse to generate the next role’s implementation.
            roles_type!(@internal ($($done,)* $head,), $($tail),*);
        }
    };
    (@internal ($($done:ident,)*),) => {};
}

//==============================================================================
// impl_const! macro
//==============================================================================

/// `impl_const!` is a helper to generate multiple `impl Trait for Type` blocks
/// that only define a single associated constant. For example:
///
/// ```ignore
/// impl_const! {
///     trait = Rxpo;
///     field = const RXPO: u8;
///     Pad0 => 0,
///     Pad1 => 1,
///     Pad2 => 2,
///     Pad3 => 3,
/// }
/// ```
/// Expands to:
/// ```ignore
/// impl Rxpo for Pad0 { const RXPO: u8 = 0; }
/// impl Rxpo for Pad1 { const RXPO: u8 = 1; }
/// impl Rxpo for Pad2 { const RXPO: u8 = 2; }
/// impl Rxpo for Pad3 { const RXPO: u8 = 3; }
/// ```
#[macro_export]
macro_rules! impl_const {
    (
        trait = $trait:path;
        field = const $field:ident : $ty:ty;
        $(
            $impl_ty:ty => $value:expr
        ),* $(,)?
    ) => {
        $(
            impl $trait for $impl_ty {
                const $field: $ty = $value;
            }
        )*
    };
}
