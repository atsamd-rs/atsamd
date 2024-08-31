#[doc = "Register `VREF` reader"]
pub type R = crate::R<VrefSpec>;
#[doc = "Register `VREF` writer"]
pub type W = crate::W<VrefSpec>;
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGOUTEN` reader - Bandgap Output Enable"]
pub type BgoutenR = crate::BitReader;
#[doc = "Field `BGOUTEN` writer - Bandgap Output Enable"]
pub type BgoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALIB` reader - Bandgap Voltage Generator Calibration"]
pub type CalibR = crate::FieldReader<u16>;
#[doc = "Field `CALIB` writer - Bandgap Voltage Generator Calibration"]
pub type CalibW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    pub fn bgouten(&self) -> BgoutenR {
        BgoutenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CalibR {
        CalibR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<VrefSpec> {
        TsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgouten(&mut self) -> BgoutenW<VrefSpec> {
        BgoutenW::new(self, 2)
    }
    #[doc = "Bits 16:26 - Bandgap Voltage Generator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CalibW<VrefSpec> {
        CalibW::new(self, 16)
    }
}
#[doc = "Voltage References System (VREF) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefSpec;
impl crate::RegisterSpec for VrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref::R`](R) reader structure"]
impl crate::Readable for VrefSpec {}
#[doc = "`write(|w| ..)` method takes [`vref::W`](W) writer structure"]
impl crate::Writable for VrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VrefSpec {
    const RESET_VALUE: u32 = 0;
}
