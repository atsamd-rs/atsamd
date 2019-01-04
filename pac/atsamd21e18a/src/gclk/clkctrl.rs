#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CLKCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR {
    #[doc = "DFLL48"]
    DFLL48,
    #[doc = "FDPLL"]
    FDPLL,
    #[doc = "FDPLL32K"]
    FDPLL32K,
    #[doc = "WDT"]
    WDT,
    #[doc = "RTC"]
    RTC,
    #[doc = "EIC"]
    EIC,
    #[doc = "USB"]
    USB,
    #[doc = "EVSYS_0"]
    EVSYS_0,
    #[doc = "EVSYS_1"]
    EVSYS_1,
    #[doc = "EVSYS_2"]
    EVSYS_2,
    #[doc = "EVSYS_3"]
    EVSYS_3,
    #[doc = "EVSYS_4"]
    EVSYS_4,
    #[doc = "EVSYS_5"]
    EVSYS_5,
    #[doc = "EVSYS_6"]
    EVSYS_6,
    #[doc = "EVSYS_7"]
    EVSYS_7,
    #[doc = "EVSYS_8"]
    EVSYS_8,
    #[doc = "EVSYS_9"]
    EVSYS_9,
    #[doc = "EVSYS_10"]
    EVSYS_10,
    #[doc = "EVSYS_11"]
    EVSYS_11,
    #[doc = "SERCOMX_SLOW"]
    SERCOMX_SLOW,
    #[doc = "SERCOM0_CORE"]
    SERCOM0_CORE,
    #[doc = "SERCOM1_CORE"]
    SERCOM1_CORE,
    #[doc = "SERCOM2_CORE"]
    SERCOM2_CORE,
    #[doc = "SERCOM3_CORE"]
    SERCOM3_CORE,
    #[doc = "SERCOM4_CORE"]
    SERCOM4_CORE,
    #[doc = "SERCOM5_CORE"]
    SERCOM5_CORE,
    #[doc = "TCC0_TCC1"]
    TCC0_TCC1,
    #[doc = "TCC2_TC3"]
    TCC2_TC3,
    #[doc = "TC4_TC5"]
    TC4_TC5,
    #[doc = "TC6_TC7"]
    TC6_TC7,
    #[doc = "ADC"]
    ADC,
    #[doc = "AC_DIG"]
    AC_DIG,
    #[doc = "AC_ANA"]
    AC_ANA,
    #[doc = "DAC"]
    DAC,
    #[doc = "I2S_0"]
    I2S_0,
    #[doc = "I2S_1"]
    I2S_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDR::DFLL48 => 0,
            IDR::FDPLL => 1,
            IDR::FDPLL32K => 2,
            IDR::WDT => 3,
            IDR::RTC => 4,
            IDR::EIC => 5,
            IDR::USB => 6,
            IDR::EVSYS_0 => 7,
            IDR::EVSYS_1 => 8,
            IDR::EVSYS_2 => 9,
            IDR::EVSYS_3 => 10,
            IDR::EVSYS_4 => 11,
            IDR::EVSYS_5 => 12,
            IDR::EVSYS_6 => 13,
            IDR::EVSYS_7 => 14,
            IDR::EVSYS_8 => 15,
            IDR::EVSYS_9 => 16,
            IDR::EVSYS_10 => 17,
            IDR::EVSYS_11 => 18,
            IDR::SERCOMX_SLOW => 19,
            IDR::SERCOM0_CORE => 20,
            IDR::SERCOM1_CORE => 21,
            IDR::SERCOM2_CORE => 22,
            IDR::SERCOM3_CORE => 23,
            IDR::SERCOM4_CORE => 24,
            IDR::SERCOM5_CORE => 25,
            IDR::TCC0_TCC1 => 26,
            IDR::TCC2_TC3 => 27,
            IDR::TC4_TC5 => 28,
            IDR::TC6_TC7 => 29,
            IDR::ADC => 30,
            IDR::AC_DIG => 31,
            IDR::AC_ANA => 32,
            IDR::DAC => 33,
            IDR::I2S_0 => 35,
            IDR::I2S_1 => 36,
            IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDR {
        match value {
            0 => IDR::DFLL48,
            1 => IDR::FDPLL,
            2 => IDR::FDPLL32K,
            3 => IDR::WDT,
            4 => IDR::RTC,
            5 => IDR::EIC,
            6 => IDR::USB,
            7 => IDR::EVSYS_0,
            8 => IDR::EVSYS_1,
            9 => IDR::EVSYS_2,
            10 => IDR::EVSYS_3,
            11 => IDR::EVSYS_4,
            12 => IDR::EVSYS_5,
            13 => IDR::EVSYS_6,
            14 => IDR::EVSYS_7,
            15 => IDR::EVSYS_8,
            16 => IDR::EVSYS_9,
            17 => IDR::EVSYS_10,
            18 => IDR::EVSYS_11,
            19 => IDR::SERCOMX_SLOW,
            20 => IDR::SERCOM0_CORE,
            21 => IDR::SERCOM1_CORE,
            22 => IDR::SERCOM2_CORE,
            23 => IDR::SERCOM3_CORE,
            24 => IDR::SERCOM4_CORE,
            25 => IDR::SERCOM5_CORE,
            26 => IDR::TCC0_TCC1,
            27 => IDR::TCC2_TC3,
            28 => IDR::TC4_TC5,
            29 => IDR::TC6_TC7,
            30 => IDR::ADC,
            31 => IDR::AC_DIG,
            32 => IDR::AC_ANA,
            33 => IDR::DAC,
            35 => IDR::I2S_0,
            36 => IDR::I2S_1,
            i => IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DFLL48`"]
    #[inline]
    pub fn is_dfll48(&self) -> bool {
        *self == IDR::DFLL48
    }
    #[doc = "Checks if the value of the field is `FDPLL`"]
    #[inline]
    pub fn is_fdpll(&self) -> bool {
        *self == IDR::FDPLL
    }
    #[doc = "Checks if the value of the field is `FDPLL32K`"]
    #[inline]
    pub fn is_fdpll32k(&self) -> bool {
        *self == IDR::FDPLL32K
    }
    #[doc = "Checks if the value of the field is `WDT`"]
    #[inline]
    pub fn is_wdt(&self) -> bool {
        *self == IDR::WDT
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == IDR::RTC
    }
    #[doc = "Checks if the value of the field is `EIC`"]
    #[inline]
    pub fn is_eic(&self) -> bool {
        *self == IDR::EIC
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline]
    pub fn is_usb(&self) -> bool {
        *self == IDR::USB
    }
    #[doc = "Checks if the value of the field is `EVSYS_0`"]
    #[inline]
    pub fn is_evsys_0(&self) -> bool {
        *self == IDR::EVSYS_0
    }
    #[doc = "Checks if the value of the field is `EVSYS_1`"]
    #[inline]
    pub fn is_evsys_1(&self) -> bool {
        *self == IDR::EVSYS_1
    }
    #[doc = "Checks if the value of the field is `EVSYS_2`"]
    #[inline]
    pub fn is_evsys_2(&self) -> bool {
        *self == IDR::EVSYS_2
    }
    #[doc = "Checks if the value of the field is `EVSYS_3`"]
    #[inline]
    pub fn is_evsys_3(&self) -> bool {
        *self == IDR::EVSYS_3
    }
    #[doc = "Checks if the value of the field is `EVSYS_4`"]
    #[inline]
    pub fn is_evsys_4(&self) -> bool {
        *self == IDR::EVSYS_4
    }
    #[doc = "Checks if the value of the field is `EVSYS_5`"]
    #[inline]
    pub fn is_evsys_5(&self) -> bool {
        *self == IDR::EVSYS_5
    }
    #[doc = "Checks if the value of the field is `EVSYS_6`"]
    #[inline]
    pub fn is_evsys_6(&self) -> bool {
        *self == IDR::EVSYS_6
    }
    #[doc = "Checks if the value of the field is `EVSYS_7`"]
    #[inline]
    pub fn is_evsys_7(&self) -> bool {
        *self == IDR::EVSYS_7
    }
    #[doc = "Checks if the value of the field is `EVSYS_8`"]
    #[inline]
    pub fn is_evsys_8(&self) -> bool {
        *self == IDR::EVSYS_8
    }
    #[doc = "Checks if the value of the field is `EVSYS_9`"]
    #[inline]
    pub fn is_evsys_9(&self) -> bool {
        *self == IDR::EVSYS_9
    }
    #[doc = "Checks if the value of the field is `EVSYS_10`"]
    #[inline]
    pub fn is_evsys_10(&self) -> bool {
        *self == IDR::EVSYS_10
    }
    #[doc = "Checks if the value of the field is `EVSYS_11`"]
    #[inline]
    pub fn is_evsys_11(&self) -> bool {
        *self == IDR::EVSYS_11
    }
    #[doc = "Checks if the value of the field is `SERCOMX_SLOW`"]
    #[inline]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == IDR::SERCOMX_SLOW
    }
    #[doc = "Checks if the value of the field is `SERCOM0_CORE`"]
    #[inline]
    pub fn is_sercom0_core(&self) -> bool {
        *self == IDR::SERCOM0_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM1_CORE`"]
    #[inline]
    pub fn is_sercom1_core(&self) -> bool {
        *self == IDR::SERCOM1_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM2_CORE`"]
    #[inline]
    pub fn is_sercom2_core(&self) -> bool {
        *self == IDR::SERCOM2_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM3_CORE`"]
    #[inline]
    pub fn is_sercom3_core(&self) -> bool {
        *self == IDR::SERCOM3_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM4_CORE`"]
    #[inline]
    pub fn is_sercom4_core(&self) -> bool {
        *self == IDR::SERCOM4_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM5_CORE`"]
    #[inline]
    pub fn is_sercom5_core(&self) -> bool {
        *self == IDR::SERCOM5_CORE
    }
    #[doc = "Checks if the value of the field is `TCC0_TCC1`"]
    #[inline]
    pub fn is_tcc0_tcc1(&self) -> bool {
        *self == IDR::TCC0_TCC1
    }
    #[doc = "Checks if the value of the field is `TCC2_TC3`"]
    #[inline]
    pub fn is_tcc2_tc3(&self) -> bool {
        *self == IDR::TCC2_TC3
    }
    #[doc = "Checks if the value of the field is `TC4_TC5`"]
    #[inline]
    pub fn is_tc4_tc5(&self) -> bool {
        *self == IDR::TC4_TC5
    }
    #[doc = "Checks if the value of the field is `TC6_TC7`"]
    #[inline]
    pub fn is_tc6_tc7(&self) -> bool {
        *self == IDR::TC6_TC7
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline]
    pub fn is_adc(&self) -> bool {
        *self == IDR::ADC
    }
    #[doc = "Checks if the value of the field is `AC_DIG`"]
    #[inline]
    pub fn is_ac_dig(&self) -> bool {
        *self == IDR::AC_DIG
    }
    #[doc = "Checks if the value of the field is `AC_ANA`"]
    #[inline]
    pub fn is_ac_ana(&self) -> bool {
        *self == IDR::AC_ANA
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == IDR::DAC
    }
    #[doc = "Checks if the value of the field is `I2S_0`"]
    #[inline]
    pub fn is_i2s_0(&self) -> bool {
        *self == IDR::I2S_0
    }
    #[doc = "Checks if the value of the field is `I2S_1`"]
    #[inline]
    pub fn is_i2s_1(&self) -> bool {
        *self == IDR::I2S_1
    }
}
#[doc = "Possible values of the field `GEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENR {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GENR::GCLK0 => 0,
            GENR::GCLK1 => 1,
            GENR::GCLK2 => 2,
            GENR::GCLK3 => 3,
            GENR::GCLK4 => 4,
            GENR::GCLK5 => 5,
            GENR::GCLK6 => 6,
            GENR::GCLK7 => 7,
            GENR::GCLK8 => 8,
            GENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GENR {
        match value {
            0 => GENR::GCLK0,
            1 => GENR::GCLK1,
            2 => GENR::GCLK2,
            3 => GENR::GCLK3,
            4 => GENR::GCLK4,
            5 => GENR::GCLK5,
            6 => GENR::GCLK6,
            7 => GENR::GCLK7,
            8 => GENR::GCLK8,
            i => GENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENR::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENR::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENR::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENR::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENR::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENR::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline]
    pub fn is_gclk6(&self) -> bool {
        *self == GENR::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline]
    pub fn is_gclk7(&self) -> bool {
        *self == GENR::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline]
    pub fn is_gclk8(&self) -> bool {
        *self == GENR::GCLK8
    }
}
#[doc = r" Value of the field"]
pub struct CLKENR {
    bits: bool,
}
impl CLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WRTLOCKR {
    bits: bool,
}
impl WRTLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `ID`"]
pub enum IDW {
    #[doc = "DFLL48"]
    DFLL48,
    #[doc = "FDPLL"]
    FDPLL,
    #[doc = "FDPLL32K"]
    FDPLL32K,
    #[doc = "WDT"]
    WDT,
    #[doc = "RTC"]
    RTC,
    #[doc = "EIC"]
    EIC,
    #[doc = "USB"]
    USB,
    #[doc = "EVSYS_0"]
    EVSYS_0,
    #[doc = "EVSYS_1"]
    EVSYS_1,
    #[doc = "EVSYS_2"]
    EVSYS_2,
    #[doc = "EVSYS_3"]
    EVSYS_3,
    #[doc = "EVSYS_4"]
    EVSYS_4,
    #[doc = "EVSYS_5"]
    EVSYS_5,
    #[doc = "EVSYS_6"]
    EVSYS_6,
    #[doc = "EVSYS_7"]
    EVSYS_7,
    #[doc = "EVSYS_8"]
    EVSYS_8,
    #[doc = "EVSYS_9"]
    EVSYS_9,
    #[doc = "EVSYS_10"]
    EVSYS_10,
    #[doc = "EVSYS_11"]
    EVSYS_11,
    #[doc = "SERCOMX_SLOW"]
    SERCOMX_SLOW,
    #[doc = "SERCOM0_CORE"]
    SERCOM0_CORE,
    #[doc = "SERCOM1_CORE"]
    SERCOM1_CORE,
    #[doc = "SERCOM2_CORE"]
    SERCOM2_CORE,
    #[doc = "SERCOM3_CORE"]
    SERCOM3_CORE,
    #[doc = "SERCOM4_CORE"]
    SERCOM4_CORE,
    #[doc = "SERCOM5_CORE"]
    SERCOM5_CORE,
    #[doc = "TCC0_TCC1"]
    TCC0_TCC1,
    #[doc = "TCC2_TC3"]
    TCC2_TC3,
    #[doc = "TC4_TC5"]
    TC4_TC5,
    #[doc = "TC6_TC7"]
    TC6_TC7,
    #[doc = "ADC"]
    ADC,
    #[doc = "AC_DIG"]
    AC_DIG,
    #[doc = "AC_ANA"]
    AC_ANA,
    #[doc = "DAC"]
    DAC,
    #[doc = "I2S_0"]
    I2S_0,
    #[doc = "I2S_1"]
    I2S_1,
}
impl IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDW::DFLL48 => 0,
            IDW::FDPLL => 1,
            IDW::FDPLL32K => 2,
            IDW::WDT => 3,
            IDW::RTC => 4,
            IDW::EIC => 5,
            IDW::USB => 6,
            IDW::EVSYS_0 => 7,
            IDW::EVSYS_1 => 8,
            IDW::EVSYS_2 => 9,
            IDW::EVSYS_3 => 10,
            IDW::EVSYS_4 => 11,
            IDW::EVSYS_5 => 12,
            IDW::EVSYS_6 => 13,
            IDW::EVSYS_7 => 14,
            IDW::EVSYS_8 => 15,
            IDW::EVSYS_9 => 16,
            IDW::EVSYS_10 => 17,
            IDW::EVSYS_11 => 18,
            IDW::SERCOMX_SLOW => 19,
            IDW::SERCOM0_CORE => 20,
            IDW::SERCOM1_CORE => 21,
            IDW::SERCOM2_CORE => 22,
            IDW::SERCOM3_CORE => 23,
            IDW::SERCOM4_CORE => 24,
            IDW::SERCOM5_CORE => 25,
            IDW::TCC0_TCC1 => 26,
            IDW::TCC2_TC3 => 27,
            IDW::TC4_TC5 => 28,
            IDW::TC6_TC7 => 29,
            IDW::ADC => 30,
            IDW::AC_DIG => 31,
            IDW::AC_ANA => 32,
            IDW::DAC => 33,
            IDW::I2S_0 => 35,
            IDW::I2S_1 => 36,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DFLL48"]
    #[inline]
    pub fn dfll48(self) -> &'a mut W {
        self.variant(IDW::DFLL48)
    }
    #[doc = "FDPLL"]
    #[inline]
    pub fn fdpll(self) -> &'a mut W {
        self.variant(IDW::FDPLL)
    }
    #[doc = "FDPLL32K"]
    #[inline]
    pub fn fdpll32k(self) -> &'a mut W {
        self.variant(IDW::FDPLL32K)
    }
    #[doc = "WDT"]
    #[inline]
    pub fn wdt(self) -> &'a mut W {
        self.variant(IDW::WDT)
    }
    #[doc = "RTC"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(IDW::RTC)
    }
    #[doc = "EIC"]
    #[inline]
    pub fn eic(self) -> &'a mut W {
        self.variant(IDW::EIC)
    }
    #[doc = "USB"]
    #[inline]
    pub fn usb(self) -> &'a mut W {
        self.variant(IDW::USB)
    }
    #[doc = "EVSYS_0"]
    #[inline]
    pub fn evsys_0(self) -> &'a mut W {
        self.variant(IDW::EVSYS_0)
    }
    #[doc = "EVSYS_1"]
    #[inline]
    pub fn evsys_1(self) -> &'a mut W {
        self.variant(IDW::EVSYS_1)
    }
    #[doc = "EVSYS_2"]
    #[inline]
    pub fn evsys_2(self) -> &'a mut W {
        self.variant(IDW::EVSYS_2)
    }
    #[doc = "EVSYS_3"]
    #[inline]
    pub fn evsys_3(self) -> &'a mut W {
        self.variant(IDW::EVSYS_3)
    }
    #[doc = "EVSYS_4"]
    #[inline]
    pub fn evsys_4(self) -> &'a mut W {
        self.variant(IDW::EVSYS_4)
    }
    #[doc = "EVSYS_5"]
    #[inline]
    pub fn evsys_5(self) -> &'a mut W {
        self.variant(IDW::EVSYS_5)
    }
    #[doc = "EVSYS_6"]
    #[inline]
    pub fn evsys_6(self) -> &'a mut W {
        self.variant(IDW::EVSYS_6)
    }
    #[doc = "EVSYS_7"]
    #[inline]
    pub fn evsys_7(self) -> &'a mut W {
        self.variant(IDW::EVSYS_7)
    }
    #[doc = "EVSYS_8"]
    #[inline]
    pub fn evsys_8(self) -> &'a mut W {
        self.variant(IDW::EVSYS_8)
    }
    #[doc = "EVSYS_9"]
    #[inline]
    pub fn evsys_9(self) -> &'a mut W {
        self.variant(IDW::EVSYS_9)
    }
    #[doc = "EVSYS_10"]
    #[inline]
    pub fn evsys_10(self) -> &'a mut W {
        self.variant(IDW::EVSYS_10)
    }
    #[doc = "EVSYS_11"]
    #[inline]
    pub fn evsys_11(self) -> &'a mut W {
        self.variant(IDW::EVSYS_11)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline]
    pub fn sercomx_slow(self) -> &'a mut W {
        self.variant(IDW::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline]
    pub fn sercom0_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline]
    pub fn sercom1_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline]
    pub fn sercom2_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM2_CORE)
    }
    #[doc = "SERCOM3_CORE"]
    #[inline]
    pub fn sercom3_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM3_CORE)
    }
    #[doc = "SERCOM4_CORE"]
    #[inline]
    pub fn sercom4_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM4_CORE)
    }
    #[doc = "SERCOM5_CORE"]
    #[inline]
    pub fn sercom5_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM5_CORE)
    }
    #[doc = "TCC0_TCC1"]
    #[inline]
    pub fn tcc0_tcc1(self) -> &'a mut W {
        self.variant(IDW::TCC0_TCC1)
    }
    #[doc = "TCC2_TC3"]
    #[inline]
    pub fn tcc2_tc3(self) -> &'a mut W {
        self.variant(IDW::TCC2_TC3)
    }
    #[doc = "TC4_TC5"]
    #[inline]
    pub fn tc4_tc5(self) -> &'a mut W {
        self.variant(IDW::TC4_TC5)
    }
    #[doc = "TC6_TC7"]
    #[inline]
    pub fn tc6_tc7(self) -> &'a mut W {
        self.variant(IDW::TC6_TC7)
    }
    #[doc = "ADC"]
    #[inline]
    pub fn adc(self) -> &'a mut W {
        self.variant(IDW::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline]
    pub fn ac_dig(self) -> &'a mut W {
        self.variant(IDW::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline]
    pub fn ac_ana(self) -> &'a mut W {
        self.variant(IDW::AC_ANA)
    }
    #[doc = "DAC"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(IDW::DAC)
    }
    #[doc = "I2S_0"]
    #[inline]
    pub fn i2s_0(self) -> &'a mut W {
        self.variant(IDW::I2S_0)
    }
    #[doc = "I2S_1"]
    #[inline]
    pub fn i2s_1(self) -> &'a mut W {
        self.variant(IDW::I2S_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GEN`"]
pub enum GENW {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = "Generic clock generator 6"]
    GCLK6,
    #[doc = "Generic clock generator 7"]
    GCLK7,
    #[doc = "Generic clock generator 8"]
    GCLK8,
}
impl GENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GENW::GCLK0 => 0,
            GENW::GCLK1 => 1,
            GENW::GCLK2 => 2,
            GENW::GCLK3 => 3,
            GENW::GCLK4 => 4,
            GENW::GCLK5 => 5,
            GENW::GCLK6 => 6,
            GENW::GCLK7 => 7,
            GENW::GCLK8 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GENW::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GENW::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GENW::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GENW::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GENW::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GENW::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GENW::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GENW::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GENW::GCLK8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline]
    pub fn id(&self) -> IDR {
        IDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&self) -> GENR {
        GENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CLKENR { bits }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline]
    pub fn wrtlock(&self) -> WRTLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        WRTLOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&mut self) -> _GENW {
        _GENW { w: self }
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline]
    pub fn wrtlock(&mut self) -> _WRTLOCKW {
        _WRTLOCKW { w: self }
    }
}
