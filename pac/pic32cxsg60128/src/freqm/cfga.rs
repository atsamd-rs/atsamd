#[doc = "Register `CFGA` reader"]
pub type R = crate::R<CfgaSpec>;
#[doc = "Register `CFGA` writer"]
pub type W = crate::W<CfgaSpec>;
#[doc = "Field `REFNUM` reader - Number of Reference Clock Cycles"]
pub type RefnumR = crate::FieldReader;
#[doc = "Field `REFNUM` writer - Number of Reference Clock Cycles"]
pub type RefnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> RefnumR {
        RefnumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Reference Clock Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn refnum(&mut self) -> RefnumW<CfgaSpec> {
        RefnumW::new(self, 0)
    }
}
#[doc = "Config A register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgaSpec;
impl crate::RegisterSpec for CfgaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfga::R`](R) reader structure"]
impl crate::Readable for CfgaSpec {}
#[doc = "`write(|w| ..)` method takes [`cfga::W`](W) writer structure"]
impl crate::Writable for CfgaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CfgaSpec {
    const RESET_VALUE: u16 = 0;
}
