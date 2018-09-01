//! Configuring the system clock sources.
//! You will typically need to create an instance of `GenericClockController`
//! before you can set up most of the peripherals on the atsamd21 device.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
use target_device::gclk::clkctrl::GENR::*;
use target_device::gclk::clkctrl::IDR::*;
use target_device::gclk::genctrl::SRCR::*;
use target_device::{self, GCLK, NVMCTRL, PM, SYSCTRL};
use time::{Hertz, U32Ext};

pub type ClockId = target_device::gclk::clkctrl::IDR;
pub type ClockGenId = target_device::gclk::clkctrl::GENR;
pub type ClockSource = target_device::gclk::genctrl::SRCR;

/// Represents a configured clock generator.
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
    gclk: GCLK,
}

impl State {
    fn reset_gclk(&mut self) {
        self.gclk.ctrl.write(|w| w.swrst().set_bit());
        while self.gclk.ctrl.read().swrst().bit_is_set()
            || self.gclk.status.read().syncbusy().bit_is_set()
        {}
    }

    fn wait_for_sync(&mut self) {
        while self.gclk.status.read().syncbusy().bit_is_set() {}
    }

    fn set_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) {
        self.gclk.gendiv.write(|w| unsafe {
            w.id().bits(gclk.bits());
            w.div().bits(divider)
        });
        self.wait_for_sync();

        self.gclk.genctrl.write(|w| unsafe {
            w.id().bits(gclk.bits());
            w.src().bits(src.bits());
            // divide directly by divider, rather than exponential
            w.divsel().clear_bit();
            w.idc().bit(improve_duty_cycle);
            w.genen().set_bit()
        });
        self.wait_for_sync();
    }

    fn enable_clock_generator(&mut self, clock: ClockId, generator: ClockGenId) {
        self.gclk.clkctrl.write(|w| unsafe {
            w.id().bits(clock.bits());
            w.gen().bits(generator.bits());
            w.clken().set_bit()
        });
        self.wait_for_sync();
    }
}

/// `GenericClockController` encapsulates the GCLK hardware.
/// It provides a type safe way to configure the system clocks.
/// Initializing the `GenericClockController` instance configures
/// the system to run at 48Mhz by setting gclk1 as a 32khz source
/// and feeding it into the DFLL48 hardware which in turn drives
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
        gclk: GCLK,
        pm: &mut PM,
        sysctrl: &mut SYSCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, pm, sysctrl, nvmctrl, false)
    }

    /// Reset the clock controller, configure the system to run
    /// at 48Mhz and reset various clock dividers.
    pub fn with_external_32kosc(
        gclk: GCLK,
        pm: &mut PM,
        sysctrl: &mut SYSCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, pm, sysctrl, nvmctrl, true)
    }

    fn new(
        gclk: GCLK,
        pm: &mut PM,
        sysctrl: &mut SYSCTRL,
        nvmctrl: &mut NVMCTRL,
        use_external_crystal: bool,
    ) -> Self {
        let mut state = State { gclk };

        set_flash_to_half_auto_wait_state(nvmctrl);
        enable_gclk_apb(pm);
        if use_external_crystal {
            enable_external_32kosc(sysctrl);
        } else {
            enable_internal_32kosc(sysctrl);
        }

        state.reset_gclk();

        // Enable a 32khz source -> GCLK1
        if use_external_crystal {
            state.set_gclk_divider_and_source(GCLK1, 1, XOSC32K, false);
        } else {
            state.set_gclk_divider_and_source(GCLK1, 1, OSC32K, false);
        }

        // Feed 32khz into the DFLL48
        state.enable_clock_generator(DFLL48, GCLK1);
        // Enable the DFLL48
        configure_and_enable_dfll48m(sysctrl, use_external_crystal);
        // Feed DFLL48 into the main clock
        state.set_gclk_divider_and_source(GCLK0, 1, DFLL48M, true);
        // We are now running at 48Mhz

        // Reset various dividers back to 1
        sysctrl.osc8m.modify(|_, w| {
            w.presc()._0();
            w.ondemand().clear_bit()
        });
        pm.cpusel.write(|w| w.cpudiv().div1());
        pm.apbasel.write(|w| w.apbadiv().div1());
        pm.apbbsel.write(|w| w.apbbdiv().div1());
        pm.apbcsel.write(|w| w.apbcdiv().div1());

        Self {
            state,
            gclks: [
                OSC48M_FREQ,
                OSC32K_FREQ,
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
            ],
            used_clocks: 1u64 << DFLL48M.bits(),
        }
    }

    /// Returns a `GClock` for gclk0, the system clock generator at 48Mhz
    pub fn gclk0(&mut self) -> GClock {
        GClock {
            gclk: GCLK0,
            freq: self.gclks[0],
        }
    }

    /// Returns a `GClock` for gclk1, the 32Khz oscillator.
    pub fn gclk1(&mut self) -> GClock {
        GClock {
            gclk: GCLK1,
            freq: self.gclks[1],
        }
    }

    /// Returns the `GClock` for the specified clock generator.
    /// If that clock generator has not yet been configured,
    /// returns None.
    pub fn get_gclk(&mut self, gclk: ClockGenId) -> Option<GClock> {
        let idx = gclk.bits() as usize;
        if self.gclks[idx].0 == 0 {
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
        let idx = gclk.bits() as usize;
        if self.gclks[idx].0 != 0 {
            return None;
        }
        self.state
            .set_gclk_divider_and_source(gclk, divider, src, improve_duty_cycle);
        let freq: Hertz = match src {
            XOSC32K | OSC32K | OSCULP32K => OSC32K_FREQ,
            GCLKGEN1 => self.gclks[1],
            OSC8M => 8.mhz().into(),
            DFLL48M => OSC48M_FREQ,
            DPLL96M => 96.mhz().into(),
            GCLKIN | XOSC | _ => unimplemented!(),
        };
        self.gclks[idx] = Hertz(freq.0 / divider as u32);
        Some(GClock { gclk, freq })
    }
}

macro_rules! clock_generator {
    ($(($id:ident, $Type:ident, $clock:ident),)+) => {

$(
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
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
        let bits : u64 = 1<<$clock.bits() as u64;
        if (self.used_clocks & bits) != 0 {
            return None;
        }
        self.used_clocks |= bits;

        self.state.enable_clock_generator($clock, generator.gclk);
        let freq = self.gclks[generator.gclk.bits() as usize];
        Some($Type{freq})
    }
    )+
}
    }
}

clock_generator!(
    (tcc2_tc3, Tcc2Tc3Clock, TCC2_TC3),
    (tc4_tc5, Tc4Tc5Clock, TC4_TC5),
    (tc6_tc7, Tc6Tc7Clock, TC6_TC7),
    (sercom0_core, Sercom0CoreClock, SERCOM0_CORE),
    (sercom1_core, Sercom1CoreClock, SERCOM1_CORE),
    (sercom2_core, Sercom2CoreClock, SERCOM2_CORE),
    (sercom3_core, Sercom3CoreClock, SERCOM3_CORE),
    (sercom4_core, Sercom4CoreClock, SERCOM4_CORE),
    (sercom5_core, Sercom5CoreClock, SERCOM5_CORE),
    (usb, UsbClock, USB),
);

/// Helper type for computing effective frequency given a source
/// clock frequency and a desired frequency.
#[derive(Debug, Clone, Copy)]
pub struct ClockParams {
    /// The frequency of the source/input clock
    pub src_freq: Hertz,
    /// The linear division value.  This is constrained to the range
    /// of values supported by the hardware.
    pub divider: u16,
    /// The effective frequency, which is ideally the desired frequency,
    /// but is produced by dividing the `src_freq` by the `divider`.
    pub effective_freq: Hertz,
}

impl ClockParams {
    /// Given a source frequency and a desired frequency, compute the
    /// `ClockParams` values for the closest matching clock configuration.
    pub fn new(src_freq: Hertz, desired_freq: Hertz) -> Self {
        let divider = (src_freq.0 / desired_freq.0.saturating_sub(1).max(1)).next_power_of_two();
        let divider = match divider {
            1 | 2 | 4 | 8 | 16 | 64 | 256 | 1024 => divider,
            // There are a couple of gaps, so we round up to the next largest
            // divider; we'll need to count twice as many but it will work.
            32 => 64,
            128 => 256,
            512 => 1024,
            // Catch all case; this is lame.  Would be great to detect this
            // and fail at compile time.
            _ => 1024,
        };
        let effective_freq = Hertz(src_freq.0 / divider);
        Self {
            src_freq,
            divider: divider as u16,
            effective_freq,
        }
    }
}

/// The frequency of the 48Mhz source.
pub const OSC48M_FREQ: Hertz = Hertz(48_000_000);
/// The frequency of the 32Khz source.
pub const OSC32K_FREQ: Hertz = Hertz(32_000);

fn set_flash_to_half_auto_wait_state(nvmctrl: &mut NVMCTRL) {
    nvmctrl.ctrlb.modify(|_, w| w.rws().half());
}

fn enable_gclk_apb(pm: &mut PM) {
    pm.apbamask.modify(|_, w| w.gclk_().set_bit());
}

/// Turn on the internal 32hkz oscillator
fn enable_internal_32kosc(sysctrl: &mut SYSCTRL) {
    let calibration = super::calibration::osc32k_cal();
    sysctrl.osc32k.write(|w| {
        unsafe {
            w.ondemand().clear_bit();
            w.calib().bits(calibration);
            // 6 here means: use 66 cycles of OSC32k to start up this oscillator
            w.startup().bits(6);
        }
        w.en32k().set_bit();
        w.enable().set_bit()
    });
    while sysctrl.pclksr.read().osc32krdy().bit_is_clear() {
        // Wait for the oscillator to stabilize
    }
}

/// Turn on the external 32hkz oscillator
fn enable_external_32kosc(sysctrl: &mut SYSCTRL) {
    sysctrl.xosc32k.modify(|_, w| {
        unsafe {
            // 6 here means: use 64k cycles of OSCULP32k to start up this oscillator
            w.startup().bits(6);
        }
        w.ondemand().clear_bit();
        // Enable 32khz output
        w.en32k().set_bit();
        // Crystal connected to xin32/xout32
        w.xtalen().set_bit()
    });
    sysctrl.xosc32k.modify(|_, w| w.enable().set_bit());
    while sysctrl.pclksr.read().xosc32krdy().bit_is_clear() {
        // Wait for the oscillator to stabilize
    }
}

fn wait_for_dfllrdy(sysctrl: &mut SYSCTRL) {
    while sysctrl.pclksr.read().dfllrdy().bit_is_clear() {}
}

/// Configure the dfll48m to operate at 48Mhz
fn configure_and_enable_dfll48m(sysctrl: &mut SYSCTRL, use_external_crystal: bool) {
    // Turn it off while we configure it.
    // Note that we need to turn off on-demand mode and
    // disable it here, rather than just reseting the ctrl
    // register, otherwise our configuration attempt fails.
    sysctrl.dfllctrl.write(|w| w.ondemand().clear_bit());
    wait_for_dfllrdy(sysctrl);

    if use_external_crystal {
        sysctrl.dfllmul.write(|w| unsafe {
            w.cstep().bits(31);
            w.fstep().bits(511);
            // scaling factor between the clocks
            w.mul().bits(((48_000_000u32 + 32768 / 2) / 32768) as u16)
        });

        // Turn it on
        sysctrl.dfllctrl.write(|w| {
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
        let coarse = super::calibration::dfll48m_coarse_cal();
        let fine = 0x1ff;

        sysctrl.dfllval.write(|w| unsafe {
            w.coarse().bits(coarse);
            w.fine().bits(fine)
        });

        sysctrl.dfllmul.write(|w| unsafe {
            w.cstep().bits(coarse / 4);
            w.fstep().bits(10);
            // scaling factor between the clocks
            w.mul().bits((48_000_000u32 / 32768) as u16)
        });

        // Turn it on
        sysctrl.dfllctrl.write(|w| {
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
    sysctrl.dfllctrl.modify(|_, w| w.enable().set_bit());

    wait_for_dfllrdy(sysctrl);
}
