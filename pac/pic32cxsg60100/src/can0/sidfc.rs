#[doc = "Register `SIDFC` reader"]
pub type R = crate::R<SidfcSpec>;
#[doc = "Register `SIDFC` writer"]
pub type W = crate::W<SidfcSpec>;
#[doc = "Field `FLSSA` reader - Filter List Standard Start Address"]
pub type FlssaR = crate::FieldReader<u16>;
#[doc = "Field `FLSSA` writer - Filter List Standard Start Address"]
pub type FlssaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LSS` reader - List Size Standard"]
pub type LssR = crate::FieldReader;
#[doc = "Field `LSS` writer - List Size Standard"]
pub type LssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FlssaR {
        FlssaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LssR {
        LssR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FlssaW<SidfcSpec> {
        FlssaW::new(self, 0)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LssW<SidfcSpec> {
        LssW::new(self, 16)
    }
}
#[doc = "Standard ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidfcSpec;
impl crate::RegisterSpec for SidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidfc::R`](R) reader structure"]
impl crate::Readable for SidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`sidfc::W`](W) writer structure"]
impl crate::Writable for SidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDFC to value 0"]
impl crate::Resettable for SidfcSpec {
    const RESET_VALUE: u32 = 0;
}
