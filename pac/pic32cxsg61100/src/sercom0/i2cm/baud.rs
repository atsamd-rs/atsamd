#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BaudSpec>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BaudSpec>;
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub type BaudR = crate::FieldReader;
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BAUDLOW` reader - Baud Rate Value Low"]
pub type BaudlowR = crate::FieldReader;
#[doc = "Field `BAUDLOW` writer - Baud Rate Value Low"]
pub type BaudlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSBAUD` reader - High Speed Baud Rate Value"]
pub type HsbaudR = crate::FieldReader;
#[doc = "Field `HSBAUD` writer - High Speed Baud Rate Value"]
pub type HsbaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSBAUDLOW` reader - High Speed Baud Rate Value Low"]
pub type HsbaudlowR = crate::FieldReader;
#[doc = "Field `HSBAUDLOW` writer - High Speed Baud Rate Value Low"]
pub type HsbaudlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    pub fn baudlow(&self) -> BaudlowR {
        BaudlowR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    pub fn hsbaud(&self) -> HsbaudR {
        HsbaudR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    pub fn hsbaudlow(&self) -> HsbaudlowR {
        HsbaudlowR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BaudW<BaudSpec> {
        BaudW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Baud Rate Value Low"]
    #[inline(always)]
    #[must_use]
    pub fn baudlow(&mut self) -> BaudlowW<BaudSpec> {
        BaudlowW::new(self, 8)
    }
    #[doc = "Bits 16:23 - High Speed Baud Rate Value"]
    #[inline(always)]
    #[must_use]
    pub fn hsbaud(&mut self) -> HsbaudW<BaudSpec> {
        HsbaudW::new(self, 16)
    }
    #[doc = "Bits 24:31 - High Speed Baud Rate Value Low"]
    #[inline(always)]
    #[must_use]
    pub fn hsbaudlow(&mut self) -> HsbaudlowW<BaudSpec> {
        HsbaudlowW::new(self, 24)
    }
}
#[doc = "I2CM Baud Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudSpec;
impl crate::RegisterSpec for BaudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BaudSpec {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BaudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BaudSpec {
    const RESET_VALUE: u32 = 0;
}
