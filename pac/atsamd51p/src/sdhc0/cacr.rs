#[doc = "Register `CACR` reader"]
pub struct R(crate::R<CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR` writer"]
pub struct W(crate::W<CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR_SPEC>;
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
impl From<crate::W<CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPWREN` reader - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CAPWREN_R = crate::BitReader<bool>;
#[doc = "Field `CAPWREN` writer - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CAPWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACR_SPEC, bool, O>;
#[doc = "Field `KEY` reader - Key (0x46)"]
pub type KEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY` writer - Key (0x46)"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CACR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&self) -> CAPWREN_R {
        CAPWREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    #[must_use]
    pub fn capwren(&mut self) -> CAPWREN_W<0> {
        CAPWREN_W::new(self)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capabilities Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](index.html) module"]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacr::R](R) reader structure"]
impl crate::Readable for CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr::W](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
