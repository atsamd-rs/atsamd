//! Provides low-level access to the [Real Time Clock (RTC)](https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-14/GUID-E17D8859-D42B-4B0E-9B81-76168A0C38AC.html) peripheral on ATSAMD chips.
//!
//! The main abstraction is the [`RtcMode`] trait, which exposes
//! static/associated functions to use the RTC in in a particular mode. All functions are marked as [`inline`](https://matklad.github.io/2021/07/09/inline-in-rust.html)
//! so that this should be a zero cost abstraction.
//!
//! This module is intended to serve as the basis for the safe
//! [`Rtc`](crate::rtc::Rtc) abstraction as well as RTIC and embassy time
//! drivers.
//!
//! Abstraction benefits:
//! - Handles all RTC register accesses.
//! - Handles RTC [register synchronization](https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-14/GUID-ABE2D37F-8125-4279-9955-BC3900046CFF.html).
//! - Handles ATSAMD chip variations.
//!
//! The idea is that various higher-level users of these abstractions will not
//! have to handle these low-level aspects of using the RTC. However, this
//! module does not present a safe interface. For example, many of the methods
//! in [`RtcMode`] assume that the RTC has already been put into the correct
//! mode (using [`RtcMode::set_mode`]), but without enforcing this in any way.

// As explained in the [datasheets](https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-14/GUID-ABE2D37F-8125-4279-9955-BC3900046CFF.html),
// reading a read-synced register may result in
// an old value, which we try to avoid by ensuring that SYNCBUSY is clear
// before reading. A write to a write-synced register will be discarded if
// syncing is happening during the write. As such, we also ensure that SYNCBUSY
// is clear before writing to a synced register. Throughout the crate, every
// register access should be prefaced by a `SYNC` comment indicating the
// required synchronization. The presence of this comment signals that this
// access was checked in the datasheet and accounted for.

use crate::pac;
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use pac::Rtc;

/// Type-level enum for RTC interrupts.
pub trait RtcInterrupt {
    /// Enable this interrupt.
    fn enable(rtc: &Rtc);
    /// Returns whether the interrupt has been triggered.
    fn check_flag(rtc: &Rtc) -> bool;
    /// Clears the interrupt flag so the ISR will not be called again
    /// immediately.
    fn clear_flag(rtc: &Rtc);
}

/// Macro to easily declare an RTC interrupt.
macro_rules! create_rtc_interrupt {
    ($mode:ident, $name:ident, $bit:ident) => {
        #[doc = concat!("Type-level variant for the ", stringify!($name), " interrupt in ", stringify!($mode))]
        pub enum $name {}
        impl RtcInterrupt for $name {
            #[inline]
            fn enable(rtc: &Rtc) {
                // SYNC: write
                rtc.$mode().intenset().write(|w| w.$bit().set_bit());
                sync_wait!(rtc, enable)
            }

            #[inline]
            fn check_flag(rtc: &Rtc) -> bool {
                // SYNC: None
                rtc.$mode().intflag().read().$bit().bit_is_set()
            }

            #[inline]
            fn clear_flag(rtc: &Rtc) {
                // SYNC: None
                rtc.$mode().intflag().write(|w| w.$bit().set_bit());
            }
        }
    };
}

macro_rules! sync_wait {
    ($rtc:expr, $register:ident) => {
        while $rtc.mode0().syncbusy().read().$register().bit_is_set() {}
    };
}

/// An abstraction of an RTC in a particular mode that provides low-level
/// access and handles all register syncing issues using only associated
/// functions.
pub trait RtcMode {
    /// The type of the COUNT register.
    type Count: Copy + PartialEq + Eq;

    /// Sets this mode in the CTRL register.
    ///
    /// # Safety
    ///
    /// This can be called any time but is typically only called once before
    /// calling most other methods.
    fn set_mode(rtc: &Rtc);

    /// Sets a compare value.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    fn set_compare(rtc: &Rtc, number: usize, value: Self::Count);

    /// Retrieves a compare from the register.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    fn get_compare(rtc: &Rtc, number: usize) -> Self::Count;

    /// Returns the current synced COUNT value.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    fn count(rtc: &Rtc) -> Self::Count;

    /// Returns whether register syncing is currently happening.
    ///
    /// # Safety
    ///
    /// Can be called any time.
    #[inline]
    #[hal_macro_helper]
    fn sync_busy(rtc: &Rtc) -> bool {
        // NOTE: This register and field are the same in all modes.
        // SYNC: None
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        return rtc.mode0().status().read().syncbusy().bit_is_set();
        // SYNC: None
        #[hal_cfg("rtc-d5x")]
        return rtc.mode0().syncbusy().read().bits() != 0;
    }

    /// Resets the RTC, leaving it disabled in MODE0.
    ///
    /// # Safety
    ///
    /// Can be called any time.
    #[inline]
    #[hal_macro_helper]
    fn reset(rtc: &Rtc) {
        // Reset RTC back to initial settings, which disables it and enters mode 0.
        // NOTE: This register and field are the same in all modes.
        // SYNC: Write
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.swrst().set_bit());
        #[hal_cfg("rtc-d5x")]
        rtc.mode0().ctrla().modify(|_, w| w.swrst().set_bit());

        // Wait for the reset to complete
        // SYNC: Write (we just read though)
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode0().ctrl().read().swrst().bit_is_set() {}
        #[hal_cfg("rtc-d5x")]
        {
            while rtc.mode0().ctrla().read().swrst().bit_is_set() {}
            sync_wait!(rtc, swrst)
        }
    }

    /// Starts the RTC and does any required initialization for this mode.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    #[inline]
    #[hal_macro_helper]
    fn start_and_initialize(rtc: &Rtc) {
        // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
        #[hal_cfg("rtc-d5x")]
        {
            // Enable counter synchronization
            // NOTE: This register and field are the same in all modes.
            // SYNC: Write
            rtc.mode0().ctrla().modify(|_, w| {
                w.prescaler().div1();
                w.countsync().set_bit();
                w
            });
            sync_wait!(rtc, countsync);

            Self::enable(rtc);

            // Errata: The first read of the count is incorrect so we need to read it
            // then wait for it to change.
            Self::_wait_for_count_change(rtc);
        }
    }

    /// Enables an RTC interrupt.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    #[inline]
    fn enable_interrupt<I: RtcInterrupt>(rtc: &Rtc) {
        I::enable(rtc);
    }

    /// Returns whether an RTC interrupt has been triggered.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    #[inline]
    fn check_interrupt_flag<I: RtcInterrupt>(rtc: &Rtc) -> bool {
        I::check_flag(rtc)
    }

    /// Clears an RTC interrupt flag so the ISR will not be called again
    /// immediately.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode).
    #[inline]
    fn clear_interrupt_flag<I: RtcInterrupt>(rtc: &Rtc) {
        I::clear_flag(rtc);
    }

    /// Waits for any register syncing to be completed, or returns immediately
    /// if not currently syncing.
    ///
    /// # Safety
    ///
    /// Can be called any time.
    #[inline]
    fn sync(rtc: &Rtc) {
        while Self::sync_busy(rtc) {}
    }

    /// Disables the RTC.
    ///
    /// # Safety
    ///
    /// Can be called any time.
    #[inline]
    #[hal_macro_helper]
    fn disable(rtc: &Rtc) {
        // NOTE: This register and field are the same in all modes.
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.enable().clear_bit());
        #[hal_cfg("rtc-d5x")]
        {
            rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());

            sync_wait!(rtc, enable)
        }
    }

    /// Enables the RTC.
    ///
    /// # Safety
    ///
    /// Can be called any time.
    #[inline]
    #[hal_macro_helper]
    fn enable(rtc: &Rtc) {
        // NOTE: This register and field are the same in all modes.
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.enable().set_bit());
        #[hal_cfg("rtc-d5x")]
        {
            rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());
            sync_wait!(rtc, enable)
        }
    }

    /// Waits until the COUNT register changes.
    ///
    /// Note that this may not necessarily be the next tick numerically due sync
    /// delay.
    ///
    /// # Safety
    ///
    /// Should be called only after setting the RTC mode using
    /// [`set_mode`](RtcMode::set_mode). This will halt forever if called when
    /// the RTC is disabled.
    #[inline]
    fn _wait_for_count_change(rtc: &Rtc) -> Self::Count {
        let mut last_count = Self::count(rtc);

        loop {
            let count = Self::count(rtc);

            if count != last_count {
                break count;
            }

            last_count = count;
        }
    }
}

/// Interface for using the RTC in MODE0 (32-bit COUNT)
pub mod mode0 {
    use super::*;

    create_rtc_interrupt!(mode0, Compare0, cmp0);
    #[hal_cfg("rtc-d5x")]
    create_rtc_interrupt!(mode0, Compare1, cmp1);
    #[hal_cfg("rtc-d5x")]
    create_rtc_interrupt!(mode0, Overflow, ovf);

    /// The RTC operating in MODE0 (32-bit COUNT)
    pub struct RtcMode0;

    impl RtcMode for RtcMode0 {
        type Count = u32;

        #[inline]
        #[hal_macro_helper]
        fn set_mode(rtc: &Rtc) {
            // NOTE: This register and field are the same in all modes.
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            rtc.mode0().ctrl().modify(|_, w| w.mode().count32());
            #[hal_cfg("rtc-d5x")]
            rtc.mode0().ctrla().modify(|_, w| w.mode().count32());
        }

        #[inline]
        fn set_compare(rtc: &Rtc, number: usize, value: Self::Count) {
            // SYNC: Write
            unsafe {
                rtc.mode0().comp(number).write(|w| w.comp().bits(value));
            }

            match number {
                0 => sync_wait!(rtc, comp0),
                1 => sync_wait!(rtc, comp1),
                _ => {}
            }
        }

        #[inline]
        fn get_compare(rtc: &Rtc, number: usize) -> Self::Count {
            // SYNC: Write (we just read though)
            rtc.mode0().comp(number).read().bits()
        }

        #[inline]
        #[hal_macro_helper]
        fn count(rtc: &Rtc) -> Self::Count {
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            {
                // Request syncing of the COUNT register.
                // SYNC: None
                rtc.mode0().readreq().modify(|_, w| w.rreq().set_bit());
            }

            // SYNC: Read/Write
            sync_wait!(rtc, count);
            rtc.mode0().count().read().bits()
        }
    }
}

/// Interface for using the RTC in MODE1 (16-bit COUNT)
pub mod mode1 {
    use super::*;

    create_rtc_interrupt!(mode1, Compare0, cmp0);
    create_rtc_interrupt!(mode1, Compare1, cmp1);
    create_rtc_interrupt!(mode1, Overflow, ovf);

    /// The RTC operating in MODE1 (16-bit COUNT)
    pub struct RtcMode1;

    impl RtcMode for RtcMode1 {
        type Count = u16;

        #[inline]
        #[hal_macro_helper]
        fn set_mode(rtc: &Rtc) {
            // SYNC: Write
            Self::sync(rtc);
            // NOTE: This register and field are the same in all modes.
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            rtc.mode0().ctrl().modify(|_, w| w.mode().count16());
            #[hal_cfg("rtc-d5x")]
            rtc.mode0().ctrla().modify(|_, w| w.mode().count16());

            // Set the mode 1 period
            // SYNC: Write
            Self::sync(rtc);
            unsafe { rtc.mode1().per().write(|w| w.bits(0xFFFF)) };
        }

        #[inline]
        fn set_compare(rtc: &Rtc, number: usize, value: Self::Count) {
            // SYNC: Write
            Self::sync(rtc);
            unsafe { rtc.mode1().comp(number).write(|w| w.comp().bits(value)) };
        }

        #[inline]
        fn get_compare(rtc: &Rtc, number: usize) -> Self::Count {
            // SYNC: Write (we just read though)
            rtc.mode1().comp(number).read().bits()
        }

        #[inline]
        #[hal_macro_helper]
        fn count(rtc: &Rtc) -> Self::Count {
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            {
                // Request syncing of the COUNT register.
                // SYNC: None
                rtc.mode1().readreq().modify(|_, w| w.rreq().set_bit());
            }

            // SYNC: Read/Write
            Self::sync(rtc);
            rtc.mode1().count().read().bits()
        }
    }
}
