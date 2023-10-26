#[doc = "Register `PADCAL` reader"]
pub type R = crate::R<PADCAL_SPEC>;
#[doc = "Register `PADCAL` writer"]
pub type W = crate::W<PADCAL_SPEC>;
#[doc = "Field `TRANSP` reader - USB Pad Transp calibration"]
pub type TRANSP_R = crate::FieldReader;
#[doc = "Field `TRANSP` writer - USB Pad Transp calibration"]
pub type TRANSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TRANSN` reader - USB Pad Transn calibration"]
pub type TRANSN_R = crate::FieldReader;
#[doc = "Field `TRANSN` writer - USB Pad Transn calibration"]
pub type TRANSN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TRIM` reader - USB Pad Trim calibration"]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - USB Pad Trim calibration"]
pub type TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    pub fn transp(&self) -> TRANSP_R {
        TRANSP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    pub fn transn(&self) -> TRANSN_R {
        TRANSN_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - USB Pad Transp calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transp(&mut self) -> TRANSP_W<PADCAL_SPEC, 0> {
        TRANSP_W::new(self)
    }
    #[doc = "Bits 6:10 - USB Pad Transn calibration"]
    #[inline(always)]
    #[must_use]
    pub fn transn(&mut self) -> TRANSN_W<PADCAL_SPEC, 6> {
        TRANSN_W::new(self)
    }
    #[doc = "Bits 12:14 - USB Pad Trim calibration"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<PADCAL_SPEC, 12> {
        TRIM_W::new(self)
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
#[doc = "USB PAD Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCAL_SPEC;
impl crate::RegisterSpec for PADCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`padcal::R`](R) reader structure"]
impl crate::Readable for PADCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcal::W`](W) writer structure"]
impl crate::Writable for PADCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCAL to value 0"]
impl crate::Resettable for PADCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
