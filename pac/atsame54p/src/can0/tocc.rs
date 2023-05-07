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
pub type ETOC_R = crate::BitReader<bool>;
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type ETOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCC_SPEC, bool, O>;
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TOS_R = crate::FieldReader<u8, TOSSELECT_A>;
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOSSELECT_A {
    #[doc = "0: Continuout operation"]
    CONT = 0,
    #[doc = "1: Timeout controlled by TX Event FIFO"]
    TXEF = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    RXF0 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    RXF1 = 3,
}
impl From<TOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TOSSELECT_A) -> Self {
        variant as _
    }
}
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOSSELECT_A {
        match self.bits {
            0 => TOSSELECT_A::CONT,
            1 => TOSSELECT_A::TXEF,
            2 => TOSSELECT_A::RXF0,
            3 => TOSSELECT_A::RXF1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == TOSSELECT_A::CONT
    }
    #[doc = "Checks if the value of the field is `TXEF`"]
    #[inline(always)]
    pub fn is_txef(&self) -> bool {
        *self == TOSSELECT_A::TXEF
    }
    #[doc = "Checks if the value of the field is `RXF0`"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == TOSSELECT_A::RXF0
    }
    #[doc = "Checks if the value of the field is `RXF1`"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == TOSSELECT_A::RXF1
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TOCC_SPEC, u8, TOSSELECT_A, 2, O>;
impl<'a, const O: u8> TOS_W<'a, O> {
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut W {
        self.variant(TOSSELECT_A::CONT)
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn txef(self) -> &'a mut W {
        self.variant(TOSSELECT_A::TXEF)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut W {
        self.variant(TOSSELECT_A::RXF0)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut W {
        self.variant(TOSSELECT_A::RXF1)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
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
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<0> {
        ETOC_W::new(self)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<1> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<16> {
        TOP_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
