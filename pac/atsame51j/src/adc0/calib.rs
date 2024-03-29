#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIB_SPEC>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIB_SPEC>;
#[doc = "Field `BIASCOMP` reader - Bias Comparator Scaling"]
pub type BIASCOMP_R = crate::FieldReader;
#[doc = "Field `BIASCOMP` writer - Bias Comparator Scaling"]
pub type BIASCOMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BIASR2R` reader - Bias R2R Ampli scaling"]
pub type BIASR2R_R = crate::FieldReader;
#[doc = "Field `BIASR2R` writer - Bias R2R Ampli scaling"]
pub type BIASR2R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BIASREFBUF` reader - Bias Reference Buffer Scaling"]
pub type BIASREFBUF_R = crate::FieldReader;
#[doc = "Field `BIASREFBUF` writer - Bias Reference Buffer Scaling"]
pub type BIASREFBUF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    pub fn biascomp(&self) -> BIASCOMP_R {
        BIASCOMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    pub fn biasr2r(&self) -> BIASR2R_R {
        BIASR2R_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    pub fn biasrefbuf(&self) -> BIASREFBUF_R {
        BIASREFBUF_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Comparator Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biascomp(&mut self) -> BIASCOMP_W<CALIB_SPEC, 0> {
        BIASCOMP_W::new(self)
    }
    #[doc = "Bits 4:6 - Bias R2R Ampli scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biasr2r(&mut self) -> BIASR2R_W<CALIB_SPEC, 4> {
        BIASR2R_W::new(self)
    }
    #[doc = "Bits 8:10 - Bias Reference Buffer Scaling"]
    #[inline(always)]
    #[must_use]
    pub fn biasrefbuf(&mut self) -> BIASREFBUF_W<CALIB_SPEC, 8> {
        BIASREFBUF_W::new(self)
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
#[doc = "Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
