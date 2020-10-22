#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u16, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u16, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0x04"]
impl crate::ResetValue for super::CTRLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `AUTOWS`"]
pub type AUTOWS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOWS`"]
pub struct AUTOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOWS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SUSPEN`"]
pub type SUSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEN`"]
pub struct SUSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WMODE_A {
    #[doc = "0: Manual Write"]
    MAN = 0,
    #[doc = "1: Automatic Double Word Write"]
    ADW = 1,
    #[doc = "2: Automatic Quad Word"]
    AQW = 2,
    #[doc = "3: Automatic Page Write"]
    AP = 3,
}
impl From<WMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WMODE`"]
pub type WMODE_R = crate::R<u8, WMODE_A>;
impl WMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMODE_A {
        match self.bits {
            0 => WMODE_A::MAN,
            1 => WMODE_A::ADW,
            2 => WMODE_A::AQW,
            3 => WMODE_A::AP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline(always)]
    pub fn is_man(&self) -> bool {
        *self == WMODE_A::MAN
    }
    #[doc = "Checks if the value of the field is `ADW`"]
    #[inline(always)]
    pub fn is_adw(&self) -> bool {
        *self == WMODE_A::ADW
    }
    #[doc = "Checks if the value of the field is `AQW`"]
    #[inline(always)]
    pub fn is_aqw(&self) -> bool {
        *self == WMODE_A::AQW
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == WMODE_A::AP
    }
}
#[doc = "Write proxy for field `WMODE`"]
pub struct WMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Manual Write"]
    #[inline(always)]
    pub fn man(self) -> &'a mut W {
        self.variant(WMODE_A::MAN)
    }
    #[doc = "Automatic Double Word Write"]
    #[inline(always)]
    pub fn adw(self) -> &'a mut W {
        self.variant(WMODE_A::ADW)
    }
    #[doc = "Automatic Quad Word"]
    #[inline(always)]
    pub fn aqw(self) -> &'a mut W {
        self.variant(WMODE_A::AQW)
    }
    #[doc = "Automatic Page Write"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(WMODE_A::AP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Power Reduction Mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRM_A {
    #[doc = "0: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    SEMIAUTO = 0,
    #[doc = "1: NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    FULLAUTO = 1,
    #[doc = "3: NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    MANUAL = 3,
}
impl From<PRM_A> for u8 {
    #[inline(always)]
    fn from(variant: PRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRM`"]
pub type PRM_R = crate::R<u8, PRM_A>;
impl PRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRM_A::SEMIAUTO),
            1 => Val(PRM_A::FULLAUTO),
            3 => Val(PRM_A::MANUAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SEMIAUTO`"]
    #[inline(always)]
    pub fn is_semiauto(&self) -> bool {
        *self == PRM_A::SEMIAUTO
    }
    #[doc = "Checks if the value of the field is `FULLAUTO`"]
    #[inline(always)]
    pub fn is_fullauto(&self) -> bool {
        *self == PRM_A::FULLAUTO
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == PRM_A::MANUAL
    }
}
#[doc = "Write proxy for field `PRM`"]
pub struct PRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn semiauto(self) -> &'a mut W {
        self.variant(PRM_A::SEMIAUTO)
    }
    #[doc = "NVM block enters low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode when system is not in standby mode."]
    #[inline(always)]
    pub fn fullauto(self) -> &'a mut W {
        self.variant(PRM_A::FULLAUTO)
    }
    #[doc = "NVM block does not enter low-power mode when entering standby mode. NVM block enters low-power mode when SPRM command is issued. NVM block exits low-power mode upon first access."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(PRM_A::MANUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `RWS`"]
pub type RWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWS`"]
pub struct RWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AHBNS0`"]
pub type AHBNS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBNS0`"]
pub struct AHBNS0_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBNS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `AHBNS1`"]
pub type AHBNS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBNS1`"]
pub struct AHBNS1_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBNS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CACHEDIS0`"]
pub type CACHEDIS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEDIS0`"]
pub struct CACHEDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CACHEDIS1`"]
pub type CACHEDIS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEDIS1`"]
pub struct CACHEDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEDIS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&self) -> AUTOWS_R {
        AUTOWS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&self) -> WMODE_R {
        WMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&self) -> AHBNS0_R {
        AHBNS0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&self) -> AHBNS1_R {
        AHBNS1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&self) -> CACHEDIS0_R {
        CACHEDIS0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&self) -> CACHEDIS1_R {
        CACHEDIS1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Auto Wait State Enable"]
    #[inline(always)]
    pub fn autows(&mut self) -> AUTOWS_W {
        AUTOWS_W { w: self }
    }
    #[doc = "Bit 3 - Suspend Enable"]
    #[inline(always)]
    pub fn suspen(&mut self) -> SUSPEN_W {
        SUSPEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Write Mode"]
    #[inline(always)]
    pub fn wmode(&mut self) -> WMODE_W {
        WMODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Power Reduction Mode during Sleep"]
    #[inline(always)]
    pub fn prm(&mut self) -> PRM_W {
        PRM_W { w: self }
    }
    #[doc = "Bits 8:11 - NVM Read Wait States"]
    #[inline(always)]
    pub fn rws(&mut self) -> RWS_W {
        RWS_W { w: self }
    }
    #[doc = "Bit 12 - Force AHB0 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns0(&mut self) -> AHBNS0_W {
        AHBNS0_W { w: self }
    }
    #[doc = "Bit 13 - Force AHB1 access to NONSEQ, burst transfers are continuously rearbitrated"]
    #[inline(always)]
    pub fn ahbns1(&mut self) -> AHBNS1_W {
        AHBNS1_W { w: self }
    }
    #[doc = "Bit 14 - AHB0 Cache Disable"]
    #[inline(always)]
    pub fn cachedis0(&mut self) -> CACHEDIS0_W {
        CACHEDIS0_W { w: self }
    }
    #[doc = "Bit 15 - AHB1 Cache Disable"]
    #[inline(always)]
    pub fn cachedis1(&mut self) -> CACHEDIS1_W {
        CACHEDIS1_W { w: self }
    }
}
