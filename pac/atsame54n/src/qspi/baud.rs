#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, bool, O>;
#[doc = "Field `BAUD` reader - Serial Clock Baud Rate"]
pub type BAUD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAUD` writer - Serial Clock Baud Rate"]
pub type BAUD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SCK"]
pub type DLYBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBS` writer - Delay Before SCK"]
pub type DLYBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<1> {
        CPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<8> {
        BAUD_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DLYBS_W<16> {
        DLYBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
