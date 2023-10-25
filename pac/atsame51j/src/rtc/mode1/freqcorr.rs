#[doc = "Register `FREQCORR` reader"]
pub type R = crate::R<FREQCORR_SPEC>;
#[doc = "Register `FREQCORR` writer"]
pub type W = crate::W<FREQCORR_SPEC>;
#[doc = "Field `VALUE` reader - Correction Value"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - Correction Value"]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SIGN` reader - Correction Sign"]
pub type SIGN_R = crate::BitReader;
#[doc = "Field `SIGN` writer - Correction Sign"]
pub type SIGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<FREQCORR_SPEC, 0> {
        VALUE_W::new(self)
    }
    #[doc = "Bit 7 - Correction Sign"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<FREQCORR_SPEC, 7> {
        SIGN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frequency Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqcorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqcorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQCORR_SPEC;
impl crate::RegisterSpec for FREQCORR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`freqcorr::R`](R) reader structure"]
impl crate::Readable for FREQCORR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freqcorr::W`](W) writer structure"]
impl crate::Writable for FREQCORR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQCORR to value 0"]
impl crate::Resettable for FREQCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
