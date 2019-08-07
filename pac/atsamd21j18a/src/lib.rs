#![doc = "Peripheral access API for ATSAMD21J18A microcontrollers (generated using svd2rust v0.16.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn PM();
    fn SYSCTRL();
    fn WDT();
    fn RTC();
    fn EIC();
    fn NVMCTRL();
    fn DMAC();
    fn USB();
    fn EVSYS();
    fn SERCOM0();
    fn SERCOM1();
    fn SERCOM2();
    fn SERCOM3();
    fn SERCOM4();
    fn SERCOM5();
    fn TCC0();
    fn TCC1();
    fn TCC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn TC6();
    fn TC7();
    fn ADC();
    fn AC();
    fn DAC();
    fn I2S();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 28] = [
    Vector { _handler: PM },
    Vector { _handler: SYSCTRL },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: EIC },
    Vector { _handler: NVMCTRL },
    Vector { _handler: DMAC },
    Vector { _handler: USB },
    Vector { _handler: EVSYS },
    Vector { _handler: SERCOM0 },
    Vector { _handler: SERCOM1 },
    Vector { _handler: SERCOM2 },
    Vector { _handler: SERCOM3 },
    Vector { _handler: SERCOM4 },
    Vector { _handler: SERCOM5 },
    Vector { _handler: TCC0 },
    Vector { _handler: TCC1 },
    Vector { _handler: TCC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: TC6 },
    Vector { _handler: TC7 },
    Vector { _handler: ADC },
    Vector { _handler: AC },
    Vector { _handler: DAC },
    Vector { _reserved: 0 },
    Vector { _handler: I2S },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - PM"]
    PM,
    #[doc = "1 - SYSCTRL"]
    SYSCTRL,
    #[doc = "2 - WDT"]
    WDT,
    #[doc = "3 - RTC"]
    RTC,
    #[doc = "4 - EIC"]
    EIC,
    #[doc = "5 - NVMCTRL"]
    NVMCTRL,
    #[doc = "6 - DMAC"]
    DMAC,
    #[doc = "7 - USB"]
    USB,
    #[doc = "8 - EVSYS"]
    EVSYS,
    #[doc = "9 - SERCOM0"]
    SERCOM0,
    #[doc = "10 - SERCOM1"]
    SERCOM1,
    #[doc = "11 - SERCOM2"]
    SERCOM2,
    #[doc = "12 - SERCOM3"]
    SERCOM3,
    #[doc = "13 - SERCOM4"]
    SERCOM4,
    #[doc = "14 - SERCOM5"]
    SERCOM5,
    #[doc = "15 - TCC0"]
    TCC0,
    #[doc = "16 - TCC1"]
    TCC1,
    #[doc = "17 - TCC2"]
    TCC2,
    #[doc = "18 - TC3"]
    TC3,
    #[doc = "19 - TC4"]
    TC4,
    #[doc = "20 - TC5"]
    TC5,
    #[doc = "21 - TC6"]
    TC6,
    #[doc = "22 - TC7"]
    TC7,
    #[doc = "23 - ADC"]
    ADC,
    #[doc = "24 - AC"]
    AC,
    #[doc = "25 - DAC"]
    DAC,
    #[doc = "27 - I2S"]
    I2S,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PM => 0,
            Interrupt::SYSCTRL => 1,
            Interrupt::WDT => 2,
            Interrupt::RTC => 3,
            Interrupt::EIC => 4,
            Interrupt::NVMCTRL => 5,
            Interrupt::DMAC => 6,
            Interrupt::USB => 7,
            Interrupt::EVSYS => 8,
            Interrupt::SERCOM0 => 9,
            Interrupt::SERCOM1 => 10,
            Interrupt::SERCOM2 => 11,
            Interrupt::SERCOM3 => 12,
            Interrupt::SERCOM4 => 13,
            Interrupt::SERCOM5 => 14,
            Interrupt::TCC0 => 15,
            Interrupt::TCC1 => 16,
            Interrupt::TCC2 => 17,
            Interrupt::TC3 => 18,
            Interrupt::TC4 => 19,
            Interrupt::TC5 => 20,
            Interrupt::TC6 => 21,
            Interrupt::TC7 => 22,
            Interrupt::ADC => 23,
            Interrupt::AC => 24,
            Interrupt::DAC => 25,
            Interrupt::I2S => 27,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Comparators"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0x4200_4400 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Analog Comparators"]
pub mod ac;
#[doc = "Analog Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4200_4000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog Digital Converter"]
pub mod adc;
#[doc = "Digital Analog Converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4200_4800 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital Analog Converter"]
pub mod dac;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x4100_4800 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsu::RegisterBlock {
        0x4100_2000 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eic::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        0x4200_0400 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gclk::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmatrix::RegisterBlock {
        0x4100_7000 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Inter-IC Sound Interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x4200_5000 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "Inter-IC Sound Interface"]
pub mod i2s;
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        0x4100_6000 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub mod mtb;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        0x4100_4000 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Peripheral Access Controller 0"]
pub struct PAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC0 {}
impl PAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PAC0 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC0::ptr() }
    }
}
#[doc = "Peripheral Access Controller 0"]
pub mod pac0;
#[doc = "Peripheral Access Controller 1"]
pub struct PAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC1 {}
impl PAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        0x4100_0000 as *const _
    }
}
impl Deref for PAC1 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC1::ptr() }
    }
}
#[doc = "Peripheral Access Controller 2"]
pub struct PAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC2 {}
impl PAC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pac0::RegisterBlock {
        0x4200_0000 as *const _
    }
}
impl Deref for PAC2 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PAC2::ptr() }
    }
}
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pm::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x4100_4400 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Port Module (IOBUS)"]
pub struct PORT_IOBUS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS {}
impl PORT_IOBUS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x6000_0000 as *const _
    }
}
impl Deref for PORT_IOBUS {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_IOBUS::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Serial Communication Interface 0"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_0800 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface 0"]
pub mod sercom0;
#[doc = "Serial Communication Interface 1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_0c00 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "Serial Communication Interface 2"]
pub struct SERCOM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM2 {}
impl SERCOM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1000 as *const _
    }
}
impl Deref for SERCOM2 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM2::ptr() }
    }
}
#[doc = "Serial Communication Interface 3"]
pub struct SERCOM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM3 {}
impl SERCOM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1400 as *const _
    }
}
impl Deref for SERCOM3 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM3::ptr() }
    }
}
#[doc = "Serial Communication Interface 4"]
pub struct SERCOM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM4 {}
impl SERCOM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1800 as *const _
    }
}
impl Deref for SERCOM4 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM4::ptr() }
    }
}
#[doc = "Serial Communication Interface 5"]
pub struct SERCOM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM5 {}
impl SERCOM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sercom0::RegisterBlock {
        0x4200_1c00 as *const _
    }
}
impl Deref for SERCOM5 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SERCOM5::ptr() }
    }
}
#[doc = "System Control"]
pub struct SYSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTRL {}
impl SYSCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctrl::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTRL::ptr() }
    }
}
#[doc = "System Control"]
pub mod sysctrl;
#[doc = "Basic Timer Counter 3"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4200_2c00 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "Basic Timer Counter 3"]
pub mod tc3;
#[doc = "Basic Timer Counter 4"]
pub struct TC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC4 {}
impl TC4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4200_3000 as *const _
    }
}
impl Deref for TC4 {
    type Target = tc3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC4::ptr() }
    }
}
#[doc = "Basic Timer Counter 5"]
pub struct TC5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC5 {}
impl TC5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4200_3400 as *const _
    }
}
impl Deref for TC5 {
    type Target = tc3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC5::ptr() }
    }
}
#[doc = "Basic Timer Counter 6"]
pub struct TC6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC6 {}
impl TC6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4200_3800 as *const _
    }
}
impl Deref for TC6 {
    type Target = tc3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC6::ptr() }
    }
}
#[doc = "Basic Timer Counter 7"]
pub struct TC7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC7 {}
impl TC7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4200_3c00 as *const _
    }
}
impl Deref for TC7 {
    type Target = tc3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC7::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub struct TCC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC0 {}
impl TCC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2000 as *const _
    }
}
impl Deref for TCC0 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC0::ptr() }
    }
}
#[doc = "Timer Counter Control 0"]
pub mod tcc0;
#[doc = "Timer Counter Control 1"]
pub struct TCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC1 {}
impl TCC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2400 as *const _
    }
}
impl Deref for TCC1 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC1::ptr() }
    }
}
#[doc = "Timer Counter Control 2"]
pub struct TCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCC2 {}
impl TCC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcc0::RegisterBlock {
        0x4200_2800 as *const _
    }
}
impl Deref for TCC2 {
    type Target = tcc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TCC2::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4100_5000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal Serial Bus"]
pub mod usb;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "PAC0"]
    pub PAC0: PAC0,
    #[doc = "PAC1"]
    pub PAC1: PAC1,
    #[doc = "PAC2"]
    pub PAC2: PAC2,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "PORT_IOBUS"]
    pub PORT_IOBUS: PORT_IOBUS,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SERCOM2"]
    pub SERCOM2: SERCOM2,
    #[doc = "SERCOM3"]
    pub SERCOM3: SERCOM3,
    #[doc = "SERCOM4"]
    pub SERCOM4: SERCOM4,
    #[doc = "SERCOM5"]
    pub SERCOM5: SERCOM5,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: SYSCTRL,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TC4"]
    pub TC4: TC4,
    #[doc = "TC5"]
    pub TC5: TC5,
    #[doc = "TC6"]
    pub TC6: TC6,
    #[doc = "TC7"]
    pub TC7: TC7,
    #[doc = "TCC0"]
    pub TCC0: TCC0,
    #[doc = "TCC1"]
    pub TCC1: TCC1,
    #[doc = "TCC2"]
    pub TCC2: TCC2,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC: AC {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            PAC0: PAC0 {
                _marker: PhantomData,
            },
            PAC1: PAC1 {
                _marker: PhantomData,
            },
            PAC2: PAC2 {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            PORT_IOBUS: PORT_IOBUS {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SERCOM2: SERCOM2 {
                _marker: PhantomData,
            },
            SERCOM3: SERCOM3 {
                _marker: PhantomData,
            },
            SERCOM4: SERCOM4 {
                _marker: PhantomData,
            },
            SERCOM5: SERCOM5 {
                _marker: PhantomData,
            },
            SYSCTRL: SYSCTRL {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TC4: TC4 {
                _marker: PhantomData,
            },
            TC5: TC5 {
                _marker: PhantomData,
            },
            TC6: TC6 {
                _marker: PhantomData,
            },
            TC7: TC7 {
                _marker: PhantomData,
            },
            TCC0: TCC0 {
                _marker: PhantomData,
            },
            TCC1: TCC1 {
                _marker: PhantomData,
            },
            TCC2: TCC2 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
