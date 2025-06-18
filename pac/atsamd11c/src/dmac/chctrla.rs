#[doc = "Register `CHCTRLA` reader"]
pub type R = crate::R<ChctrlaSpec>;
#[doc = "Register `CHCTRLA` writer"]
pub type W = crate::W<ChctrlaSpec>;
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<ChctrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<ChctrlaSpec> {
        EnableW::new(self, 1)
    }
}
#[doc = "Channel Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlaSpec;
impl crate::RegisterSpec for ChctrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chctrla::R`](R) reader structure"]
impl crate::Readable for ChctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrla::W`](W) writer structure"]
impl crate::Writable for ChctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for ChctrlaSpec {}
