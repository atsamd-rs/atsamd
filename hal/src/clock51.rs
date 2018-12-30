//! Configuring the system clock sources.
//! You will typically need to create an instance of `GenericClockController`
//! before you can set up most of the peripherals on the atsamd21 device.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
use target_device::gclk::pchctrl::GENR::*;
use target_device::{self, GCLK, NVMCTRL, OSCCTRL, MCLK, OSC32KCTRL};
use target_device::gclk::genctrl::SRCR::{XOSC32K, DFLL, OSCULP32K, DPLL0};
use time::{Hertz, U32Ext};
use core::ptr;

pub type ClockGenId = target_device::gclk::pchctrl::GENR;
pub type ClockSource = target_device::gclk::genctrl::SRCR;
pub enum ClockId {
    DFLL48 = 0,
    FDPLL0,
    FDPLL1,
    SLOW_32K,
    EIC,
    FREQM_MSR,
    FREQM_REF,
    SERCOM0_CORE,
    SERCOM1_CORE,
    TC0,
    TC1,
    USB,
    EVSYS0,
    EVSYS1,
    EVSYS2,
    EVSYS3,
    EVSYS4,
    EVSYS5,
    EVSYS6,
    EVSYS7,
    EVSYS8,
    EVSYS9,
    EVSYS10,
    EVSYS11,
    SERCOM2_CORE,
    SERCOM3_CORE,
    TCC0_TCC1,
    TC2_TC3,
    CAN0,
    CAN1,
    TCC2_TCC3,
    TC4_TC5,
    PDEC,
    AC,
    CCL,
    SERCOM4_CORE,
    SERCOM5_CORE,
    SERCOM6_CORE,
    SERCOM7_CORE,
    TCC4,
    TC6_TC7,
    ADC0,
    ADC1,
    DAC,
    I2S0,
    I2S1,
    SDHC0,
    SDHC1,
    CM4_TRACE,
}
impl ClockId {
    fn bits(self) -> usize {
        self as usize
    }
}
use self::ClockId::*;

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
        self.gclk.ctrla.write(|w| w.swrst().set_bit());
        while self.gclk.ctrla.read().swrst().bit_is_set()
            || self.gclk.syncbusy.read().bits() != 0
        {}
    }

    fn wait_for_sync(&mut self) {
        while self.gclk.syncbusy.read().bits() != 0 {}
    }

    fn set_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) {
        self.gclk.genctrl[gclk.bits() as usize].write(|w| unsafe {
            w.src().bits(src.bits());
            w.div().bits(divider);
            // divide directly by divider, rather than exponential
            w.divsel().clear_bit();
            //w.idc().bit(improve_duty_cycle);
            w.genen().set_bit();
            w.oe().set_bit()
        });
        self.wait_for_sync();
    }

    fn enable_clock_generator(&mut self, clock: ClockId, generator: ClockGenId) {
        self.gclk.pchctrl[clock.bits()].write(|w| unsafe {
            w.gen().bits(generator.bits());
            w.chen().set_bit()
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
    gclks: [Hertz; 12],
    used_clocks: u64,
}

impl GenericClockController {
    /// Reset the clock controller, configure the system to run
    /// at 48Mhz and reset various clock dividers.
    pub fn with_internal_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, mclk,  osc32kctrl, oscctrl, nvmctrl, false)
    }

    /// Reset the clock controller, configure the system to run
    /// at 48Mhz and reset various clock dividers.
    pub fn with_external_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, mclk, osc32kctrl, oscctrl, nvmctrl, true)
    }

    fn new(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
        use_external_crystal: bool,
    ) -> Self {
        let mut state = State { gclk };
        nvmctrl.ctrla.modify(|_, w| w.rws().single());
        enable_internal_32kosc(osc32kctrl);
        if use_external_crystal {
            enable_external_32kosc(osc32kctrl);
            state.reset_gclk();
            state.gclk.genctrl[XOSC32K.bits() as usize].write(|w| {
                w.src().xosc32k();
                w.genen().set_bit()
            });
        } else {
            enable_internal_32kosc(osc32kctrl);
            state.reset_gclk();
            state.gclk.genctrl[OSCULP32K.bits() as usize].write(|w| {
                w.src().osculp32k();
                w.genen().set_bit()
            });
        }
        while state.gclk.syncbusy.read().genctrl3().is_gclk3() {}

        state.gclk.genctrl[0].write(|w| {
            w.src().osculp32k();
            w.genen().set_bit()
        });
        while state.gclk.syncbusy.read().genctrl0().is_gclk0() {}

        unsafe {
            oscctrl.dfllctrla.write(|w| w.bits(0));
            oscctrl.dfllmul.write(|w| {
                w.cstep().bits(1);
                w.fstep().bits(1);
                w.mul().bits(0)
            });
        }
        while oscctrl.dfllsync.read().dfllmul().bit_is_set() {}

        /*unsafe {
            oscctrl.dfllctrlb.write(|w| w.bits(0));
        }*/

        while oscctrl.dfllsync.read().dfllctrlb().bit_is_set() {}
        
        oscctrl.dfllctrla.modify(|_, w| w.enable().set_bit());
        while oscctrl.dfllsync.read().enable().bit_is_set() {}

        oscctrl.dfllctrlb.write(|w| {
            w.ccdis().set_bit();
            w.usbcrm().set_bit();
            w.waitlock().set_bit()
        });

        while oscctrl.status.read().dfllrdy().bit_is_clear() {}

        unsafe {
            state.gclk.genctrl[5].write(|w| {
                w.src().dfll();
                w.genen().set_bit();
                w.div().bits(24)
            });
        }

        while state.gclk.syncbusy.read().genctrl5().is_gclk5() {}

        configure_and_enable_dpll0(oscctrl, &mut state.gclk);
        wait_for_dpllrdy(oscctrl);

        state.gclk.genctrl[0].write(|w| {
            w.src().dpll0();
            w.idc().set_bit();
            w.genen().set_bit()
        });

        while state.gclk.syncbusy.read().genctrl0().is_gclk0() {}

        mclk.cpudiv.write(|w| {
            w.div().div1()
        });

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
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
            ],
            used_clocks: 1u64 << DFLL48.bits(),
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
            XOSC32K | OSCULP32K => OSC32K_FREQ,
            GCLKGEN1 => self.gclks[1],
            DFLL => OSC48M_FREQ,
            DPLL0 => 120.mhz().into(),
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
    (tc2_tc3, Tc2Tc3Clock, TC2_TC3),
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
    nvmctrl.ctrla.modify(|_, w| w.rws().half());
}

fn enable_gclk_apb(mclk: &mut MCLK) {
    mclk.apbamask.modify(|_, w| w.gclk_().set_bit());
}

/// Turn on the internal 32hkz oscillator
fn enable_internal_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.osculp32k.modify(|_, w| {
        w.en32k().set_bit()
    });
}

/// Turn on the external 32hkz oscillator
fn enable_external_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.xosc32k.modify(|_, w| {
        w.ondemand().clear_bit();
        // Enable 32khz output
        w.en32k().set_bit();
        // Crystal connected to xin32/xout32
        w.xtalen().set_bit();
        w.enable().set_bit();
        w.cgm().xt()
    });

    // Wait for the oscillator to stabilize
    while osc32kctrl.status.read().xosc32krdy().bit_is_clear() {}
}

fn wait_for_dpllrdy(oscctrl: &mut OSCCTRL) {
    unsafe { ptr::write_volatile(0x41008008 as _, 1 << 16); }
    while oscctrl.dpllstatus0.read().lock().bit_is_clear() ||
        oscctrl.dpllstatus0.read().clkrdy().bit_is_clear() {
            unsafe { ptr::write_volatile(0x41008018 as *mut u32,  (oscctrl.dpllstatus0.read().clkrdy().bit_is_clear() as u32) << 16); }
    }
    unsafe { ptr::write_volatile(0x41008014 as _, 1 << 16); }
}

/// Configure the dpll0 to run at 120MHz
fn configure_and_enable_dpll0(oscctrl: &mut OSCCTRL, gclk: &mut GCLK) {
   gclk.pchctrl[FDPLL0 as usize].write(|w| {
        w.chen().set_bit();
        w.gen().gclk5()
    });
    unsafe {
        oscctrl.dpllratio0.write(|w| {
            w.ldr().bits(59);
            w.ldrfrac().bits(0)
        });
    }
    oscctrl.dpllctrlb0.write(|w| {
        w.refclk().gclk();
        w.lbypass().set_bit()
    });
    oscctrl.dpllctrla0.write(|w| {
        w.enable().set_bit();
        w.ondemand().clear_bit()
    });

}
