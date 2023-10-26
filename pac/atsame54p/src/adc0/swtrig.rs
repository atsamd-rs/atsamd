#[doc = "Register `SWTRIG` reader"]
pub type R = crate::R<SWTRIG_SPEC>;
#[doc = "Register `SWTRIG` writer"]
pub type W = crate::W<SWTRIG_SPEC>;
#[doc = "Field `FLUSH` reader - ADC Conversion Flush"]
pub type FLUSH_R = crate::BitReader;
#[doc = "Field `FLUSH` writer - ADC Conversion Flush"]
pub type FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START` reader - Start ADC Conversion"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start ADC Conversion"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ADC Conversion Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Conversion Flush"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<SWTRIG_SPEC, 0> {
        FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<SWTRIG_SPEC, 1> {
        START_W::new(self)
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
#[doc = "Software Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swtrig::R`](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swtrig::W`](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
