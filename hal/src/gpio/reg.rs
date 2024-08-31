use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

#[hal_cfg(any("port-d11", "port-d21"))]
use crate::pac::port::{
    Ctrl, Dir, Dirclr, Dirset, Dirtgl, In, Out, Outclr, Outset, Outtgl, Pincfg0_ as Pincfg,
    Pmux0_ as Pmux, Wrconfig,
};

#[hal_cfg("port-d5x")]
use crate::pac::port::group::{
    Ctrl, Dir, Dirclr, Dirset, Dirtgl, In, Out, Outclr, Outset, Outtgl, Pincfg, Pmux, Wrconfig,
};

use crate::pac::Port;

use super::dynpin::*;

//==============================================================================
//  ModeFields
//==============================================================================

/// Collect all fields needed to set the [`PinMode`](super::PinMode)
#[derive(Default)]
struct ModeFields {
    dir: bool,
    inen: bool,
    pullen: bool,
    out: bool,
    pmuxen: bool,
    pmux: u8,
}

impl From<DynPinMode> for ModeFields {
    #[inline]
    #[hal_macro_helper]
    fn from(mode: DynPinMode) -> Self {
        let mut fields = Self::default();
        use DynPinMode::*;
        match mode {
            Disabled(config) => {
                use DynDisabled::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Input(config) => {
                fields.inen = true;
                use DynInput::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Interrupt(config) => {
                fields.pmuxen = true;
                fields.pmux = 0;
                use DynInterrupt::*;
                match config {
                    Floating => {
                        fields.pullen = false;
                        fields.out = false;
                    }
                    PullDown => {
                        fields.pullen = true;
                        fields.out = false;
                    }
                    PullUp => {
                        fields.pullen = true;
                        fields.out = true;
                    }
                }
            }
            Output(config) => {
                fields.dir = true;
                use DynOutput::*;
                match config {
                    PushPull => {
                        fields.inen = false;
                    }
                    Readable => {
                        fields.inen = true;
                    }
                }
            }
            Alternate(config) => {
                fields.pmuxen = true;
                use DynAlternate::*;
                match config {
                    B => {
                        fields.pmux = 1;
                    }
                    C => {
                        fields.pmux = 2;
                    }
                    D => {
                        fields.pmux = 3;
                    }
                    E => {
                        fields.pmux = 4;
                    }
                    F => {
                        fields.pmux = 5;
                    }
                    G => {
                        fields.pmux = 6;
                    }
                    #[hal_cfg(any("port-d21", "port-d5x"))]
                    H => {
                        fields.pmux = 7;
                    }
                    #[hal_cfg("port-d5x")]
                    I => {
                        fields.pmux = 8;
                    }
                    #[hal_cfg("port-d5x")]
                    J => {
                        fields.pmux = 9;
                    }
                    #[hal_cfg("port-d5x")]
                    K => {
                        fields.pmux = 10;
                    }
                    #[hal_cfg("port-d5x")]
                    L => {
                        fields.pmux = 11;
                    }
                    #[hal_cfg("port-d5x")]
                    M => {
                        fields.pmux = 12;
                    }
                    #[hal_cfg("port-d5x")]
                    N => {
                        fields.pmux = 13;
                    }
                }
            }
        };
        fields
    }
}

//==============================================================================
//  GROUP
//==============================================================================

/// Represent the [`Port`] register block
///
/// The SAMx5x PACs have a GROUP type to represent each [`Port`] group, but the
/// SAMD11 and SAMD21 PACs do not. Manually re-implement it here.
#[repr(C)]
#[allow(clippy::upper_case_acronyms)]
pub(super) struct GROUP {
    dir: Dir,
    dirclr: Dirclr,
    dirset: Dirset,
    dirtgl: Dirtgl,
    out: Out,
    outclr: Outclr,
    outset: Outset,
    outtgl: Outtgl,
    in_: In,
    ctrl: Ctrl,
    wrconfig: Wrconfig,
    _padding1: [u8; 4],
    pmux: [Pmux; 16],
    pincfg: [Pincfg; 32],
    _padding2: [u8; 32],
}

//==============================================================================
//  RegisterInterface
//==============================================================================

/// Provide a safe register interface for pin objects
///
/// [`Port`], like every PAC `struct`, is [`Send`] but not [`Sync`], because it
/// points to a `RegisterBlock` of `VolatileCell`s. Unfortunately, such an
/// interface is quite restrictive. Instead, it would be ideal if we could split
/// the [`Port`] into independent pins that are both [`Send`] and [`Sync`].
///
/// [`Port`] is a single, zero-sized marker `struct` that provides access to
/// every [`Port`] register. Instead, we would like to create zero-sized marker
/// `struct`s for every pin, where each pin is only allowed to control its own
/// registers. Furthermore, each pin `struct` should be a singleton, so that
/// exclusive access to the `struct` also guarantees exclusive access to the
/// corresponding registers. Finally, the pin `struct`s should not have any
/// interior mutability. Together, these requirements would allow the pin
/// `struct`s to be both [`Send`] and [`Sync`].
///
/// This trait creates a safe API for accomplishing these goals. Implementers
/// supply a pin ID through the [`id`] function. The remaining functions provide
/// a safe API for accessing the registers associated with that pin ID. Any
/// modification of the registers requires `&mut self`, which destroys interior
/// mutability.
///
/// # Safety
///
/// Users should only implement the [`id`] function. No default function
/// implementations should be overridden. The implementing type must also have
/// "control" over the corresponding pin ID, i.e. it must guarantee that a each
/// pin ID is a singleton.
///
/// [`id`]: Self::id
pub(super) unsafe trait RegisterInterface {
    /// Provide a [`DynPinId`] identifying the set of registers controlled by
    /// this type.
    fn id(&self) -> DynPinId;

    /// Pointer to the array of [`GROUP`] register blocks
    const GROUPS: *const GROUP = Port::ptr() as *const _;

    #[inline]
    #[hal_macro_helper]
    fn group(&self) -> &GROUP {
        let offset = match self.id().group {
            DynGroup::A => 0,
            #[hal_cfg("pin-group-b")]
            DynGroup::B => 1,
            #[hal_cfg("pin-group-c")]
            DynGroup::C => 2,
            #[hal_cfg("pin-group-d")]
            DynGroup::D => 3,
        };
        // Safety: It is safe to create shared references to each PAC register
        // or register block, because all registers are wrapped in
        // `UnsafeCell`s. We should never create unique references to the
        // registers, to prevent any risk of UB.
        unsafe { &*Self::GROUPS.add(offset) }
    }

    #[inline]
    fn mask_32(&self) -> u32 {
        1 << self.id().num
    }

    #[inline]
    fn mask_16(&self) -> u16 {
        1 << (self.id().num & 0xF)
    }

    #[inline]
    fn hwsel(&self) -> bool {
        self.id().num & 0x10 != 0
    }

    #[inline]
    fn pincfg(&self) -> &Pincfg {
        &self.group().pincfg[self.id().num as usize]
    }

    /// Change the pin mode
    ///
    /// We use the Wrconfig register to avoid using the Pmux register. Each Pmux
    /// register stores the Pmux values for two different pins.  Changing the
    /// Pmux value for one pin would require a read/modify/write operation that
    /// could be preempted by the other pin. This is fundamentally unsound. The
    /// Wrconfig register lets us modify *only* the fields corresponding to this
    /// particular PinId/DynPinId.
    #[inline]
    fn change_mode(&mut self, mode: DynPinMode) {
        let ModeFields {
            dir,
            inen,
            pullen,
            out,
            pmuxen,
            pmux,
        } = mode.into();
        // The bit patterns here are guaranteed to be safe, because they can
        // ultimately be traced back to associated constants defined on the
        // `PinId` and `PinMode` traits, which are guaranteed to be correct.
        self.group().wrconfig.write(|w| unsafe {
            w.hwsel().bit(self.hwsel());
            w.wrpincfg().set_bit();
            w.wrpmux().set_bit();
            w.pmux().bits(pmux);
            w.pullen().bit(pullen);
            w.inen().bit(inen);
            w.pmuxen().bit(pmuxen);
            w.pinmask().bits(self.mask_16())
        });
        self.set_dir(dir);
        if pullen {
            self.write_pin(out)
        };
    }

    /// Set the direction of a pin
    #[inline]
    fn set_dir(&mut self, bit: bool) {
        let mask = self.mask_32();
        // Safety: Dirset & Dirclr are "mask" registers, and we only write the
        // bit for this pin ID
        unsafe {
            if bit {
                self.group().dirset.write(|w| w.bits(mask));
            } else {
                self.group().dirclr.write(|w| w.bits(mask));
            }
        }
    }

    /// Read the logic level of an input put
    #[inline]
    #[allow(dead_code)]
    fn read_pin(&self) -> bool {
        let mask = self.mask_32();
        self.group().in_.read().bits() & mask != 0
    }

    /// Write the logic level of an output pin
    #[inline]
    fn write_pin(&mut self, bit: bool) {
        let mask = self.mask_32();
        // Safety: Outset & Outclr are "mask" registers, and we only write the
        // bit for this pin ID
        unsafe {
            if bit {
                self.group().outset.write(|w| w.bits(mask));
            } else {
                self.group().outclr.write(|w| w.bits(mask));
            }
        }
    }

    /// Toggle the logic level of an output pin
    #[inline]
    fn toggle_pin(&mut self) {
        let mask = self.mask_32();
        // Safety: Outtgl is a "mask" register, and we only write the bit for
        // this pin ID
        unsafe { self.group().outtgl.write(|w| w.bits(mask)) };
    }

    /// Read back the logic level of an output pin
    #[inline]
    #[allow(dead_code)]
    fn read_out_pin(&self) -> bool {
        let mask = self.mask_32();
        self.group().out.read().bits() & mask != 0
    }

    /// Read the drive strength of a pin
    #[inline]
    fn read_drive_strength(&self) -> bool {
        self.pincfg().read().drvstr().bit()
    }

    /// Write the drive strength of a pin
    #[inline]
    fn write_drive_strength(&mut self, bit: bool) {
        self.pincfg().modify(|_, w| w.drvstr().bit(bit));
    }
}
