#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LQOS` reader - Latency Quality Of Service"]
pub struct LQOS_R(crate::FieldReader<u8, u8>);
impl LQOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LQOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LQOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LQOS` writer - Latency Quality Of Service"]
pub struct LQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "DMA Trigger Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCCDMALEVEL_A {
    #[doc = "0: Trigger rises when DCC is empty"]
    EMPTY = 0,
    #[doc = "1: Trigger rises when DCC is full"]
    FULL = 1,
}
impl From<DCCDMALEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCCDMALEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DCCDMALEVEL` reader - DMA Trigger Level"]
pub struct DCCDMALEVEL_R(crate::FieldReader<u8, DCCDMALEVEL_A>);
impl DCCDMALEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCCDMALEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCCDMALEVEL_A> {
        match self.bits {
            0 => Some(DCCDMALEVEL_A::EMPTY),
            1 => Some(DCCDMALEVEL_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == DCCDMALEVEL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == DCCDMALEVEL_A::FULL
    }
}
impl core::ops::Deref for DCCDMALEVEL_R {
    type Target = crate::FieldReader<u8, DCCDMALEVEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCCDMALEVEL` writer - DMA Trigger Level"]
pub struct DCCDMALEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCCDMALEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCCDMALEVEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(DCCDMALEVEL_A::EMPTY)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(DCCDMALEVEL_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ETBRAMEN` reader - Trace Control"]
pub struct ETBRAMEN_R(crate::FieldReader<bool, bool>);
impl ETBRAMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETBRAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETBRAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETBRAMEN` writer - Trace Control"]
pub struct ETBRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETBRAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&self) -> LQOS_R {
        LQOS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&self) -> DCCDMALEVEL_R {
        DCCDMALEVEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&self) -> ETBRAMEN_R {
        ETBRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&mut self) -> LQOS_W {
        LQOS_W { w: self }
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&mut self) -> DCCDMALEVEL_W {
        DCCDMALEVEL_W { w: self }
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&mut self) -> ETBRAMEN_W {
        ETBRAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x02"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
