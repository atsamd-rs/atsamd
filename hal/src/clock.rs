// Sketching out clock configuration while I figure out
// how all the pieces fit together and how to model them.
pub use atsamd21g18a::gclk::clkctrl::GENW as GenericClockGenerator;
use atsamd21g18a::{GCLK, NVMCTRL, PM, SYSCTRL};
use time::Hertz;

#[derive(Debug, Clone, Copy)]
pub struct ClockParams {
    pub generator: ClockGenerator,
    pub divider: u16,
    pub effective_freq: Hertz,
}

#[derive(Debug, Clone, Copy)]
pub enum ClockGenerator {
    Osc48m,
    Osc32k,
}

impl Into<GenericClockGenerator> for ClockGenerator {
    fn into(self) -> GenericClockGenerator {
        match self {
            // GCLK0 is the CPU clock and is set to 48MHz
            ClockGenerator::Osc48m => GenericClockGenerator::GCLK0,
            // GCLK1 is assigned to the 32k osc by our startup
            ClockGenerator::Osc32k => GenericClockGenerator::GCLK1,
        }
    }
}

impl Into<Hertz> for ClockGenerator {
    fn into(self) -> Hertz {
        match self {
            ClockGenerator::Osc48m => OSC48M,
            ClockGenerator::Osc32k => OSC32K,
        }
    }
}

impl From<Hertz> for ClockGenerator {
    fn from(freq: Hertz) -> ClockGenerator {
        if freq.0 <= OSC32K.0 {
            ClockGenerator::Osc32k
        } else {
            ClockGenerator::Osc48m
        }
    }
}

pub const OSC48M: Hertz = Hertz(48_000_000);
pub const OSC32K: Hertz = Hertz(32_000);

macro_rules! clock_config {
    ($(($id:ident, $Type:ident),)+) => {

$(
#[derive(Debug, Clone)]
pub struct $Type {
    gen: ClockGenerator,
}

impl $Type {
    pub fn generator(&self) -> ClockGenerator {
        self.gen
    }
}
)+


/// Frozen clock configuration record
#[derive(Debug, Clone)]
pub struct Clocks {
    $($id: Option<$Type>,)+
}

#[derive(Debug, Default)]
pub struct ClockConfiguration {
    $($id: Option<ClockGenerator>,)+
}
impl ClockConfiguration {
    $(
    pub fn $id<F: Into<Hertz>>(mut self, freq: F) -> Self {
        self.$id = Some(freq.into().into());
        self
    }
    )+

    /// Freeze the configuration builder and apply it to
    /// the appropriate peripherals.
    pub fn freeze(
        self,
        mut gclk: GCLK,
        pm: &mut PM,
        sysctrl: &mut SYSCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Clocks {
        set_system_clock_to_48mhz(&mut gclk, pm, sysctrl, nvmctrl);

        $(
        if let Some(gen) = self.$id {
            gclk.clkctrl.write(|w| {
                w.id().$id();
                w.gen().variant(gen.into());
                w.clken().set_bit()
            });
            wait_for_gclk_sync(&mut gclk);
        }
        )+

        Clocks {
            $(
            $id: self.$id.map(|gen| $Type{gen}),
            )+
        }
    }
}

impl Clocks {
    $(
    pub fn $id(&self) -> Option<&$Type> {
        self.$id.as_ref()
    }
    )+
}

    }
}

clock_config!(
    (tcc2_tc3, Tcc2Tc3Clock),
    (tc4_tc5, Tc4Tc5Clock),
    (sercom0_core, Sercom0CoreClock),
    (sercom1_core, Sercom1CoreClock),
    (sercom2_core, Sercom2CoreClock),
    (sercom3_core, Sercom3CoreClock),
    (sercom4_core, Sercom4CoreClock),
    (sercom5_core, Sercom5CoreClock),
);

impl Clocks {
    pub fn sysclock(&self) -> Hertz {
        OSC48M
    }

    /// Compute best matching clock for a given frequency
    pub fn clock_params(&self, desired_freq: Hertz) -> ClockParams {
        let generator = desired_freq.into();
        self.constrained_clock_params(generator, desired_freq)
    }

    pub fn constrained_clock_params(
        &self,
        generator: ClockGenerator,
        desired_freq: Hertz,
    ) -> ClockParams {
        let freq: Hertz = generator.into();
        let divider = (freq.0 / desired_freq.0.saturating_sub(1).max(1)).next_power_of_two();
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
        let effective_freq = Hertz(freq.0 / divider);
        ClockParams {
            generator,
            divider: divider as u16,
            effective_freq,
        }
    }
}

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

pub(crate) fn wait_for_gclk_sync(gclk: &mut GCLK) {
    while gclk.status.read().syncbusy().bit_is_set() {}
}

/// Reset all clocks to a known state and wait for the clock
/// state to synchronize
fn reset_gclk(gclk: &mut GCLK) {
    gclk.ctrl.write(|w| w.swrst().set_bit());
    while gclk.ctrl.read().swrst().bit_is_set() || gclk.status.read().syncbusy().bit_is_set() {}
}

/// We're going to assign clock generator 1 to be backed by the osc32k.
fn assign_32k_src_to_clock_generator_1(gclk: &mut GCLK) {
    gclk.gendiv.write(|w| unsafe { w.id().bits(1) });
    wait_for_gclk_sync(gclk);
    gclk.genctrl.write(|w| {
        unsafe {
            w.id().bits(1);
        }
        w.src().osc32k();
        w.genen().set_bit()
    });
    wait_for_gclk_sync(gclk);
}

/// Use the 32k osc that we assigned to clock generator 1 as
/// the reference for the 48mhz clock machinery.
fn assign_clock_generator_1_as_dfll48_reference(gclk: &mut GCLK) {
    gclk.clkctrl.write(|w| {
        w.id().dfll48();
        w.gen().gclk1();
        w.clken().set_bit()
    });
    wait_for_gclk_sync(gclk);
}

fn wait_for_dfllrdy(sysctrl: &mut SYSCTRL) {
    while sysctrl.pclksr.read().dfllrdy().bit_is_clear() {}
}

/// Configure the dfll48m to operate at 48Mhz
fn configure_and_enable_dfll48m(sysctrl: &mut SYSCTRL) {
    // Turn it off while we configure it.
    // Note that we need to turn off on-demand mode and
    // disable it here, rather than just reseting the ctrl
    // register, otherwise our configuration attempt fails.
    sysctrl.dfllctrl.write(|w| w.ondemand().clear_bit());
    wait_for_dfllrdy(sysctrl);

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

    wait_for_dfllrdy(sysctrl);

    // and finally enable it!
    sysctrl.dfllctrl.modify(|_, w| w.enable().set_bit());

    wait_for_dfllrdy(sysctrl);
}

const GCLK_MAIN: u8 = 0;

/// Now that the 48MHz clock is up and running, point clock generator 0 at it;
/// that will cause the CPU to run at 48MHz.
fn assign_dfll48m_as_gclk_main(gclk: &mut GCLK) {
    // Select GCLK_MAIN with no divider
    gclk.gendiv.write(|w| unsafe {
        w.id().bits(GCLK_MAIN);
        w.div().bits(0)
    });
    wait_for_gclk_sync(gclk);

    gclk.genctrl.write(|w| {
        // gclk_main
        unsafe {
            w.id().bits(GCLK_MAIN);
        }
        w.src().dfll48m();
        // Improve Duty Cycle to 50/50
        w.idc().set_bit();
        w.genen().set_bit()
    });
    wait_for_gclk_sync(gclk);
}

/// Configure oscillators and switch the main clock source
/// to be a 48Mhz clock.
fn set_system_clock_to_48mhz(
    gclk: &mut GCLK,
    pm: &mut PM,
    sysctrl: &mut SYSCTRL,
    nvmctrl: &mut NVMCTRL,
) {
    set_flash_to_half_auto_wait_state(nvmctrl);
    enable_gclk_apb(pm);
    enable_internal_32kosc(sysctrl);
    reset_gclk(gclk);
    assign_32k_src_to_clock_generator_1(gclk);
    assign_clock_generator_1_as_dfll48_reference(gclk);
    configure_and_enable_dfll48m(sysctrl);
    assign_dfll48m_as_gclk_main(gclk);

    pm.cpusel.write(|w| w.cpudiv().div1());
    pm.apbasel.write(|w| w.apbadiv().div1());
    pm.apbbsel.write(|w| w.apbbdiv().div1());
    pm.apbcsel.write(|w| w.apbcdiv().div1());

    sysctrl.osc8m.modify(|_, w| {
        w.presc()._0();
        w.ondemand().clear_bit()
    });
}
