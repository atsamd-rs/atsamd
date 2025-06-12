#[doc = "Register `RJFML` reader"]
pub type R = crate::R<RjfmlSpec>;
#[doc = "Register `RJFML` writer"]
pub type W = crate::W<RjfmlSpec>;
#[doc = "Field `FML` reader - Frame Max Length"]
pub type FmlR = crate::FieldReader<u16>;
#[doc = "Field `FML` writer - Frame Max Length"]
pub type FmlW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&self) -> FmlR {
        FmlR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Max Length"]
    #[inline(always)]
    pub fn fml(&mut self) -> FmlW<RjfmlSpec> {
        FmlW::new(self, 0)
    }
}
#[doc = "RX Jumbo Frame Max Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rjfml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rjfml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RjfmlSpec;
impl crate::RegisterSpec for RjfmlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rjfml::R`](R) reader structure"]
impl crate::Readable for RjfmlSpec {}
#[doc = "`write(|w| ..)` method takes [`rjfml::W`](W) writer structure"]
impl crate::Writable for RjfmlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RJFML to value 0x3fff"]
impl crate::Resettable for RjfmlSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
