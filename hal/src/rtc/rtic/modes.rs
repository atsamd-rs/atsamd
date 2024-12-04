//! Abstraction of basic RTC functions for each mode.
//!
//! As explained in the [datasheets](https://onlinedocs.microchip.com/oxy/GUID-F5813793-E016-46F5-A9E2-718D8BCED496-en-US-14/GUID-ABE2D37F-8125-4279-9955-BC3900046CFF.html),
//! reading a read-synced register may result in
//! an old value, which we try to avoid by ensuring that SYNCBUSY is clear
//! before reading. A write to a write-synced register will be discarded if
//! syncing is happening during the write. As such, we also ensure that SYNCBUSY
//! is clear before writing to a synced register. Every register access should
//! be prefaced by a `SYNC`` comment indicating the required synchronization,
//! the presence of this comment indicates that this access was checked and
//! accounted for.

use crate::pac;
use atsamd_hal_macros::hal_macro_helper;
use pac::Rtc;

// TODO: Test code
#[derive(Default)]
pub struct LoopChecker {
    count: usize,
}
impl LoopChecker {
    pub fn too_many(&mut self) -> bool {
        self.count += 1;

        self.count > 0x800000
    }
}

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
        /// Type-level variant for the $name interrupt in $mode.
        enum $name {}
        impl RtcInterrupt for $name {
            #[inline]
            fn enable(rtc: &Rtc) {
                // SYNC: None
                rtc.$mode().intenset().write(|w| w.$bit().set_bit());
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

/// An abstraction of an RTC in a particular mode that provides low-level
/// access and handles all register syncing issues using only associated
/// functions.
#[hal_macro_helper]
pub trait RtcMode {
    /// The type of the COUNT register.
    type Count: Copy + PartialEq + Eq;
    /// The COUNT value representing a half period.
    const HALF_PERIOD: Self::Count;
    /// The minimum number of ticks that compares need to be ahead of the COUNT
    /// in order to trigger.
    const MIN_COMPARE_TICKS: Self::Count;

    // TODO: Test code
    fn check_loop(rtc: &Rtc, checker: &mut LoopChecker, item: &str);
    fn check_for_clock_anomaly(rtc: &Rtc, last_count: Self::Count, count: Self::Count);

    /// Sets this mode in the CTRL register.
    unsafe fn set_mode(rtc: &Rtc);
    /// Sets a compare value.
    unsafe fn set_compare(rtc: &Rtc, number: usize, value: Self::Count);
    /// Retrieves a compare from the register.
    fn get_compare(rtc: &Rtc, number: usize) -> Self::Count;
    /// Starts the RTC and does any required initialization for this mode.
    fn start_and_initialize(rtc: &Rtc);
    /// Returns the current synced COUNT value.
    fn count(rtc: &Rtc) -> Self::Count;
    /// Returns whether register syncing is currently happening.
    fn sync_busy(rtc: &Rtc) -> bool;

    /// Resets the RTC, leaving it disabled in MODE0.
    #[inline]
    fn reset(rtc: &Rtc) {
        // Reset RTC back to initial settings, which disables it and enters mode 0.
        // NOTE: This register and field are the same in all modes.
        // SYNC: Write
        Self::sync(rtc);
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.swrst().set_bit());
        #[hal_cfg("rtc-d5x")]
        rtc.mode0().ctrla().modify(|_, w| w.swrst().set_bit());

        // Wait for the reset to complete
        let mut loop_checker = LoopChecker::default();
        // SYNC: Write (we just read though)
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while rtc.mode0().ctrl().read().swrst().bit_is_set() {
            Self::check_loop(rtc, &mut loop_checker, "reset");
        }
        #[hal_cfg("rtc-d5x")]
        while rtc.mode0().ctrla().read().swrst().bit_is_set() {
            Self::check_loop(rtc, &mut loop_checker, "reset");
        }
    }

    /// Enables an RTC interrupt.
    #[inline]
    fn enable_interrupt<I: RtcInterrupt>(rtc: &Rtc) {
        I::enable(rtc);
    }

    /// Returns whether an RTC interrupt has been triggered.
    #[inline]
    fn check_interrupt_flag<I: RtcInterrupt>(rtc: &Rtc) -> bool {
        I::check_flag(rtc)
    }

    /// Clears an RTC interrupt flag so the ISR will not be called again
    /// immediately.
    #[inline]
    fn clear_interrupt_flag<I: RtcInterrupt>(rtc: &Rtc) {
        I::clear_flag(rtc);
    }

    /// Waits for any register syncing to be completed, or returns immediately
    /// if no currently syncing.
    #[inline]
    fn sync(rtc: &Rtc) {
        let mut loop_checker = LoopChecker::default();

        while Self::sync_busy(rtc) {
            Self::check_loop(rtc, &mut loop_checker, "sync");
        }
    }

    /// Disables the RTC.
    #[inline]
    fn disable(rtc: &Rtc) {
        // SYNC: Write
        Self::sync(rtc);
        // NOTE: This register and field are the same in all modes.
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.enable().clear_bit());
        #[hal_cfg("rtc-d5x")]
        rtc.mode0().ctrla().modify(|_, w| w.enable().clear_bit());
    }

    /// Enables the RTC.
    #[inline]
    fn enable(rtc: &Rtc) {
        // SYNC: Write
        Self::sync(rtc);
        // NOTE: This register and field are the same in all modes.
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        rtc.mode0().ctrl().modify(|_, w| w.enable().set_bit());
        #[hal_cfg("rtc-d5x")]
        rtc.mode0().ctrla().modify(|_, w| w.enable().set_bit());
    }

    /// Returns whether the RTC is enabled.
    #[inline]
    fn is_enabled(rtc: &Rtc) -> bool {
        // SYNC: Write (we just read though)
        // NOTE: This register and field are the same in all modes.
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        return rtc.mode0().ctrl().read().enable().bit_is_set();
        #[hal_cfg("rtc-d5x")]
        return rtc.mode0().ctrla().read().enable().bit_is_set();
    }

    /// Waits until the COUNT register changes.
    ///
    /// Note that this may not necessarily be the next tick due sync delay.
    /// Beware that this will wait forever if the RTC is disabled!
    #[inline]
    fn wait_for_count_change(rtc: &Rtc) -> Self::Count {
        let mut last_count = Self::count(rtc);

        let mut loop_checker = LoopChecker::default();
        loop {
            let count = Self::count(rtc);

            if count != last_count {
                Self::check_for_clock_anomaly(rtc, last_count, count);

                break count;
            }

            Self::check_loop(rtc, &mut loop_checker, "wait_for_count_change");

            last_count = count;
        }
    }
}

#[hal_macro_helper]
pub mod mode0 {
    use super::*;

    create_rtc_interrupt!(mode0, Compare0, cmp0);
    #[hal_cfg("rtc-d5x")]
    create_rtc_interrupt!(mode0, Compare1, cmp1);
    #[hal_cfg("rtc-d5x")]
    create_rtc_interrupt!(mode0, Overflow, ovf);

    /// The RTC operating in MODE0 (23-bit COUNT)
    pub struct RtcMode0;

    impl RtcMode for RtcMode0 {
        type Count = u32;
        const HALF_PERIOD: Self::Count = 0x8000_0000;
        const MIN_COMPARE_TICKS: Self::Count = 5;

        // TODO: Test code
        fn check_loop(rtc: &Rtc, checker: &mut LoopChecker, item: &str) {
            if checker.too_many() {
                panic!(
                "Took too long in '{item}' with count 0x{:X} and cmp 0x{:X}. Int flags: 0x{:X}, Enabled: {:?}",
                Self::count(rtc),
                Self::get_compare(rtc, 0),
                rtc.mode0().intflag().read().bits(),
                Self::is_enabled(rtc),
            );
            }
        }

        // TODO: Test code
        fn check_for_clock_anomaly(rtc: &Rtc, last_count: Self::Count, count: Self::Count) {
            if last_count > 0 && count != last_count.wrapping_add(4) {
                panic!("Clock anomaly at 0x{count:X} with previous step at 0x{last_count:X} with cmp 0x{:X}", Self::get_compare(rtc, 0));
            }
        }

        #[inline]
        unsafe fn set_mode(rtc: &Rtc) {
            // SYNC: Write
            Self::sync(rtc);
            // NOTE: This register and field are the same in all modes.
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            rtc.mode0().ctrl().modify(|_, w| w.mode().count32());
            #[hal_cfg("rtc-d5x")]
            rtc.mode0().ctrla().modify(|_, w| w.mode().count32());
        }

        #[inline]
        unsafe fn set_compare(rtc: &Rtc, number: usize, value: Self::Count) {
            // SYNC: Write
            Self::sync(rtc);
            rtc.mode0().comp(number).write(|w| w.comp().bits(value));
        }

        #[inline]
        fn get_compare(rtc: &Rtc, number: usize) -> Self::Count {
            // SYNC: Write (we just read though)
            rtc.mode0().comp(number).read().bits()
        }

        #[inline]
        fn start_and_initialize(rtc: &Rtc) {
            Self::enable(rtc);

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                // Enable counter synchronization
                // SYNC: Write
                Self::sync(rtc);
                #[hal_cfg("rtc-d5x")]
                rtc.mode0().ctrla().modify(|_, w| w.countsync().set_bit());

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                Self::wait_for_count_change(rtc);
            }
        }

        #[inline]
        fn count(rtc: &Rtc) -> Self::Count {
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            {
                // Request syncing of the COUNT register.
                // SYNC: None
                rtc.mode0().readreq().modify(|_, w| w.rreq().set_bit());
            }

            // SYNC: Read/Write
            Self::sync(rtc);
            rtc.mode0().count().read().bits()
        }

        #[inline]
        fn sync_busy(rtc: &Rtc) -> bool {
            // SYNC: None
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            return rtc.mode0().status().read().syncbusy().bit_is_set();

            // SYNC: None
            #[hal_cfg("rtc-d5x")]
            return rtc.mode0().syncbusy().read().bits() != 0;
        }
    }

    // The SAMD11/21 chips do not feature a second compare in MODE0.
    #[hal_cfg(any("rtc-d11", "rtc-d21"))]
    crate::__internal_basic_backend!(RtcBackend, RtcMode0, Compare0);

    #[hal_cfg("rtc-d5x")]
    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode0, Compare0, Compare1, Overflow
    );
}

#[hal_macro_helper]
pub mod mode1 {
    use super::*;

    create_rtc_interrupt!(mode1, Compare0, cmp0);
    create_rtc_interrupt!(mode1, Compare1, cmp1);
    create_rtc_interrupt!(mode1, Overflow, ovf);

    /// The RTC operating in MODE1 (16-bit COUNT)
    pub struct RtcMode1;

    impl RtcMode for RtcMode1 {
        type Count = u16;
        const HALF_PERIOD: Self::Count = 0x8000;
        const MIN_COMPARE_TICKS: Self::Count = 5;

        // TODO: Test code
        fn check_loop(rtc: &Rtc, checker: &mut LoopChecker, item: &str) {
            if checker.too_many() {
                panic!(
                "Took too long in '{item}' with count 0x{:X} and cmp 0x{:X}. Int flags: 0x{:X}, Enabled: {:?}",
                Self::count(rtc),
                Self::get_compare(rtc, 0),
                rtc.mode1().intflag().read().bits(),
                Self::is_enabled(rtc),
            );
            }
        }

        // TODO: Test code
        fn check_for_clock_anomaly(rtc: &Rtc, last_count: Self::Count, count: Self::Count) {
            if last_count > 0 && count != last_count.wrapping_add(4) {
                panic!("Clock anomaly at 0x{count:X} with previous step at 0x{last_count:X} with cmp 0x{:X}", Self::get_compare(rtc, 0));
            }
        }

        #[inline]
        unsafe fn set_mode(rtc: &Rtc) {
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
            rtc.mode1().per().write(|w| w.bits(0xFFFF));
        }

        #[inline]
        unsafe fn set_compare(rtc: &Rtc, number: usize, value: Self::Count) {
            // SYNC: Write
            Self::sync(rtc);
            rtc.mode1().comp(number).write(|w| w.comp().bits(value));
        }

        #[inline]
        fn get_compare(rtc: &Rtc, number: usize) -> Self::Count {
            // SYNC: Write (we just read though)
            rtc.mode1().comp(number).read().bits()
        }

        #[inline]
        fn start_and_initialize(rtc: &Rtc) {
            Self::enable(rtc);

            // Enable counter sync on SAMx5x, the counter cannot be read otherwise.
            #[hal_cfg("rtc-d5x")]
            {
                // Enable counter synchronization
                // SYNC: Write
                Self::sync(rtc);
                rtc.mode1().ctrla().modify(|_, w| w.countsync().set_bit());

                // Errata: The first read of the count is incorrect so we need to read it
                // then wait for it to change.
                Self::wait_for_count_change(rtc);
            }
        }

        #[inline]
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

        #[inline]
        fn sync_busy(rtc: &Rtc) -> bool {
            // SYNC: None
            #[hal_cfg(any("rtc-d11", "rtc-d21"))]
            return rtc.mode1().status().read().syncbusy().bit_is_set();

            // SYNC: None
            #[hal_cfg("rtc-d5x")]
            return rtc.mode1().syncbusy().read().bits() != 0;
        }
    }

    crate::__internal_half_period_counting_backend!(
        RtcBackend, RtcMode1, Compare0, Compare1, Overflow
    );
}
