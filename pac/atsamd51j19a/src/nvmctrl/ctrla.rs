#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u16 {
        0x04
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct AUTOWSR {
    bits: bool,
}
impl AUTOWSR {
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
pub struct SUSPENR {
    bits: bool,
}
impl SUSPENR {
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
#[doc = "Possible values of the field `WMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODER {
    #[doc = "Manual Write"]
    MAN,
    #[doc = "Automatic Double Word Write"]
    ADW,
    #[doc = "Automatic Quad Word"]
    AQW,
    #[doc = "Automatic Page Write"]
    AP,
}
impl WMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WMODER::MAN => 0,
            WMODER::ADW => 0x01,
            WMODER::AQW => 0x02,
            WMODER::AP => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WMODER {
        match value {
            0 => WMODER::MAN,
            1 => WMODER::ADW,
            2 => WMODER::AQW,
            3 => WMODER::AP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline]
    pub fn is_man(&self) -> bool {
        *self == WMODER::MAN
    }
    #[doc = "Checks if the value of the field is `ADW`"]
    #[inline]
    pub fn is_adw(&self) -> bool {
        *self == WMODER::ADW
    }
    #[doc = "Checks if the value of the field is `AQW`"]
    #[inline]
    pub fn is_aqw(&self) -> bool {
        *self == WMODER::AQW
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline]
    pub fn is_ap(&self) -> bool {
        *self == WMODER::AP
    }
}
#[doc = "Possible values of the field `PRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMR {
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    SEMIAUTO,
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    FULLAUTO,
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    MANUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRMR::SEMIAUTO => 0,
            PRMR::FULLAUTO => 0x01,
            PRMR::MANUAL => 0x03,
            PRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRMR {
        match value {
            0 => PRMR::SEMIAUTO,
            1 => PRMR::FULLAUTO,
            3 => PRMR::MANUAL,
            i => PRMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SEMIAUTO`"]
    #[inline]
    pub fn is_semiauto(&self) -> bool {
        *self == PRMR::SEMIAUTO
    }
    #[doc = "Checks if the value of the field is `FULLAUTO`"]
    #[inline]
    pub fn is_fullauto(&self) -> bool {
        *self == PRMR::FULLAUTO
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline]
    pub fn is_manual(&self) -> bool {
        *self == PRMR::MANUAL
    }
}
#[doc = "Possible values of the field `RWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSR {
    #[doc = "Single Auto Wait State"]
    SINGLE,
    #[doc = "Half Auto Wait State"]
    HALF,
    #[doc = "Dual Auto Wait State"]
    DUAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RWSR::SINGLE => 0,
            RWSR::HALF => 0x01,
            RWSR::DUAL => 0x02,
            RWSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RWSR {
        match value {
            0 => RWSR::SINGLE,
            1 => RWSR::HALF,
            2 => RWSR::DUAL,
            i => RWSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == RWSR::SINGLE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == RWSR::HALF
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == RWSR::DUAL
    }
}
#[doc = r" Value of the field"]
pub struct AHBNS0R {
    bits: bool,
}
impl AHBNS0R {
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
pub struct AHBNS1R {
    bits: bool,
}
impl AHBNS1R {
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
pub struct CACHEDIS0R {
    bits: bool,
}
impl CACHEDIS0R {
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
pub struct CACHEDIS1R {
    bits: bool,
}
impl CACHEDIS1R {
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
#[doc = r" Proxy"]
pub struct _AUTOWSW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOWSW<'a> {
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u16) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUSPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENW<'a> {
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
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u16) & 0x01) << 3;
        self.w
    }
}
#[doc = "Values that can be written to the field `WMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMODEW {
    #[doc = "Manual Write"]
    MAN,
    #[doc = "Automatic Double Word Write"]
    ADW,
    #[doc = "Automatic Quad Word"]
    AQW,
    #[doc = "Automatic Page Write"]
    AP,
}
impl WMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WMODEW::MAN => 0,
            WMODEW::ADW => 1,
            WMODEW::AQW => 2,
            WMODEW::AP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Manual Write"]
    #[inline]
    pub fn man(self) -> &'a mut W {
        self.variant(WMODEW::MAN)
    }
    #[doc = "Automatic Double Word Write"]
    #[inline]
    pub fn adw(self) -> &'a mut W {
        self.variant(WMODEW::ADW)
    }
    #[doc = "Automatic Quad Word"]
    #[inline]
    pub fn aqw(self) -> &'a mut W {
        self.variant(WMODEW::AQW)
    }
    #[doc = "Automatic Page Write"]
    #[inline]
    pub fn ap(self) -> &'a mut W {
        self.variant(WMODEW::AP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 4);
        self.w.bits |= ((value as u16) & 0x03) << 4;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMW {
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    SEMIAUTO,
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    FULLAUTO,
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    MANUAL,
}
impl PRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRMW::SEMIAUTO => 0,
            PRMW::FULLAUTO => 1,
            PRMW::MANUAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRMW<'a> {
    w: &'a mut W,
}
impl<'a> _PRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline]
    pub fn semiauto(self) -> &'a mut W {
        self.variant(PRMW::SEMIAUTO)
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline]
    pub fn fullauto(self) -> &'a mut W {
        self.variant(PRMW::FULLAUTO)
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline]
    pub fn manual(self) -> &'a mut W {
        self.variant(PRMW::MANUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 6);
        self.w.bits |= ((value as u16) & 0x03) << 6;
        self.w
    }
}
#[doc = "Values that can be written to the field `RWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSW {
    #[doc = "Single Auto Wait State"]
    SINGLE,
    #[doc = "Half Auto Wait State"]
    HALF,
    #[doc = "Dual Auto Wait State"]
    DUAL,
}
impl RWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RWSW::SINGLE => 0,
            RWSW::HALF => 1,
            RWSW::DUAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWSW<'a> {
    w: &'a mut W,
}
impl<'a> _RWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single Auto Wait State"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(RWSW::SINGLE)
    }
    #[doc = "Half Auto Wait State"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(RWSW::HALF)
    }
    #[doc = "Dual Auto Wait State"]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(RWSW::DUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 8);
        self.w.bits |= ((value as u16) & 0x0f) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHBNS0W<'a> {
    w: &'a mut W,
}
impl<'a> _AHBNS0W<'a> {
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
        self.w.bits &= !(0x01 << 12);
        self.w.bits |= ((value as u16) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHBNS1W<'a> {
    w: &'a mut W,
}
impl<'a> _AHBNS1W<'a> {
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
        self.w.bits &= !(0x01 << 13);
        self.w.bits |= ((value as u16) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CACHEDIS0W<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEDIS0W<'a> {
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
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u16) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CACHEDIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEDIS1W<'a> {
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
        self.w.bits &= !(0x01 << 15);
        self.w.bits |= ((value as u16) & 0x01) << 15;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline]
    pub fn autows(&self) -> AUTOWSR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        AUTOWSR { bits }
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline]
    pub fn suspen(&self) -> SUSPENR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        SUSPENR { bits }
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline]
    pub fn wmode(&self) -> WMODER {
        WMODER::_from(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn prm(&self) -> PRMR {
        PRMR::_from(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&self) -> RWSR {
        RWSR::_from(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline]
    pub fn ahbns0(&self) -> AHBNS0R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        AHBNS0R { bits }
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline]
    pub fn ahbns1(&self) -> AHBNS1R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        AHBNS1R { bits }
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline]
    pub fn cachedis0(&self) -> CACHEDIS0R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        CACHEDIS0R { bits }
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline]
    pub fn cachedis1(&self) -> CACHEDIS1R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        CACHEDIS1R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline]
    pub fn autows(&mut self) -> _AUTOWSW {
        _AUTOWSW { w: self }
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline]
    pub fn suspen(&mut self) -> _SUSPENW {
        _SUSPENW { w: self }
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline]
    pub fn wmode(&mut self) -> _WMODEW {
        _WMODEW { w: self }
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline]
    pub fn prm(&mut self) -> _PRMW {
        _PRMW { w: self }
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline]
    pub fn rws(&mut self) -> _RWSW {
        _RWSW { w: self }
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline]
    pub fn ahbns0(&mut self) -> _AHBNS0W {
        _AHBNS0W { w: self }
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline]
    pub fn ahbns1(&mut self) -> _AHBNS1W {
        _AHBNS1W { w: self }
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline]
    pub fn cachedis0(&mut self) -> _CACHEDIS0W {
        _CACHEDIS0W { w: self }
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline]
    pub fn cachedis1(&mut self) -> _CACHEDIS1W {
        _CACHEDIS1W { w: self }
    }
}
