#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKUPCLK` reader - Backup Clock Select"]
pub type BkupclkR = crate::BitReader;
#[doc = "Field `BKUPCLK` writer - Backup Clock Select"]
pub type BkupclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup Clock Select"]
    #[inline(always)]
    pub fn bkupclk(&self) -> BkupclkR {
        BkupclkR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CfdenW<CtrlSpec> {
        CfdenW::new(self, 2)
    }
    #[doc = "Bit 4 - Backup Clock Select"]
    #[inline(always)]
    pub fn bkupclk(&mut self) -> BkupclkW<CtrlSpec> {
        BkupclkW::new(self, 4)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
