#[doc = "Register `PINCFG0_%s` reader"]
pub type R = crate::R<Pincfg0_Spec>;
#[doc = "Register `PINCFG0_%s` writer"]
pub type W = crate::W<Pincfg0_Spec>;
#[doc = "Field `PMUXEN` reader - Peripheral Multiplexer Enable"]
pub type PmuxenR = crate::BitReader;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN` reader - Input Enable"]
pub type InenR = crate::BitReader;
#[doc = "Field `INEN` writer - Input Enable"]
pub type InenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLEN` reader - Pull Enable"]
pub type PullenR = crate::BitReader;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PullenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRVSTR` reader - Output Driver Strength Selection"]
pub type DrvstrR = crate::BitReader;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DrvstrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    pub fn pmuxen(&self) -> PmuxenR {
        PmuxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    pub fn inen(&self) -> InenR {
        InenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    pub fn pullen(&self) -> PullenR {
        PullenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    pub fn drvstr(&self) -> DrvstrR {
        DrvstrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxen(&mut self) -> PmuxenW<Pincfg0_Spec> {
        PmuxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> InenW<Pincfg0_Spec> {
        InenW::new(self, 1)
    }
    #[doc = "Bit 2 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pullen(&mut self) -> PullenW<Pincfg0_Spec> {
        PullenW::new(self, 2)
    }
    #[doc = "Bit 6 - Output Driver Strength Selection"]
    #[inline(always)]
    #[must_use]
    pub fn drvstr(&mut self) -> DrvstrW<Pincfg0_Spec> {
        DrvstrW::new(self, 6)
    }
}
#[doc = "Pin Configuration n - Group 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg0_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg0_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pincfg0_Spec;
impl crate::RegisterSpec for Pincfg0_Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pincfg0_::R`](R) reader structure"]
impl crate::Readable for Pincfg0_Spec {}
#[doc = "`write(|w| ..)` method takes [`pincfg0_::W`](W) writer structure"]
impl crate::Writable for Pincfg0_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PINCFG0_%s to value 0"]
impl crate::Resettable for Pincfg0_Spec {
    const RESET_VALUE: u8 = 0;
}
