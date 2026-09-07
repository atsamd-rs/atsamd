//! Real-time clock/counter
use atsamd_hal_macros::hal_cfg;
use fugit::ExtU32;

use crate::ehal;
use crate::ehal_02;
use crate::pac;
use crate::time::{Hertz, Nanoseconds};
use crate::timer_traits::InterruptDrivenTimer;
use crate::typelevel::Sealed;
use core::convert::Infallible;
use core::marker::PhantomData;
use modes::{
    RtcMode as _,
    mode0::{Compare0, RtcMode0},
    mode2::RtcMode2,
};

#[cfg(feature = "sdmmc")]
use embedded_sdmmc::{TimeSource, Timestamp};

mod modes;

#[cfg(feature = "rtic")]
pub mod rtic;

#[cfg(feature = "embassy-time")]
pub mod embassy;

// SAMx5x imports
#[hal_cfg("rtc-d5x")]
use crate::pac::{Mclk as Pm, rtc::mode0::ctrla::Prescalerselect};

// SAMD11/SAMD21 imports
#[hal_cfg(any("rtc-d11", "rtc-d21"))]
use crate::pac::{Pm, rtc::mode0::ctrl::Prescalerselect};

pub use modes::mode2::Datetime;

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

/// Represents the RTC peripheral for either clock/calendar or timer mode.
pub struct Rtc<Mode: RtcMode> {
    rtc: pac::Rtc,
    rtc_clock_freq: Hertz,
    _mode: PhantomData<Mode>,
}

impl<Mode: RtcMode> Rtc<Mode> {
    fn set_count32_mode(rtc: &pac::Rtc) {
        RtcMode0::disable(rtc);
        RtcMode0::reset(rtc);
        RtcMode0::set_mode(rtc);
        RtcMode0::start_and_initialize(rtc);
    }

    fn set_clock_mode(rtc: &pac::Rtc, rtc_clock_freq: Hertz) {
        let divider = TimerParams::prescaler_from_divider(rtc_clock_freq.raw() as u64)
            .unwrap_or_else(|| {
                panic!("cannot use clock mode with an RTC clock rate of {rtc_clock_freq}")
            });

        RtcMode2::disable(rtc);
        RtcMode2::reset(rtc);
        RtcMode2::set_mode(rtc);
        RtcMode2::set_prescaler(rtc, divider);
        RtcMode2::start_and_initialize(rtc);
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

    /// Reconfigures the RTC for 32-bit counter mode with no prescaler (default
    /// state after reset) and the counter initialized to zero and started.
    pub fn into_count32_mode(self) -> Rtc<Count32Mode> {
        Self::set_count32_mode(&self.rtc);
        self.into_mode()
    }

    /// Reconfigures the peripheral for clock/calendar mode. Requires the source
    /// clock to be running at 1024 Hz.
    pub fn into_clock_mode(self) -> Rtc<ClockMode> {
        Self::set_clock_mode(&self.rtc, self.rtc_clock_freq);
        self.into_mode()
    }

    /// Releases the RTC resource
    pub fn free(self) -> pac::Rtc {
        self.rtc
    }
}

impl Rtc<Count32Mode> {
    /// Configures the RTC in 32-bit counter mode with no prescaler (default
    /// state after reset) and the counter initialized to zero and started.
    pub fn count32_mode(rtc: pac::Rtc, rtc_clock_freq: Hertz, pm: &mut Pm) -> Self {
        RtcMode0::disable(&rtc);

        // Enable the RTC clock.
        // TODO: This will probably be done eventually using the clock v2 API instead.
        // This should also allow automatic determination of the clock rate.
        pm.apbamask().modify(|_, w| w.rtc_().set_bit());

        Self::set_count32_mode(&rtc);

        Self {
            rtc,
            rtc_clock_freq,
            _mode: PhantomData,
        }
    }

    /// Returns the internal counter value.
    #[inline]
    pub fn count32(&self) -> u32 {
        RtcMode0::count(&self.rtc)
    }

    /// Sets the internal counter value.
    #[inline]
    pub fn set_count32(&mut self, count: u32) {
        RtcMode0::disable(&self.rtc);
        RtcMode0::set_count(&self.rtc, count);
        RtcMode0::enable(&self.rtc);
    }

    /// This resets the internal counter and sets the prescaler to match the
    /// provided timeout.
    fn reset_and_set_prescaler(&mut self, divider: Prescalerselect) {
        RtcMode0::disable(&self.rtc);
        RtcMode0::reset(&self.rtc);
        RtcMode0::set_mode(&self.rtc);
        RtcMode0::set_prescaler(&self.rtc, divider);
    }

    /// This resets the internal counter, sets the prescaler to match the
    /// provided timeout, and starts the counter. You should configure the
    /// prescaler using the longest timeout you plan to measure.
    pub fn reset_and_compute_prescaler<T: Into<Nanoseconds>>(&mut self, timeout: T) -> &Self {
        let params = TimerParams::new_us(timeout, self.rtc_clock_freq);
        let divider = params.divider;

        self.reset_and_set_prescaler(divider);
        RtcMode0::start_and_initialize(&self.rtc);

        self
    }
}

impl Rtc<ClockMode> {
    pub fn clock_mode(rtc: pac::Rtc, rtc_clock_freq: Hertz, pm: &mut Pm) -> Self {
        Rtc::count32_mode(rtc, rtc_clock_freq, pm).into_clock_mode()
    }

    /// Returns the current clock/calendar value.
    pub fn current_time(&self) -> Datetime {
        RtcMode2::count(&self.rtc)
    }

    /// Updates the current clock/calendar value.
    pub fn set_time(&mut self, time: Datetime) {
        RtcMode2::set_count(&self.rtc, time);
    }
}

// --- Timer / Counter Functionality
impl ehal_02::timer::Periodic for Rtc<Count32Mode> {}
impl ehal_02::timer::CountDown for Rtc<Count32Mode> {
    type Time = Nanoseconds;

    /// Starts the timer and puts it in periodic mode in which the counter
    /// counts up to the specified `timeout` and then resets repeatedly back
    /// to zero.
    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        <Self as InterruptDrivenTimer>::start(self, timeout);
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        <Self as InterruptDrivenTimer>::wait(self).map_err(|e| e.map(|_| panic!()))
    }
}

impl ehal::delay::DelayNs for Rtc<Count32Mode> {
    fn delay_ns(&mut self, ns: u32) {
        <Self as InterruptDrivenTimer>::start(self, ns.nanos());
        // Note that this cannot error since the error type is `Infallible`
        let _ = nb::block!(<Self as InterruptDrivenTimer>::wait(self));
    }
}

impl InterruptDrivenTimer for Rtc<Count32Mode> {
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        RtcMode0::enable_interrupt::<Compare0>(&self.rtc);
    }

    /// Starts the timer and puts it in periodic mode in which the counter
    /// counts up to the specified `timeout` and then resets repeatedly back
    /// to zero.
    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Nanoseconds>,
    {
        let params = TimerParams::new_us(timeout, self.rtc_clock_freq);
        let divider = params.divider;
        let cycles = params.cycles;

        // Reset and set prescaler, keeping the counter disabled
        self.reset_and_set_prescaler(divider);
        // Set the compare 0 value to the desired time
        RtcMode0::set_compare(&self.rtc, 0, cycles);
        // Clear on match for periodicity
        RtcMode0::set_match_clear(&self.rtc, true);

        // Start the counter
        RtcMode0::start_and_initialize(&self.rtc);
    }

    fn wait(&mut self) -> nb::Result<(), Infallible> {
        if RtcMode0::check_interrupt_flag::<Compare0>(&self.rtc) {
            // Clear the flag
            RtcMode0::clear_interrupt_flag::<Compare0>(&self.rtc);
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
        RtcMode0::disable_interrupt::<Compare0>(&self.rtc);
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
    fn prescaler_from_divider(divider_value: u64) -> Option<Prescalerselect> {
        match divider_value {
            1 => Some(Prescalerselect::Div1),
            2 => Some(Prescalerselect::Div2),
            4 => Some(Prescalerselect::Div4),
            8 => Some(Prescalerselect::Div8),
            16 => Some(Prescalerselect::Div16),
            32 => Some(Prescalerselect::Div32),
            64 => Some(Prescalerselect::Div64),
            128 => Some(Prescalerselect::Div128),
            256 => Some(Prescalerselect::Div256),
            512 => Some(Prescalerselect::Div512),
            1024 => Some(Prescalerselect::Div1024),
            _ => None,
        }
    }

    /// calculates RTC timer parameters based on the input frequency-based
    /// timeout.
    pub fn new(timeout: impl Into<Hertz>, src_freq: impl Into<Hertz>) -> Self {
        let timeout = timeout.into();
        let src_freq = src_freq.into();
        let ticks = src_freq.to_Hz() / timeout.to_Hz().max(1);
        Self::new_from_ticks(ticks as u64)
    }

    /// calculates RTC timer parameters based on the input period-based timeout.
    pub fn new_us(timeout: impl Into<Nanoseconds>, src_freq: impl Into<Hertz>) -> Self {
        let timeout = timeout.into();
        let src_freq = src_freq.into();
        let ticks = timeout.to_nanos() as u64 * src_freq.to_Hz() as u64 / 1_000_000_000;
        Self::new_from_ticks(ticks)
    }

    /// Common helper function that gets the best divider & calculates cycles
    /// with that divider.
    fn new_from_ticks(ticks: u64) -> Self {
        let mut divider_value = ((ticks >> 16) + 1).next_power_of_two();
        let divider = Self::prescaler_from_divider(divider_value).unwrap_or_else(|| {
            divider_value = 1024;

            Prescalerselect::Div1024
        });

        let cycles: u32 = (ticks / divider_value)
            .try_into()
            .expect("cannot achieve the timeout even with the maximum RTC prescaler");

        TimerParams { divider, cycles }
    }
}
