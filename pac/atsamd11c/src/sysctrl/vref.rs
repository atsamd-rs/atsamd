#[doc = "Register `VREF` reader"]
pub type R = crate::R<VREF_SPEC>;
#[doc = "Register `VREF` writer"]
pub type W = crate::W<VREF_SPEC>;
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BGOUTEN` reader - Bandgap Output Enable"]
pub type BGOUTEN_R = crate::BitReader;
#[doc = "Field `BGOUTEN` writer - Bandgap Output Enable"]
pub type BGOUTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALIB` reader - Bandgap Voltage Generator Calibration"]
pub type CALIB_R = crate::FieldReader<u16>;
#[doc = "Field `CALIB` writer - Bandgap Voltage Generator Calibration"]
pub type CALIB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    pub fn bgouten(&self) -> BGOUTEN_R {
        BGOUTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<VREF_SPEC, 1> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgouten(&mut self) -> BGOUTEN_W<VREF_SPEC, 2> {
        BGOUTEN_W::new(self)
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<VREF_SPEC, 16> {
        CALIB_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage References System (VREF) Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref::R`](R) reader structure"]
impl crate::Readable for VREF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vref::W`](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
