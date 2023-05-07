#[doc = "Register `ILE` reader"]
pub struct R(crate::R<ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILE` writer"]
pub struct W(crate::W<ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILE_SPEC>;
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
impl From<crate::W<ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EINT0` reader - Enable Interrupt Line 0"]
pub type EINT0_R = crate::BitReader<bool>;
#[doc = "Field `EINT0` writer - Enable Interrupt Line 0"]
pub type EINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILE_SPEC, bool, O>;
#[doc = "Field `EINT1` reader - Enable Interrupt Line 1"]
pub type EINT1_R = crate::BitReader<bool>;
#[doc = "Field `EINT1` writer - Enable Interrupt Line 1"]
pub type EINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ILE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<0> {
        EINT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<1> {
        EINT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Line Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](index.html) module"]
pub struct ILE_SPEC;
impl crate::RegisterSpec for ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ile::R](R) reader structure"]
impl crate::Readable for ILE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ile::W](W) writer structure"]
impl crate::Writable for ILE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for ILE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
