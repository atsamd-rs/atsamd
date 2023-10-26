#[doc = "Register `FNUM` reader"]
pub type R = crate::R<FNUM_SPEC>;
#[doc = "Register `FNUM` writer"]
pub type W = crate::W<FNUM_SPEC>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MFNUM_R = crate::FieldReader;
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub type MFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new((self.bits >> 3) & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn mfnum(&mut self) -> MFNUM_W<FNUM_SPEC, 0> {
        MFNUM_W::new(self)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FNUM_W<FNUM_SPEC, 3> {
        FNUM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HOST Host Frame Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FNUM_SPEC;
impl crate::RegisterSpec for FNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fnum::R`](R) reader structure"]
impl crate::Readable for FNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fnum::W`](W) writer structure"]
impl crate::Writable for FNUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
