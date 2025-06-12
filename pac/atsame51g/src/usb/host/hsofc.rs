#[doc = "Register `HSOFC` reader"]
pub type R = crate::R<HsofcSpec>;
#[doc = "Register `HSOFC` writer"]
pub type W = crate::W<HsofcSpec>;
#[doc = "Field `FLENC` reader - Frame Length Control"]
pub type FlencR = crate::FieldReader;
#[doc = "Field `FLENC` writer - Frame Length Control"]
pub type FlencW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLENCE` reader - Frame Length Control Enable"]
pub type FlenceR = crate::BitReader;
#[doc = "Field `FLENCE` writer - Frame Length Control Enable"]
pub type FlenceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&self) -> FlencR {
        FlencR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&self) -> FlenceR {
        FlenceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&mut self) -> FlencW<HsofcSpec> {
        FlencW::new(self, 0)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&mut self) -> FlenceW<HsofcSpec> {
        FlenceW::new(self, 7)
    }
}
#[doc = "HOST Host Start Of Frame Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hsofc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsofc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsofcSpec;
impl crate::RegisterSpec for HsofcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hsofc::R`](R) reader structure"]
impl crate::Readable for HsofcSpec {}
#[doc = "`write(|w| ..)` method takes [`hsofc::W`](W) writer structure"]
impl crate::Writable for HsofcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSOFC to value 0"]
impl crate::Resettable for HsofcSpec {}
