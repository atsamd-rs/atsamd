//! Real-time clock/counter
use crate::target_device::rtc::{MODE0, MODE2};
use crate::target_device::RTC;
use crate::time::{Hertz, Nanoseconds};
use crate::timer_traits::InterruptDrivenTimer;
use crate::typelevel::Sealed;
use core::marker::PhantomData;
use hal::timer::{CountDown, Periodic};
use void::Void;

// SAMx5x imports
#[cfg(feature = "min-samd51g")]
use crate::target_device::{
    rtc::mode0::ctrla::PRESCALER_A, rtc::mode0::CTRLA as MODE0_CTRLA,
    rtc::mode2::CTRLA as MODE2_CTRLA, MCLK as PM,
};

// SAMD11/SAMD21 imports
#[cfg(any(feature = "samd11", feature = "samd21"))]
use crate::target_device::{
    rtc::mode0::ctrl::PRESCALER_A, rtc::mode0::CTRL as MODE0_CTRLA,
    rtc::mode2::CTRL as MODE2_CTRLA, PM,
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

type ClockR = crate::target_device::rtc::mode2::clock::R;

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

/// Count32Mode represents the 32-bit counter mode
pub enum Count32Mode {}

impl RtcMode for Count32Mode {}
impl Sealed for Count32Mode {}

/// DisabledMode represents the disabled (default) mode
pub enum DisabledMode {}

impl RtcMode for DisabledMode {}
impl Sealed for DisabledMode {}

/// Rtc represents the RTC peripheral for either clock/calendar or timer mode.
pub struct Rtc<Mode: RtcMode> {
    rtc: RTC,
    rtc_clock_freq: Hertz,
    _mode: PhantomData<Mode>,
}

impl<Mode: RtcMode> Rtc<Mode> {
    // --- Helper Functions for M0 vs M4 targets
    #[inline]
    fn mode0(&self) -> &MODE0 {
        self.rtc.mode0()
    }

    #[inline]
    fn mode2(&self) -> &MODE2 {
        self.rtc.mode2()
    }

    #[inline]
    fn mode0_ctrla(&self) -> &MODE0_CTRLA {
        #[cfg(feature = "min-samd51g")]
        return &self.mode0().ctrla;
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        return &self.mode0().ctrl;
    }

    #[inline]
    fn mode2_ctrla(&self) -> &MODE2_CTRLA {
        #[cfg(feature = "min-samd51g")]
        return &self.mode2().ctrla;
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        return &self.mode2().ctrl;
    }

    #[inline]
    fn sync(&self) {
        #[cfg(feature = "min-samd51g")]
        while self.mode2().syncbusy.read().bits() != 0 {}
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        while self.mode2().status.read().syncbusy().bit_is_set() {}
    }

    #[inline]
    fn reset(&self) {
        self.mode0_ctrla().modify(|_, w| w.swrst().set_bit());
        self.sync();
    }

    #[inline]
    fn enable(&self, enable: bool) {
        if enable {
            self.mode0_ctrla().modify(|_, w| w.enable().set_bit());
        } else {
            self.mode0_ctrla().modify(|_, w| w.enable().clear_bit());
        }
        self.sync();
    }

    fn create(rtc: RTC, rtc_clock_freq: Hertz) -> Self {
        Self {
            rtc,
            rtc_clock_freq,
            _mode: PhantomData,
        }
    }

    fn into_mode<M: RtcMode>(self) -> Rtc<M> {
        Rtc::create(self.rtc, self.rtc_clock_freq)
    }

    /// Configures the peripheral for 32bit counter mode.
    pub fn count32_mode(self) -> Rtc<Count32Mode> {
        self.mode0_ctrla().modify(|_, w| {
            w.mode().count32() // enable mode2 (clock)
            .matchclr().clear_bit()
            .prescaler().div1() // No prescaler
        });
        self.sync();

        // enable clock sync on SAMx5x
        #[cfg(feature = "min-samd51g")]
        {
            self.mode2_ctrla().modify(|_, w| {
                w.clocksync().set_bit() // synchronize the CLOCK register
            });

            self.sync();
        }

        self.enable(true);
        self.into_mode()
    }

    // --- clock functions

    /// Configures the peripheral for clock/calendar mode. Requires the source
    /// clock to be running at 1024 Hz.
    pub fn clock_mode(self) -> Rtc<ClockMode> {
        // The max divisor is 1024, so to get 1 Hz, we need a 1024 Hz source.
        assert_eq!(self.rtc_clock_freq.0, 1024_u32, "RTC clk not 1024 Hz!");

        self.mode2_ctrla().modify(|_, w| {
            w.mode().clock() // enable mode2 (clock)
            .clkrep().clear_bit()
            .matchclr().clear_bit()
            .prescaler().div1024() // 1.024 kHz / 1024 = 1Hz
        });
        self.sync();

        // enable clock sync on SAMx5x
        #[cfg(feature = "min-samd51g")]
        {
            self.mode2_ctrla().modify(|_, w| {
                w.clocksync().set_bit() // synchronize the CLOCK register
            });

            self.sync();
        }

        self.enable(true);
        self.into_mode()
    }
}

impl Rtc<DisabledMode> {
    /// Resets & does the basic configuration of the RTC peripheral,
    /// but doesn't configure it into a specific mode.
    pub fn new(rtc: RTC, rtc_clock_freq: Hertz, pm: &mut PM) -> Self {
        pm.apbamask.modify(|_, w| w.rtc_().set_bit());

        let new_rtc = Rtc {
            rtc,
            rtc_clock_freq,
            _mode: PhantomData,
        };

        new_rtc.reset();
        new_rtc
    }
}

impl Rtc<Count32Mode> {
    #[inline]
    fn count32(&self) -> u32 {
        // synchronize this read on SAMD11/21. SAMx5x is automatically synchronized
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        {
            self.mode0().readreq.modify(|_, w| w.rcont().set_bit());
            self.sync();
        }
        self.mode0().count.read().bits()
    }
}

impl Rtc<ClockMode> {
    // --- timer functions

    /// Returns the current clock/calendar value.
    pub fn current_time(&self) -> Datetime {
        // synchronize this read on SAMD11/21. SAMx5x is automatically synchronized
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        {
            self.mode2().readreq.modify(|_, w| w.rcont().set_bit());
            self.sync();
        }
        self.mode2().clock.read().into()
    }

    /// Updates the current clock/calendar value.
    pub fn set_time(&mut self, time: Datetime) {
        self.mode2().clock.write(|w| unsafe {
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

impl Periodic for Rtc<DisabledMode> {}
impl CountDown for Rtc<DisabledMode> {
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        let params = TimerParams::new_us(timeout, self.rtc_clock_freq.0);
        let divider = params.divider;
        let cycles = params.cycles;

        // Disable the timer while we reconfigure it
        self.enable(false);

        // 32 bit counter mode plz
        self.mode0_ctrla().modify(|_, w| w.mode().count32());

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        self.reset();

        while self.mode0_ctrla().read().swrst().bit_is_set() {}

        // set cycles to compare to...
        self.mode0().comp[0].write(|w| unsafe { w.comp().bits(cycles) });
        self.mode0_ctrla().modify(|_, w| {
            // set clock divider...
            w.prescaler().variant(divider);
            // clear timer on match...
            w.matchclr().set_bit();
            // and enable RTC.
            w.enable().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.mode0().intflag.read().cmp0().bit_is_set() {
            // Writing a 1 clears the flag
            self.mode0().intflag.modify(|_, w| w.cmp0().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl InterruptDrivenTimer for Rtc<DisabledMode> {
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.mode0().intenset.write(|w| w.cmp0().set_bit());
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.mode0().intenclr.write(|w| w.cmp0().set_bit());
    }
}

/// Helper type for computing cycles and divider given frequency
#[derive(Debug, Clone, Copy)]
pub struct TimerParams {
    pub divider: PRESCALER_A,
    pub cycles: u32,
}

impl TimerParams {
    /// calculates RTC timer paramters based on the input frequency-based
    /// timeout.
    pub fn new<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Hertz>,
    {
        let timeout = timeout.into();
        let ticks: u32 = src_freq / timeout.0.max(1);
        TimerParams::new_from_ticks(ticks)
    }

    /// calculates RTC timer paramters based on the input period-based timeout.
    pub fn new_us<T>(timeout: T, src_freq: u32) -> Self
    where
        T: Into<Nanoseconds>,
    {
        let timeout = timeout.into();
        let ticks: u32 = (timeout.0 as u64 * src_freq as u64 / 1_000_000_000_u64) as u32;
        Self::new_from_ticks(ticks)
    }

    /// Common helper function that gets the best divider & calculates cycles
    /// with that divider.
    fn new_from_ticks(ticks: u32) -> Self {
        let divider_value = ((ticks >> 16) + 1).next_power_of_two();
        let divider = match divider_value {
            1 => PRESCALER_A::DIV1,
            2 => PRESCALER_A::DIV2,
            4 => PRESCALER_A::DIV4,
            8 => PRESCALER_A::DIV8,
            16 => PRESCALER_A::DIV16,
            32 => PRESCALER_A::DIV32,
            64 => PRESCALER_A::DIV64,
            128 => PRESCALER_A::DIV128,
            256 => PRESCALER_A::DIV256,
            512 => PRESCALER_A::DIV512,
            1024 => PRESCALER_A::DIV1024,
            _ => PRESCALER_A::DIV1024, /* would be nice to catch this at compile time
                                        * (rust-lang/rust#51999) */
        };

        let cycles: u32 = ticks / divider_value as u32;

        TimerParams { divider, cycles }
    }
}
