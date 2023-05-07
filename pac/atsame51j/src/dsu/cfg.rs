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
pub type LQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LQOS` writer - Latency Quality Of Service"]
pub type LQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DCCDMALEVEL` reader - DMA Trigger Level"]
pub type DCCDMALEVEL_R = crate::FieldReader<u8, DCCDMALEVELSELECT_A>;
#[doc = "DMA Trigger Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCCDMALEVELSELECT_A {
    #[doc = "0: Trigger rises when DCC is empty"]
    EMPTY = 0,
    #[doc = "1: Trigger rises when DCC is full"]
    FULL = 1,
}
impl From<DCCDMALEVELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DCCDMALEVELSELECT_A) -> Self {
        variant as _
    }
}
impl DCCDMALEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCCDMALEVELSELECT_A> {
        match self.bits {
            0 => Some(DCCDMALEVELSELECT_A::EMPTY),
            1 => Some(DCCDMALEVELSELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == DCCDMALEVELSELECT_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DCCDMALEVELSELECT_A::FULL
    }
}
#[doc = "Field `DCCDMALEVEL` writer - DMA Trigger Level"]
pub type DCCDMALEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_SPEC, u8, DCCDMALEVELSELECT_A, 2, O>;
impl<'a, const O: u8> DCCDMALEVEL_W<'a, O> {
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(DCCDMALEVELSELECT_A::EMPTY)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(DCCDMALEVELSELECT_A::FULL)
    }
}
#[doc = "Field `ETBRAMEN` reader - Trace Control"]
pub type ETBRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `ETBRAMEN` writer - Trace Control"]
pub type ETBRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&self) -> LQOS_R {
        LQOS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&self) -> DCCDMALEVEL_R {
        DCCDMALEVEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&self) -> ETBRAMEN_R {
        ETBRAMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    #[must_use]
    pub fn lqos(&mut self) -> LQOS_W<0> {
        LQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn dccdmalevel(&mut self) -> DCCDMALEVEL_W<2> {
        DCCDMALEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    #[must_use]
    pub fn etbramen(&mut self) -> ETBRAMEN_W<4> {
        ETBRAMEN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x02"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
