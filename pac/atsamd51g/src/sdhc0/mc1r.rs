#[doc = "Reader of register MC1R"]
pub type R = crate::R<u8, super::MC1R>;
#[doc = "Writer for register MC1R"]
pub type W = crate::W<u8, super::MC1R>;
#[doc = "Register MC1R `reset()`'s with value 0"]
impl crate::ResetValue for super::MC1R {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "e.MMC Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDTYP_A {
    #[doc = "0: Not a MMC specific command"]
    NORMAL = 0,
    #[doc = "1: Wait IRQ Command"]
    WAITIRQ = 1,
    #[doc = "2: Stream Command"]
    STREAM = 2,
    #[doc = "3: Boot Command"]
    BOOT = 3,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDTYP`"]
pub type CMDTYP_R = crate::R<u8, CMDTYP_A>;
impl CMDTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::NORMAL,
            1 => CMDTYP_A::WAITIRQ,
            2 => CMDTYP_A::STREAM,
            3 => CMDTYP_A::BOOT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `WAITIRQ`"]
    #[inline(always)]
    pub fn is_waitirq(&self) -> bool {
        *self == CMDTYP_A::WAITIRQ
    }
    #[doc = "Checks if the value of the field is `STREAM`"]
    #[inline(always)]
    pub fn is_stream(&self) -> bool {
        *self == CMDTYP_A::STREAM
    }
    #[doc = "Checks if the value of the field is `BOOT`"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == CMDTYP_A::BOOT
    }
}
#[doc = "Write proxy for field `CMDTYP`"]
pub struct CMDTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Not a MMC specific command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYP_A::NORMAL)
    }
    #[doc = "Wait IRQ Command"]
    #[inline(always)]
    pub fn waitirq(self) -> &'a mut W {
        self.variant(CMDTYP_A::WAITIRQ)
    }
    #[doc = "Stream Command"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut W {
        self.variant(CMDTYP_A::STREAM)
    }
    #[doc = "Boot Command"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut W {
        self.variant(CMDTYP_A::BOOT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DDR`"]
pub type DDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR`"]
pub struct DDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OPD`"]
pub type OPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPD`"]
pub struct OPD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BOOTA`"]
pub type BOOTA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTA`"]
pub struct BOOTA_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RSTN`"]
pub type RSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTN`"]
pub struct RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FCD`"]
pub type FCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCD`"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    pub fn boota(&self) -> BOOTA_R {
        BOOTA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - e.MMC Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CMDTYP_W {
        CMDTYP_W { w: self }
    }
    #[doc = "Bit 3 - e.MMC HSDDR Mode"]
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W {
        DDR_W { w: self }
    }
    #[doc = "Bit 4 - e.MMC Open Drain Mode"]
    #[inline(always)]
    pub fn opd(&mut self) -> OPD_W {
        OPD_W { w: self }
    }
    #[doc = "Bit 5 - e.MMC Boot Acknowledge Enable"]
    #[inline(always)]
    pub fn boota(&mut self) -> BOOTA_W {
        BOOTA_W { w: self }
    }
    #[doc = "Bit 6 - e.MMC Reset Signal"]
    #[inline(always)]
    pub fn rstn(&mut self) -> RSTN_W {
        RSTN_W { w: self }
    }
    #[doc = "Bit 7 - e.MMC Force Card Detect"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
}
