//! Real-time clock/counter
use atsamd_hal_macros::{hal_cfg, hal_macro_helper};
use fugit::NanosDurationU32;

use crate::ehal_02;
use crate::pac;
use crate::pac::rtc::{Mode0, Mode2};
use crate::time::{Hertz, Nanoseconds};
use crate::timer_traits::InterruptDrivenTimer;
use crate::typelevel::Sealed;
use core::convert::Infallible;
use core::marker::PhantomData;

#[cfg(feature = "sdmmc")]
use embedded_sdmmc::{TimeSource, Timestamp};

#[cfg(feature = "rtic")]
pub mod rtic;

// SAMx5x imports
#[hal_cfg("rtc-d5x")]
use crate::pac::{
    rtc::mode0::ctrla::Prescalerselect, rtc::mode0::Ctrla as Mode0CtrlA,
    rtc::mode2::Ctrla as Mode2CtrlA, Mclk as Pm,
};

// SAMD11/SAMD21 imports
#[hal_cfg(any("rtc-d11", "rtc-d21"))]
use crate::pac::{
    rtc::mode0::ctrl::Prescalerselect, rtc::mode0::Ctrl as Mode0CtrlA,
    rtc::mode2::Ctrl as Mode2CtrlA, Pm,
};

/// Datetime represents an RTC clock/calendar value.
#[derive(Debug, Clone, Copy)]
pub struct Datetime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

type ClockR = crate::pac::rtc::mode2::clock::R;

impl From<ClockR> for Datetime {
    fn from(clock: ClockR) -> Datetime {
        Datetime {
            seconds: clock.second().bits(),
            minutes: clock.minute().bits(),
            hours: clock.hour().bits(),
            day: clock.day().bits(),
            month: clock.month().bits(),
            year: clock.year().bits(),
        }
    }
}

/// RtcMode represents the mode of the RTC
pub trait RtcMode: Sealed {}

/// ClockMode represents the Clock/Alarm mode
pub enum ClockMode {}

impl RtcMode for ClockMode {}
impl Sealed for ClockMode {}

/// Count32Mode represents the 32-bit counter mode.
///
/// This is a free running
/// count-up timer. When used in Periodic/CountDown mode with the embedded-hal
/// trait(s), it resets to zero on compare and starts counting up again.
pub enum Count32Mode {}

impl RtcMode for Count32Mode {}
impl Sealed for Count32Mode {}

#[cfg(feature = "sdmmc")]
impl From<Datetime> for Timestamp {
    fn from(clock: Datetime) -> Timestamp {
        Timestamp {
            year_since_1970: clock.year,
            zero_indexed_month: clock.month,
            zero_indexed_day: clock.day,
            hours: clock.hours,
            minutes: clock.minutes,
            seconds: clock.seconds,
        }
    }
}

/// Rtc represents the RTC peripheral for either clock/calendar or timer mode.
pub struct Rtc<Mode: RtcMode> {
    rtc: pac::Rtc,
    rtc_clock_freq: Hertz,
    _mode: PhantomData<Mode>,
}

#[hal_macro_helper]
impl<Mode: RtcMode> Rtc<Mode> {
    // --- Helper Functions for M0 vs M4 targets
    #[inline]
    fn mode0(&self) -> &Mode0 {
        self.rtc.mode0()
    }

    #[inline]
    fn mode2(&self) -> &Mode2 {
        self.rtc.mode2()
    }

    #[inline]
    fn mode0_ctrla(&self) -> &Mode0CtrlA {
        #[hal_cfg("rtc-d5x")]
        return self.mode0().ctrla();
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        return self.mode0().ctrl();
    }

    #[inline]
    fn mode2_ctrla(&self) -> &Mode2CtrlA {
        #[hal_cfg("rtc-d5x")]
        return self.mode2().ctrla();
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        return self.mode2().ctrl();
    }

    #[inline]
    fn sync(&self) {
        #[hal_cfg("rtc-d5x")]
        while self.mode2().syncbusy().read().bits() != 0 {}
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        while self.mode2().status().read().syncbusy().bit_is_set() {}
    }

    #[inline]
    fn reset(&mut self) {
        self.mode0_ctrla().modify(|_, w| w.swrst().set_bit());
        self.sync();
    }

    #[inline]
    fn enable(&mut self, enable: bool) {
        if enable {
            self.mode0_ctrla().modify(|_, w| w.enable().set_bit());
        } else {
            self.mode0_ctrla().modify(|_, w| w.enable().clear_bit());
        }
        self.sync();
    }

    fn create(rtc: pac::Rtc, rtc_clock_freq: Hertz) -> Self {
        Self {
            rtc,
            rtc_clock_freq,
            _mode: PhantomData,
        }
    }

    fn into_mode<M: RtcMode>(self) -> Rtc<M> {
        Rtc::create(self.rtc, self.rtc_clock_freq)
    }

    /// Reonfigures the peripheral for 32bit counter mode.
    pub fn into_count32_mode(mut self) -> Rtc<Count32Mode> {
        self.enable(false);
        self.sync();
        self.mode0_ctrla().modify(|_, w| {
            w.mode().count32() // enable mode2 (clock)
            .matchclr().clear_bit()
            .prescaler().div1() // No prescaler
        });
        self.sync();

        // enable clock sync on SAMx5x
        #[hal_cfg("rtc-d5x")]
        {
            self.mode2_ctrla().modify(|_, w| {
                w.clocksync().set_bit() // synchronize the CLOCK register
            });

            self.sync();
        }

        self.enable(true);
        self.into_mode()
    }

    /// Reconfigures the peripheral for clock/calendar mode. Requires the source
    /// clock to be running at 1024 Hz.
    pub fn into_clock_mode(mut self) -> Rtc<ClockMode> {
        // The max divisor is 1024, so to get 1 Hz, we need a 1024 Hz source.
        assert_eq!(
            self.rtc_clock_freq.to_Hz(),
            1024_u32,
            "RTC clk not 1024 Hz!"
        );

        self.sync();
        self.enable(false);
        self.sync();
        self.mode2_ctrla().modify(|_, w| {
            w.mode().clock() // enable mode2 (clock)
            .clkrep().clear_bit()
            .matchclr().clear_bit()
            .prescaler().div1024() // 1.024 kHz / 1024 = 1Hz
        });

        // enable clock sync on SAMx5x
        #[hal_cfg("rtc-d5x")]
        {
            self.mode2_ctrla().modify(|_, w| {
                w.clocksync().set_bit() // synchronize the CLOCK register
            });

            self.sync();
        }

        self.sync();
        self.enable(true);
        self.into_mode()
    }

    /// Releases the RTC resource
    pub fn free(self) -> pac::Rtc {
        self.rtc
    }
}

impl Rtc<Count32Mode> {
    /// Configures the RTC in 32-bit counter mode with no prescaler (default
    /// state after reset) and the counter initialized to zero.
    pub fn count32_mode(rtc: pac::Rtc, rtc_clock_freq: Hertz, pm: &mut Pm) -> Self {
        pm.apbamask().modify(|_, w| w.rtc_().set_bit());

        // TODO: This may not work properly because here the count sync bit is not set
        // as it is in Self::into_count32_mode Maybe we can just call that to
        // avoid code duplication

        let mut new_rtc = Self {
            rtc,
            rtc_clock_freq,
            _mode: PhantomData,
        };

        new_rtc.reset();
        new_rtc.enable(true);
        new_rtc
    }

    /// Returns the internal counter value.
    #[inline]
    #[hal_macro_helper]
    pub fn count32(&self) -> u32 {
        // synchronize this read on SAMD11/21. SAMx5x is automatically synchronized
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            self.mode0().readreq().modify(|_, w| w.rcont().set_bit());
            self.sync();
        }
        self.mode0().count().read().bits()
    }

    /// Sets the internal counter value.
    #[inline]
    pub fn set_count32(&mut self, count: u32) {
        self.sync();
        self.enable(false);

        self.sync();
        self.mode0()
            .count()
            .write(|w| unsafe { w.count().bits(count) });

        self.sync();
        self.enable(true);
    }

    /// This resets the internal counter and sets the prescaler to match the
    /// provided timeout. You should configure the prescaler using the longest
    /// timeout you plan to measure.
    pub fn reset_and_compute_prescaler<T: Into<<Self as ehal_02::timer::CountDown>::Time>>(
        &mut self,
        timeout: T,
    ) -> &Self {
        let params = TimerParams::new_us(timeout, self.rtc_clock_freq);
        let divider = params.divider;

        // Disable the timer while we reconfigure it
        self.sync();
        self.enable(false);

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        self.sync();
        self.reset();

        while self.mode0_ctrla().read().swrst().bit_is_set() {}

        self.mode0_ctrla().modify(|_, w| {
            // set clock divider...
            w.prescaler().variant(divider);
            // and enable RTC.
            w.enable().set_bit()
        });
        self
    }
}

impl Rtc<ClockMode> {
    pub fn clock_mode(rtc: pac::Rtc, rtc_clock_freq: Hertz, pm: &mut Pm) -> Self {
        Rtc::count32_mode(rtc, rtc_clock_freq, pm).into_clock_mode()
    }

    /// Returns the current clock/calendar value.
    #[hal_macro_helper]
    pub fn current_time(&self) -> Datetime {
        // synchronize this read on SAMD11/21. SAMx5x is automatically synchronized
        #[hal_cfg(any("rtc-d11", "rtc-d21"))]
        {
            self.mode2().readreq().modify(|_, w| w.rcont().set_bit());
            self.sync();
        }
        self.mode2().clock().read().into()
    }

    /// Updates the current clock/calendar value.
    pub fn set_time(&mut self, time: Datetime) {
        self.mode2().clock().write(|w| unsafe {
            w.second()
                .bits(time.seconds)
                .minute()
                .bits(time.minutes)
                .hour()
                .bits(time.hours)
                .day()
                .bits(time.day)
                .month()
                .bits(time.month)
                .year()
                .bits(time.year)
        });
        self.sync();
    }
}

// --- Timer / Counter Functionality

impl ehal_02::timer::Periodic for Rtc<Count32Mode> {}
impl ehal_02::timer::CountDown for Rtc<Count32Mode> {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        <Self as InterruptDrivenTimer>::start(self, timeout);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        // Unwrapping an unreacheable error is totally OK
        <Self as InterruptDrivenTimer>::wait(self).unwrap();
        Ok(())
    }
}

impl InterruptDrivenTimer for Rtc<Count32Mode> {
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.mode0().intenset().write(|w| w.cmp0().set_bit());
    }

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<NanosDurationU32>,
    {
        let params = TimerParams::new_us(timeout, self.rtc_clock_freq);
        let divider = params.divider;
        let cycles = params.cycles;

        // Disable the timer while we reconfigure it
        self.enable(false);

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        self.reset();
        while self.mode0_ctrla().read().swrst().bit_is_set() {}

        // set cycles to compare to...
        self.mode0()
            .comp(0)
            .write(|w| unsafe { w.comp().bits(cycles) });
        self.mode0_ctrla().modify(|_, w| {
            // set clock divider...
            w.prescaler().variant(divider);
            // clear timer on match for periodicity...
            w.matchclr().set_bit();
            // and enable RTC.
            w.enable().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Infallible> {
        if self.mode0().intflag().read().cmp0().bit_is_set() {
            // Writing a 1 clears the flag
            self.mode0().intflag().modify(|_, w| w.cmp0().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.mode0().intenclr().write(|w| w.cmp0().set_bit());
    }
}

#[cfg(feature = "sdmmc")]
impl TimeSource for Rtc<ClockMode> {
    fn get_timestamp(&self) -> Timestamp {
        self.current_time().into()
    }
}

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: Prescalerselect,
    pub cycles: u32,
}

impl TimerParams {
    /// calculates RTC timer paramters based on the input frequency-based
    /// timeout.
    pub fn new(timeout: impl Into<Hertz>, src_freq: impl Into<Hertz>) -> Self {
        let timeout = timeout.into();
        let src_freq = src_freq.into();
        let ticks: u32 = src_freq.to_Hz() / timeout.to_Hz().max(1);
        Self::new_from_ticks(ticks)
    }

    /// calculates RTC timer paramters based on the input period-based timeout.
    pub fn new_us(timeout: impl Into<Nanoseconds>, src_freq: impl Into<Hertz>) -> Self {
        let timeout = timeout.into();
        let src_freq = src_freq.into();
        let ticks: u32 =
            (timeout.to_nanos() as u64 * src_freq.to_Hz() as u64 / 1_000_000_000_u64) as u32;
        Self::new_from_ticks(ticks)
    }

    /// Common helper function that gets the best divider & calculates cycles
    /// with that divider.
    fn new_from_ticks(ticks: u32) -> Self {
        let divider_value = ((ticks >> 16) + 1).next_power_of_two();
        let divider = match divider_value {
            1 => Prescalerselect::Div1,
            2 => Prescalerselect::Div2,
            4 => Prescalerselect::Div4,
            8 => Prescalerselect::Div8,
            16 => Prescalerselect::Div16,
            32 => Prescalerselect::Div32,
            64 => Prescalerselect::Div64,
            128 => Prescalerselect::Div128,
            256 => Prescalerselect::Div256,
            512 => Prescalerselect::Div512,
            1024 => Prescalerselect::Div1024,
            _ => Prescalerselect::Div1024, /* would be nice to catch this at compile time
                                            * (rust-lang/rust#51999) */
        };

        let cycles: u32 = ticks / divider_value;

        TimerParams { divider, cycles }
    }
}
