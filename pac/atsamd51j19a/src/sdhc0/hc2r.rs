#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::HC2R {
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
#[doc = "Possible values of the field `UHSMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UHSMSR {
    #[doc = "SDR12"]
    SDR12,
    #[doc = "SDR25"]
    SDR25,
    #[doc = "SDR50"]
    SDR50,
    #[doc = "SDR104"]
    SDR104,
    #[doc = "DDR50"]
    DDR50,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UHSMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UHSMSR::SDR12 => 0,
            UHSMSR::SDR25 => 1,
            UHSMSR::SDR50 => 2,
            UHSMSR::SDR104 => 3,
            UHSMSR::DDR50 => 4,
            UHSMSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UHSMSR {
        match value {
            0 => UHSMSR::SDR12,
            1 => UHSMSR::SDR25,
            2 => UHSMSR::SDR50,
            3 => UHSMSR::SDR104,
            4 => UHSMSR::DDR50,
            i => UHSMSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMSR::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMSR::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMSR::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMSR::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMSR::DDR50
    }
}
#[doc = "Possible values of the field `VS18EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS18ENR {
    #[doc = "3.3V Signaling"]
    S33V,
    #[doc = "1.8V Signaling"]
    S18V,
}
impl VS18ENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VS18ENR::S33V => false,
            VS18ENR::S18V => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VS18ENR {
        match value {
            false => VS18ENR::S33V,
            true => VS18ENR::S18V,
        }
    }
    #[doc = "Checks if the value of the field is `S33V`"]
    #[inline]
    pub fn is_s33v(&self) -> bool {
        *self == VS18ENR::S33V
    }
    #[doc = "Checks if the value of the field is `S18V`"]
    #[inline]
    pub fn is_s18v(&self) -> bool {
        *self == VS18ENR::S18V
    }
}
#[doc = "Possible values of the field `DRVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSELR {
    #[doc = "Driver Type B is Selected (Default)"]
    B,
    #[doc = "Driver Type A is Selected"]
    A,
    #[doc = "Driver Type C is Selected"]
    C,
    #[doc = "Driver Type D is Selected"]
    D,
}
impl DRVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRVSELR::B => 0,
            DRVSELR::A => 1,
            DRVSELR::C => 2,
            DRVSELR::D => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRVSELR {
        match value {
            0 => DRVSELR::B,
            1 => DRVSELR::A,
            2 => DRVSELR::C,
            3 => DRVSELR::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline]
    pub fn is_b(&self) -> bool {
        *self == DRVSELR::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline]
    pub fn is_a(&self) -> bool {
        *self == DRVSELR::A
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline]
    pub fn is_c(&self) -> bool {
        *self == DRVSELR::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline]
    pub fn is_d(&self) -> bool {
        *self == DRVSELR::D
    }
}
#[doc = "Possible values of the field `EXTUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTUNR {
    #[doc = "Not Tuned or Tuning Completed"]
    NO,
    #[doc = "Execute Tuning"]
    REQUESTED,
}
impl EXTUNR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EXTUNR::NO => false,
            EXTUNR::REQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTUNR {
        match value {
            false => EXTUNR::NO,
            true => EXTUNR::REQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == EXTUNR::NO
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline]
    pub fn is_requested(&self) -> bool {
        *self == EXTUNR::REQUESTED
    }
}
#[doc = "Possible values of the field `SLCKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLCKSELR {
    #[doc = "Fixed clock is used to sample data"]
    FIXED,
    #[doc = "Tuned clock is used to sample data"]
    TUNED,
}
impl SLCKSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SLCKSELR::FIXED => false,
            SLCKSELR::TUNED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLCKSELR {
        match value {
            false => SLCKSELR::FIXED,
            true => SLCKSELR::TUNED,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == SLCKSELR::FIXED
    }
    #[doc = "Checks if the value of the field is `TUNED`"]
    #[inline]
    pub fn is_tuned(&self) -> bool {
        *self == SLCKSELR::TUNED
    }
}
#[doc = "Possible values of the field `ASINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASINTENR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ASINTENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ASINTENR::DISABLED => false,
            ASINTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASINTENR {
        match value {
            false => ASINTENR::DISABLED,
            true => ASINTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ASINTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ASINTENR::ENABLED
    }
}
#[doc = "Possible values of the field `PVALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVALENR {
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    HOST,
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    AUTO,
}
impl PVALENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PVALENR::HOST => false,
            PVALENR::AUTO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PVALENR {
        match value {
            false => PVALENR::HOST,
            true => PVALENR::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == PVALENR::HOST
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == PVALENR::AUTO
    }
}
#[doc = "Values that can be written to the field `UHSMS`"]
pub enum UHSMSW {
    #[doc = "SDR12"]
    SDR12,
    #[doc = "SDR25"]
    SDR25,
    #[doc = "SDR50"]
    SDR50,
    #[doc = "SDR104"]
    SDR104,
    #[doc = "DDR50"]
    DDR50,
}
impl UHSMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UHSMSW::SDR12 => 0,
            UHSMSW::SDR25 => 1,
            UHSMSW::SDR50 => 2,
            UHSMSW::SDR104 => 3,
            UHSMSW::DDR50 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UHSMSW<'a> {
    w: &'a mut W,
}
impl<'a> _UHSMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UHSMSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDR12"]
    #[inline]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(UHSMSW::SDR12)
    }
    #[doc = "SDR25"]
    #[inline]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(UHSMSW::SDR25)
    }
    #[doc = "SDR50"]
    #[inline]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(UHSMSW::SDR50)
    }
    #[doc = "SDR104"]
    #[inline]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(UHSMSW::SDR104)
    }
    #[doc = "DDR50"]
    #[inline]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(UHSMSW::DDR50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VS18EN`"]
pub enum VS18ENW {
    #[doc = "3.3V Signaling"]
    S33V,
    #[doc = "1.8V Signaling"]
    S18V,
}
impl VS18ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VS18ENW::S33V => false,
            VS18ENW::S18V => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VS18ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VS18ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VS18ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "3.3V Signaling"]
    #[inline]
    pub fn s33v(self) -> &'a mut W {
        self.variant(VS18ENW::S33V)
    }
    #[doc = "1.8V Signaling"]
    #[inline]
    pub fn s18v(self) -> &'a mut W {
        self.variant(VS18ENW::S18V)
    }
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRVSEL`"]
pub enum DRVSELW {
    #[doc = "Driver Type B is Selected (Default)"]
    B,
    #[doc = "Driver Type A is Selected"]
    A,
    #[doc = "Driver Type C is Selected"]
    C,
    #[doc = "Driver Type D is Selected"]
    D,
}
impl DRVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRVSELW::B => 0,
            DRVSELW::A => 1,
            DRVSELW::C => 2,
            DRVSELW::D => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DRVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Driver Type B is Selected (Default)"]
    #[inline]
    pub fn b(self) -> &'a mut W {
        self.variant(DRVSELW::B)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline]
    pub fn a(self) -> &'a mut W {
        self.variant(DRVSELW::A)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline]
    pub fn c(self) -> &'a mut W {
        self.variant(DRVSELW::C)
    }
    #[doc = "Driver Type D is Selected"]
    #[inline]
    pub fn d(self) -> &'a mut W {
        self.variant(DRVSELW::D)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTUN`"]
pub enum EXTUNW {
    #[doc = "Not Tuned or Tuning Completed"]
    NO,
    #[doc = "Execute Tuning"]
    REQUESTED,
}
impl EXTUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTUNW::NO => false,
            EXTUNW::REQUESTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTUNW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(EXTUNW::NO)
    }
    #[doc = "Execute Tuning"]
    #[inline]
    pub fn requested(self) -> &'a mut W {
        self.variant(EXTUNW::REQUESTED)
    }
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLCKSEL`"]
pub enum SLCKSELW {
    #[doc = "Fixed clock is used to sample data"]
    FIXED,
    #[doc = "Tuned clock is used to sample data"]
    TUNED,
}
impl SLCKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLCKSELW::FIXED => false,
            SLCKSELW::TUNED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLCKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLCKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SLCKSELW::FIXED)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline]
    pub fn tuned(self) -> &'a mut W {
        self.variant(SLCKSELW::TUNED)
    }
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASINTEN`"]
pub enum ASINTENW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ASINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASINTENW::DISABLED => false,
            ASINTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASINTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASINTENW::ENABLED)
    }
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
#[doc = "Values that can be written to the field `PVALEN`"]
pub enum PVALENW {
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    HOST,
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    AUTO,
}
impl PVALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PVALENW::HOST => false,
            PVALENW::AUTO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PVALENW<'a> {
    w: &'a mut W,
}
impl<'a> _PVALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PVALENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Controller"]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(PVALENW::HOST)
    }
    #[doc = "Automatic Selection by Preset Value is Enabled"]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(PVALENW::AUTO)
    }
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
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline]
    pub fn uhsms(&self) -> UHSMSR {
        UHSMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline]
    pub fn vs18en(&self) -> VS18ENR {
        VS18ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline]
    pub fn drvsel(&self) -> DRVSELR {
        DRVSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline]
    pub fn extun(&self) -> EXTUNR {
        EXTUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline]
    pub fn slcksel(&self) -> SLCKSELR {
        SLCKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline]
    pub fn asinten(&self) -> ASINTENR {
        ASINTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline]
    pub fn pvalen(&self) -> PVALENR {
        PVALENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
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
    #[doc = "Bits 0:2 - UHS Mode Select"]
    #[inline]
    pub fn uhsms(&mut self) -> _UHSMSW {
        _UHSMSW { w: self }
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable"]
    #[inline]
    pub fn vs18en(&mut self) -> _VS18ENW {
        _VS18ENW { w: self }
    }
    #[doc = "Bits 4:5 - Driver Strength Select"]
    #[inline]
    pub fn drvsel(&mut self) -> _DRVSELW {
        _DRVSELW { w: self }
    }
    #[doc = "Bit 6 - Execute Tuning"]
    #[inline]
    pub fn extun(&mut self) -> _EXTUNW {
        _EXTUNW { w: self }
    }
    #[doc = "Bit 7 - Sampling Clock Select"]
    #[inline]
    pub fn slcksel(&mut self) -> _SLCKSELW {
        _SLCKSELW { w: self }
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable"]
    #[inline]
    pub fn asinten(&mut self) -> _ASINTENW {
        _ASINTENW { w: self }
    }
    #[doc = "Bit 15 - Preset Value Enable"]
    #[inline]
    pub fn pvalen(&mut self) -> _PVALENW {
        _PVALENW { w: self }
    }
}
