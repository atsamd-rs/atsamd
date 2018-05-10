use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak PM\nPM = DH_TRAMPOLINE\n.weak SYSCTRL\nSYSCTRL = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak EIC\nEIC = DH_TRAMPOLINE\n.weak NVMCTRL\nNVMCTRL = DH_TRAMPOLINE\n.weak DMAC\nDMAC = DH_TRAMPOLINE\n.weak USB\nUSB = DH_TRAMPOLINE\n.weak EVSYS\nEVSYS = DH_TRAMPOLINE\n.weak SERCOM0\nSERCOM0 = DH_TRAMPOLINE\n.weak SERCOM1\nSERCOM1 = DH_TRAMPOLINE\n.weak SERCOM2\nSERCOM2 = DH_TRAMPOLINE\n.weak SERCOM3\nSERCOM3 = DH_TRAMPOLINE\n.weak TCC0\nTCC0 = DH_TRAMPOLINE\n.weak TCC1\nTCC1 = DH_TRAMPOLINE\n.weak TCC2\nTCC2 = DH_TRAMPOLINE\n.weak TC3\nTC3 = DH_TRAMPOLINE\n.weak TC4\nTC4 = DH_TRAMPOLINE\n.weak TC5\nTC5 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak AC\nAC = DH_TRAMPOLINE\n.weak DAC\nDAC = DH_TRAMPOLINE\n.weak I2S\nI2S = DH_TRAMPOLINE" ) ;
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
    fn TCC0();
    fn TCC1();
    fn TCC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn ADC();
    fn AC();
    fn DAC();
    fn I2S();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 28] = [
    Some(PM),
    Some(SYSCTRL),
    Some(WDT),
    Some(RTC),
    Some(EIC),
    Some(NVMCTRL),
    Some(DMAC),
    Some(USB),
    Some(EVSYS),
    Some(SERCOM0),
    Some(SERCOM1),
    Some(SERCOM2),
    Some(SERCOM3),
    None,
    None,
    Some(TCC0),
    Some(TCC1),
    Some(TCC2),
    Some(TC3),
    Some(TC4),
    Some(TC5),
    None,
    None,
    Some(ADC),
    Some(AC),
    Some(DAC),
    None,
    Some(I2S),
];
#[doc = r" Enumeration of all the interrupts"]
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
    #[doc = "23 - ADC"]
    ADC,
    #[doc = "24 - AC"]
    AC,
    #[doc = "25 - DAC"]
    DAC,
    #[doc = "27 - I2S"]
    I2S,
}
unsafe impl Nr for Interrupt {
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
            Interrupt::TCC0 => 15,
            Interrupt::TCC1 => 16,
            Interrupt::TCC2 => 17,
            Interrupt::TC3 => 18,
            Interrupt::TC4 => 19,
            Interrupt::TC5 => 20,
            Interrupt::ADC => 23,
            Interrupt::AC => 24,
            Interrupt::DAC => 25,
            Interrupt::I2S => 27,
        }
    }
}
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::PM),
            1 => Ok(Interrupt::SYSCTRL),
            2 => Ok(Interrupt::WDT),
            3 => Ok(Interrupt::RTC),
            4 => Ok(Interrupt::EIC),
            5 => Ok(Interrupt::NVMCTRL),
            6 => Ok(Interrupt::DMAC),
            7 => Ok(Interrupt::USB),
            8 => Ok(Interrupt::EVSYS),
            9 => Ok(Interrupt::SERCOM0),
            10 => Ok(Interrupt::SERCOM1),
            11 => Ok(Interrupt::SERCOM2),
            12 => Ok(Interrupt::SERCOM3),
            15 => Ok(Interrupt::TCC0),
            16 => Ok(Interrupt::TCC1),
            17 => Ok(Interrupt::TCC2),
            18 => Ok(Interrupt::TC3),
            19 => Ok(Interrupt::TC4),
            20 => Ok(Interrupt::TC5),
            23 => Ok(Interrupt::ADC),
            24 => Ok(Interrupt::AC),
            25 => Ok(Interrupt::DAC),
            27 => Ok(Interrupt::I2S),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
