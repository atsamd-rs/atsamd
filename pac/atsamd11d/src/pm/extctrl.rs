#[doc = "Register `EXTCTRL` reader"]
pub type R = crate::R<ExtctrlSpec>;
#[doc = "Register `EXTCTRL` writer"]
pub type W = crate::W<ExtctrlSpec>;
#[doc = "Field `SETDIS` reader - External Reset Disable"]
pub type SetdisR = crate::BitReader;
#[doc = "Field `SETDIS` writer - External Reset Disable"]
pub type SetdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&self) -> SetdisR {
        SetdisR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&mut self) -> SetdisW<ExtctrlSpec> {
        SetdisW::new(self, 0)
    }
}
#[doc = "External Reset Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`extctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtctrlSpec;
impl crate::RegisterSpec for ExtctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extctrl::R`](R) reader structure"]
impl crate::Readable for ExtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`extctrl::W`](W) writer structure"]
impl crate::Writable for ExtctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTCTRL to value 0"]
impl crate::Resettable for ExtctrlSpec {}
