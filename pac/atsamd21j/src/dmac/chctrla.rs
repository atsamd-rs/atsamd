#[doc = "Register `CHCTRLA` reader"]
pub struct R(crate::R<CHCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLA` writer"]
pub struct W(crate::W<CHCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLA_SPEC>;
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
impl From<crate::W<CHCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrla](index.html) module"]
pub struct CHCTRLA_SPEC;
impl crate::RegisterSpec for CHCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chctrla::R](R) reader structure"]
impl crate::Readable for CHCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrla::W](W) writer structure"]
impl crate::Writable for CHCTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for CHCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
