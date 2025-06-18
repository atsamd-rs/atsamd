#[doc = "Register `DADD` reader"]
pub type R = crate::R<DaddSpec>;
#[doc = "Register `DADD` writer"]
pub type W = crate::W<DaddSpec>;
#[doc = "Field `DADD` reader - Device Address"]
pub type DaddR = crate::FieldReader;
#[doc = "Field `DADD` writer - Device Address"]
pub type DaddW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDEN` reader - Device Address Enable"]
pub type AddenR = crate::BitReader;
#[doc = "Field `ADDEN` writer - Device Address Enable"]
pub type AddenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&self) -> DaddR {
        DaddR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> AddenR {
        AddenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&mut self) -> DaddW<DaddSpec> {
        DaddW::new(self, 0)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&mut self) -> AddenW<DaddSpec> {
        AddenW::new(self, 7)
    }
}
#[doc = "DEVICE Device Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddSpec;
impl crate::RegisterSpec for DaddSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dadd::R`](R) reader structure"]
impl crate::Readable for DaddSpec {}
#[doc = "`write(|w| ..)` method takes [`dadd::W`](W) writer structure"]
impl crate::Writable for DaddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADD to value 0"]
impl crate::Resettable for DaddSpec {}
