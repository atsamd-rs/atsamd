#[doc = "Register `TOCC` reader"]
pub struct R(crate::R<TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCC` writer"]
pub struct W(crate::W<TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub struct ETOC_R(crate::FieldReader<bool, bool>);
impl ETOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: Continuout operation"]
    CONT = 0,
    #[doc = "1: Timeout controlled by TX Event FIFO"]
    TXEF = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    RXF0 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    RXF1 = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOS` reader - Timeout Select"]
pub struct TOS_R(crate::FieldReader<u8, TOS_A>);
impl TOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::CONT,
            1 => TOS_A::TXEF,
            2 => TOS_A::RXF0,
            3 => TOS_A::RXF1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        **self == TOS_A::CONT
    }
    #[doc = "Checks if the value of the field is `TXEF`"]
    #[inline(always)]
    pub fn is_txef(&self) -> bool {
        **self == TOS_A::TXEF
    }
    #[doc = "Checks if the value of the field is `RXF0`"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        **self == TOS_A::RXF0
    }
    #[doc = "Checks if the value of the field is `RXF1`"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        **self == TOS_A::RXF1
    }
}
impl core::ops::Deref for TOS_R {
    type Target = crate::FieldReader<u8, TOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(TOS_A::CONT)
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn txef(self) -> &'a mut W {
        self.variant(TOS_A::TXEF)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut W {
        self.variant(TOS_A::RXF0)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut W {
        self.variant(TOS_A::RXF1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub struct TOP_R(crate::FieldReader<u16, u16>);
impl TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP` writer - Timeout Period"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](index.html) module"]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocc::R](R) reader structure"]
impl crate::Readable for TOCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocc::W](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
