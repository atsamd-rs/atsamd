#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIB_SPEC>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIB_SPEC>;
#[doc = "Field `LINEARITY_CAL` reader - Linearity Calibration Value"]
pub type LINEARITY_CAL_R = crate::FieldReader;
#[doc = "Field `LINEARITY_CAL` writer - Linearity Calibration Value"]
pub type LINEARITY_CAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BIAS_CAL` reader - Bias Calibration Value"]
pub type BIAS_CAL_R = crate::FieldReader;
#[doc = "Field `BIAS_CAL` writer - Bias Calibration Value"]
pub type BIAS_CAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    pub fn linearity_cal(&self) -> LINEARITY_CAL_R {
        LINEARITY_CAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    pub fn bias_cal(&self) -> BIAS_CAL_R {
        BIAS_CAL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linearity Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn linearity_cal(&mut self) -> LINEARITY_CAL_W<CALIB_SPEC, 0> {
        LINEARITY_CAL_W::new(self)
    }
    #[doc = "Bits 8:10 - Bias Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn bias_cal(&mut self) -> BIAS_CAL_W<CALIB_SPEC, 8> {
        BIAS_CAL_W::new(self)
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
