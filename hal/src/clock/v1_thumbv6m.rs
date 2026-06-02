//! Configuring the system clock sources.
//!
//! You will typically need to create an instance of `GenericClockController`
//! before you can set up most of the peripherals on the atsamd21 device.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
#![allow(clippy::from_over_into)]

use atsamd_hal_macros::{hal_cfg, hal_macro_helper};

use fugit::RateExtU32;

use crate::pac::gclk::clkctrl::Genselect::*;
use crate::pac::gclk::clkctrl::Idselect::*;
use crate::pac::gclk::genctrl::Srcselect::*;
use crate::pac::{self, Gclk, Nvmctrl, Pm, Sysctrl};
use crate::time::Hertz;

pub type ClockId = pac::gclk::clkctrl::Idselect;
pub type ClockGenId = pac::gclk::clkctrl::Genselect;
pub type ClockSource = pac::gclk::genctrl::Srcselect;

/// Represents a configured clock generator.
///
/// Can be converted into the effective clock frequency.
/// Its primary purpose is to be passed in to methods
/// such as `GenericClockController::tcc2_tc3` to configure
/// the clock for a peripheral.
#[derive(Clone, Copy)]
pub struct GClock {
    gclk: ClockGenId,
    freq: Hertz,
}

impl Into<Hertz> for GClock {
    fn into(self) -> Hertz {
        self.freq
    }
}

struct State {
    gclk: Gclk,
}

impl State {
    fn reset_gclk(&mut self) {
        self.gclk.ctrl().write(|w| w.swrst().set_bit());
        while self.gclk.ctrl().read().swrst().bit_is_set()
            || self.gclk.status().read().syncbusy().bit_is_set()
        {}
    }

    fn wait_for_sync(&mut self) {
        while self.gclk.status().read().syncbusy().bit_is_set() {}
    }

    fn set_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) {
        // validate the divisor factor based on gclk ID (samd21 see 15.8.5, for samd11
        // see 14.8.5)
        let mut divisor_invalid = false;
        if gclk == Gclk1 {
            if divider as u32 >= 2_u32.pow(16) {
                divisor_invalid = true;
            }
        } else if gclk == Gclk2 {
            if divider >= 2_u16.pow(5) {
                divisor_invalid = true;
            }
        } else if divider >= 2_u16.pow(8) {
            divisor_invalid = true;
        }
        if divisor_invalid {
            panic!("invalid divisor {} for Gclk {}", divider, gclk as u8);
        }

        self.gclk.gendiv().write(|w| unsafe {
            w.id().bits(u8::from(gclk));
            w.div().bits(divider)
        });
        self.wait_for_sync();

        self.gclk.genctrl().write(|w| unsafe {
            w.id().bits(u8::from(gclk));
            w.src().bits(u8::from(src));
            // divide directly by divider, rather than exponential
            w.divsel().clear_bit();
            w.idc().bit(improve_duty_cycle);
            w.genen().set_bit();
            w.oe().set_bit()
        });
        self.wait_for_sync();
    }

    fn enable_clock_generator(&mut self, clock: ClockId, generator: ClockGenId) {
        self.gclk.clkctrl().write(|w| unsafe {
            w.id().bits(u8::from(clock));
            w.r#gen().bits(u8::from(generator));
            w.clken().set_bit()
        });
        self.wait_for_sync();
    }

    fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        // We must first read out the configuration of genctrl to read/modify/write it.
        //   To do so, we must do an 8-bit write to GENCTRL.ID (ref 15.6.4.1 Indirect
        //   Access). 32-bit write did not work.
        unsafe {
            let genctrl_ptr_u8: *mut u8 = self.gclk.genctrl().as_ptr() as *mut u8;
            *genctrl_ptr_u8 = u8::from(gclk);
        }
        self.wait_for_sync();

        // Now that the configuration is loaded, modify it
        self.gclk.genctrl().modify(|_, w| w.runstdby().bit(enable));
        self.wait_for_sync();
    }
}

/// `GenericClockController` encapsulates the Gclk hardware.
///
/// It provides a type safe way to configure the system clocks.
/// Initializing the `GenericClockController` instance configures
/// the system to run at 48Mhz by setting gclk1 as a 32khz source
/// and feeding it into the Dfll48 hardware which in turn drives
/// gclk0 at 48Mhz.
pub struct GenericClockController {
    state: State,
    gclks: [Hertz; 8],
    used_clocks: u64,
}

impl GenericClockController {
    /// Reset the clock controller, configure the system to run
    /// at 48Mhz and reset various clock dividers.
    pub fn with_internal_32kosc(
        gclk: Gclk,
        pm: &mut Pm,
        sysctrl: &mut Sysctrl,
        nvmctrl: &mut Nvmctrl,
    ) -> Self {
        Self::new_48mhz_from_32khz(gclk, pm, sysctrl, nvmctrl, false)
    }

    /// Reset the clock controller, configure the system to run
    /// at 48Mhz and reset various clock dividers.
    pub fn with_external_32kosc(
        gclk: Gclk,
        pm: &mut Pm,
        sysctrl: &mut Sysctrl,
        nvmctrl: &mut Nvmctrl,
    ) -> Self {
        Self::new_48mhz_from_32khz(gclk, pm, sysctrl, nvmctrl, true)
    }

    #[hal_macro_helper]
    fn new_48mhz_from_32khz(
        gclk: Gclk,
        pm: &mut Pm,
        sysctrl: &mut Sysctrl,
        nvmctrl: &mut Nvmctrl,
        use_external_crystal: bool,
    ) -> Self {
        let mut state = State { gclk };

        set_flash_to_half_auto_wait_state(nvmctrl);
        #[hal_cfg("clock-d21")]
        set_flash_manual_write(nvmctrl);
        enable_gclk_apb(pm);
        if use_external_crystal {
            enable_external_32kosc(sysctrl);
        } else {
            enable_internal_32kosc(sysctrl);
        }

        state.reset_gclk();

        // Enable a 32khz source -> Gclk1
        if use_external_crystal {
            state.set_gclk_divider_and_source(Gclk1, 1, Xosc32k, false);
        } else {
            state.set_gclk_divider_and_source(Gclk1, 1, Osc32k, false);
        }

        // Feed 32khz into the Dfll48
        state.enable_clock_generator(Dfll48, Gclk1);
        // Enable the Dfll48
        configure_and_enable_dfll48m(sysctrl, use_external_crystal);
        // Feed Dfll48 into the main clock
        state.set_gclk_divider_and_source(Gclk0, 1, Dfll48m, true);
        // We are now running at 48Mhz

        // Reset various dividers back to 1
        sysctrl.osc8m().modify(|_, w| {
            w.presc()._0();
            w.ondemand().clear_bit()
        });
        pm.cpusel().write(|w| w.cpudiv().div1());
        pm.apbasel().write(|w| w.apbadiv().div1());
        pm.apbbsel().write(|w| w.apbbdiv().div1());
        pm.apbcsel().write(|w| w.apbcdiv().div1());

        Self {
            state,
            gclks: [
                OSC48M_FREQ,
                OSC32K_FREQ,
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
            ],
            used_clocks: 1u64 << u8::from(ClockId::Dfll48),
        }
    }

    /// Reset the clock controller, configure the system to run at 8Mhz from
    /// internal 8 MHz RC clock (no PLL) and reset various clock dividers.
    #[hal_macro_helper]
    pub fn with_internal_8mhz(
        gclk: Gclk,
        pm: &mut Pm,
        sysctrl: &mut Sysctrl,
        nvmctrl: &mut Nvmctrl,
    ) -> Self {
        let mut state = State { gclk };

        // No wait states needed <= 24 MHz @ 3.3v (ref. 37.12 NVM characteristics)
        #[hal_cfg("clock-d21")]
        set_flash_manual_write(nvmctrl);

        // Get rid of unused warning
        #[hal_cfg("clock-d11")]
        let _ = nvmctrl;

        enable_gclk_apb(pm);

        state.reset_gclk();

        // Enable 8 MHz source -> Gclk0
        state.set_gclk_divider_and_source(Gclk0, 1, Osc8m, false);

        // Reset various dividers back to 1
        sysctrl.osc8m().modify(|_, w| {
            w.presc()._0();
            w.ondemand().clear_bit()
        });
        pm.cpusel().write(|w| w.cpudiv().div1());
        pm.apbasel().write(|w| w.apbadiv().div1());
        pm.apbbsel().write(|w| w.apbbdiv().div1());
        pm.apbcsel().write(|w| w.apbcdiv().div1());

        Self {
            state,
            gclks: [
                OSC8M_FREQ,
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
                0.Hz(),
            ],
            used_clocks: 0,
        }
    }

    /// Returns a `GClock` for gclk0, the system clock generator at 48Mhz
    pub fn gclk0(&mut self) -> GClock {
        GClock {
            gclk: Gclk0,
            freq: self.gclks[0],
        }
    }

    /// Returns a `GClock` for gclk1, the 32Khz oscillator.
    pub fn gclk1(&mut self) -> GClock {
        GClock {
            gclk: Gclk1,
            freq: self.gclks[1],
        }
    }

    /// Returns the `GClock` for the specified clock generator.
    /// If that clock generator has not yet been configured,
    /// returns None.
    pub fn get_gclk(&mut self, gclk: ClockGenId) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].to_Hz() == 0 {
            None
        } else {
            Some(GClock {
                gclk,
                freq: self.gclks[idx],
            })
        }
    }

    /// Configures a clock generator with the specified divider and
    /// source.
    /// `divider` is a linear divider to be applied to the clock
    /// source.  While the hardware also supports an exponential divider,
    /// this function doesn't expose that functionality at this time.
    /// `improve_duty_cycle` is a boolean that, when set to true, enables
    /// a 5o/50 duty cycle for odd divider values.
    /// Returns a `GClock` for the configured clock generator.
    /// Returns `None` if the clock generator has already been configured.
    pub fn configure_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].to_Hz() != 0 {
            return None;
        }
        self.state
            .set_gclk_divider_and_source(gclk, divider, src, improve_duty_cycle);
        let freq: Hertz = match src {
            Xosc32k | Osc32k | Osculp32k => OSC32K_FREQ,
            Gclkgen1 => self.gclks[1],
            Osc8m => OSC8M_FREQ,
            Dfll48m => OSC48M_FREQ,
            Dpll96m => 96.MHz(),
            Gclkin | Xosc => unimplemented!(),
        };
        self.gclks[idx] = freq / divider as u32;
        Some(GClock { gclk, freq })
    }

    /// Enables or disables the given GClk from operation in standby.
    pub fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        self.state.configure_standby(gclk, enable)
    }
}

macro_rules! clock_generator {
    ($(($id:ident, $Type:ident, $clock:ident),)+) => {

$(
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
///
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
#[derive(Debug)]
pub struct $Type {
    freq: Hertz,
}

impl $Type {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}
impl Into<Hertz> for $Type {
    fn into(self) -> Hertz {
        self.freq
    }
}
)+

impl GenericClockController {
    $(
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    ///
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn $id(&mut self, generator: &GClock) -> Option<$Type> {
        let bits: u64 = 1<<u8::from(ClockId::$clock) as u64;
        if (self.used_clocks & bits) != 0 {
            return None;
        }
        self.used_clocks |= bits;

        self.state.enable_clock_generator(ClockId::$clock, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some($Type{freq})
    }
    )+
}
    }
}

// samd11
#[hal_cfg("clock-d11")]
clock_generator!(
    (tcc0, Tcc0Clock, Tcc0),
    (tc1_tc2, Tc1Tc2Clock, Tc1Tc2),
    (sercom0_core, Sercom0CoreClock, Sercom0Core),
    (sercom1_core, Sercom1CoreClock, Sercom1Core),
    (sercom2_core, Sercom2CoreClock, Sercom2Core),
    (rtc, RtcClock, Rtc),
    (adc, AdcClock, Adc),
    (wdt, WdtClock, Wdt),
    (eic, EicClock, Eic),
    (usb, UsbClock, Usb),
    (evsys0, Evsys0Clock, Evsys0),
    (evsys1, Evsys1Clock, Evsys1),
    (evsys2, Evsys2Clock, Evsys2),
    (evsys3, Evsys3Clock, Evsys3),
    (evsys4, Evsys4Clock, Evsys4),
    (evsys5, Evsys5Clock, Evsys5),
    (ac_ana, AcAnaClock, AcAna),
    (ac_dig, AcDigClock, AcDig),
    (dac, DacClock, Dac),
);
// samd21
#[hal_cfg("clock-d21")]
clock_generator!(
    (tcc0_tcc1, Tcc0Tcc1Clock, Tcc0Tcc1),
    (tcc2_tc3, Tcc2Tc3Clock, Tcc2Tc3),
    (tc4_tc5, Tc4Tc5Clock, Tc4Tc5),
    (tc6_tc7, Tc6Tc7Clock, Tc6Tc7),
    (sercom0_core, Sercom0CoreClock, Sercom0Core),
    (sercom1_core, Sercom1CoreClock, Sercom1Core),
    (sercom2_core, Sercom2CoreClock, Sercom2Core),
    (sercom3_core, Sercom3CoreClock, Sercom3Core),
    (sercom4_core, Sercom4CoreClock, Sercom4Core),
    (sercom5_core, Sercom5CoreClock, Sercom5Core),
    (usb, UsbClock, Usb),
    (rtc, RtcClock, Rtc),
    (adc, AdcClock, Adc),
    (wdt, WdtClock, Wdt),
    (eic, EicClock, Eic),
    (evsys0, Evsys0Clock, Evsys0),
    (evsys1, Evsys1Clock, Evsys1),
    (evsys2, Evsys2Clock, Evsys2),
    (evsys3, Evsys3Clock, Evsys3),
    (evsys4, Evsys4Clock, Evsys4),
    (evsys5, Evsys5Clock, Evsys5),
    (evsys6, Evsys6Clock, Evsys6),
    (evsys7, Evsys7Clock, Evsys7),
    (evsys8, Evsys8Clock, Evsys8),
    (evsys9, Evsys9Clock, Evsys9),
    (evsys10, Evsys10Clock, Evsys10),
    (evsys11, Evsys11Clock, Evsys11),
    (ac_ana, AcAnaClock, AcAna),
    (ac_dig, AcDigClock, AcDig),
    (dac, DacClock, Dac),
    (i2s0, I2S0Clock, I2s0),
    (i2s1, I2S1Clock, I2s1),
);

/// The frequency of the 48Mhz source.
pub const OSC48M_FREQ: Hertz = Hertz::Hz(48_000_000);
/// The frequency of the 8 Mhz source.
pub const OSC8M_FREQ: Hertz = Hertz::Hz(8_000_000);
/// The frequency of the 32Khz source.
pub const OSC32K_FREQ: Hertz = Hertz::Hz(32_768);

fn set_flash_to_half_auto_wait_state(nvmctrl: &mut Nvmctrl) {
    nvmctrl.ctrlb().modify(|_, w| w.rws().half());
}

/// Prevent automatic writes to flash by pointers to flash area
#[hal_cfg("clock-d21")]
fn set_flash_manual_write(nvmctrl: &mut Nvmctrl) {
    nvmctrl.ctrlb().modify(|_, w| w.manw().set_bit());
}

fn enable_gclk_apb(pm: &mut Pm) {
    pm.apbamask().modify(|_, w| w.gclk_().set_bit());
}

/// Turn on the internal 32hkz oscillator
pub fn enable_internal_32kosc(sysctrl: &mut Sysctrl) {
    let calibration = crate::calibration::osc32k_cal();
    sysctrl.osc32k().write(|w| {
        unsafe {
            w.ondemand().clear_bit();
            w.calib().bits(calibration);
            // 6 here means: use 66 cycles of OSC32k to start up this oscillator
            w.startup().bits(6);
        }
        w.en32k().set_bit();
        w.enable().set_bit();
        w.runstdby().set_bit()
    });
    while sysctrl.pclksr().read().osc32krdy().bit_is_clear() {
        // Wait for the oscillator to stabilize
    }
}

/// Turn on the external 32hkz oscillator
pub fn enable_external_32kosc(sysctrl: &mut Sysctrl) {
    sysctrl.xosc32k().modify(|_, w| {
        unsafe {
            // 6 here means: use 64k cycles of OSCULP32k to start up this oscillator
            w.startup().bits(6);
        }
        w.ondemand().clear_bit();
        // Enable 32khz output
        w.en32k().set_bit();
        // Crystal connected to xin32/xout32
        w.xtalen().set_bit();
        w.runstdby().set_bit()
    });
    sysctrl.xosc32k().modify(|_, w| w.enable().set_bit());
    while sysctrl.pclksr().read().xosc32krdy().bit_is_clear() {
        // Wait for the oscillator to stabilize
    }
}

fn wait_for_dfllrdy(sysctrl: &mut Sysctrl) {
    while sysctrl.pclksr().read().dfllrdy().bit_is_clear() {}
}

/// Configure the dfll48m to operate at 48Mhz
#[hal_macro_helper]
fn configure_and_enable_dfll48m(sysctrl: &mut Sysctrl, use_external_crystal: bool) {
    // Turn it off while we configure it.
    // Note that we need to turn off on-demand mode and
    // disable it here, rather than just reseting the ctrl
    // register, otherwise our configuration attempt fails.
    sysctrl.dfllctrl().write(|w| w.ondemand().clear_bit());
    wait_for_dfllrdy(sysctrl);

    if use_external_crystal {
        sysctrl.dfllmul().write(|w| unsafe {
            w.cstep().bits(31);
            w.fstep().bits(511);
            // scaling factor between the clocks
            w.mul().bits(((48_000_000u32 + 32768 / 2) / 32768) as u16)
        });

        // Turn it on
        sysctrl.dfllctrl().write(|w| {
            // always on
            w.ondemand().clear_bit();

            // closed loop mode
            w.mode().set_bit();

            w.waitlock().set_bit();

            // Disable quick lock
            w.qldis().set_bit()
        });
    } else {
        // Apply calibration
        let coarse = crate::calibration::dfll48m_coarse_cal();
        let fine = 0x1ff;

        sysctrl.dfllval().write(|w| unsafe {
            w.coarse().bits(coarse);
            w.fine().bits(fine)
        });

        sysctrl.dfllmul().write(|w| unsafe {
            w.cstep().bits(coarse / 4);
            w.fstep().bits(10);
            // scaling factor for 1 kHz Usb SOF signal
            w.mul().bits((48_000_000u32 / 1000) as u16)
        });

        // Turn it on
        sysctrl.dfllctrl().write(|w| {
            // always on
            w.ondemand().clear_bit();

            // closed loop mode
            w.mode().set_bit();

            // chill cycle disable
            w.ccdis().set_bit();

            // usb correction
            w.usbcrm().set_bit();

            // bypass coarse lock (have calibration data)
            w.bplckc().set_bit()
        });
    }

    wait_for_dfllrdy(sysctrl);

    // and finally enable it!
    sysctrl.dfllctrl().modify(|_, w| w.enable().set_bit());

    #[hal_cfg("clock-d21")]
    if use_external_crystal {
        // wait for lock
        while sysctrl.pclksr().read().dflllckc().bit_is_clear()
            || sysctrl.pclksr().read().dflllckf().bit_is_clear()
        {}
    }

    wait_for_dfllrdy(sysctrl);
}
