#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Start Conversion Event Input"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Conversion Event Input"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTYEO` reader - Data Buffer Empty Event Output"]
pub type EmptyeoR = crate::BitReader;
#[doc = "Field `EMPTYEO` writer - Data Buffer Empty Event Output"]
pub type EmptyeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    pub fn emptyeo(&self) -> EmptyeoR {
        EmptyeoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input"]
    #[inline(always)]
    #[must_use]
    pub fn startei(&mut self) -> StarteiW<EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Event Output"]
    #[inline(always)]
    #[must_use]
    pub fn emptyeo(&mut self) -> EmptyeoW<EvctrlSpec> {
        EmptyeoW::new(self, 1)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u8 = 0;
}
