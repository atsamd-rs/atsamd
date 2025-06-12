#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "Field `ECCDIS` reader - Debugger ECC Read Disable"]
pub type EccdisR = crate::BitReader;
#[doc = "Field `ECCDIS` writer - Debugger ECC Read Disable"]
pub type EccdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCELOG` reader - Debugger ECC Error Tracking Mode"]
pub type EccelogR = crate::BitReader;
#[doc = "Field `ECCELOG` writer - Debugger ECC Error Tracking Mode"]
pub type EccelogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debugger ECC Read Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> EccdisR {
        EccdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debugger ECC Error Tracking Mode"]
    #[inline(always)]
    pub fn eccelog(&self) -> EccelogR {
        EccelogR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debugger ECC Read Disable"]
    #[inline(always)]
    pub fn eccdis(&mut self) -> EccdisW<DbgctrlSpec> {
        EccdisW::new(self, 0)
    }
    #[doc = "Bit 1 - Debugger ECC Error Tracking Mode"]
    #[inline(always)]
    pub fn eccelog(&mut self) -> EccelogW<DbgctrlSpec> {
        EccelogW::new(self, 1)
    }
}
#[doc = "Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DbgctrlSpec {}
