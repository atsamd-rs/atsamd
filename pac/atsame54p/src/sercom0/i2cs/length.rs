#[doc = "Register `LENGTH` reader"]
pub struct R(crate::R<LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LENGTH` writer"]
pub struct W(crate::W<LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LENGTH_SPEC>;
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
impl From<crate::W<LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - Data Length"]
pub type LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN` writer - Data Length"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, LENGTH_SPEC, u8, u8, 8, O>;
#[doc = "Field `LENEN` reader - Data Length Enable"]
pub type LENEN_R = crate::BitReader<bool>;
#[doc = "Field `LENEN` writer - Data Length Enable"]
pub type LENEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, LENGTH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lenen(&mut self) -> LENEN_W<8> {
        LENEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](index.html) module"]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [length::R](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [length::W](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
