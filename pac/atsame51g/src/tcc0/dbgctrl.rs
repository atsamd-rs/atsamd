#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "Field `DBGRUN` reader - Debug Running Mode"]
pub type DbgrunR = crate::BitReader;
#[doc = "Field `DBGRUN` writer - Debug Running Mode"]
pub type DbgrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDDBD` reader - Fault Detection on Debug Break Detection"]
pub type FddbdR = crate::BitReader;
#[doc = "Field `FDDBD` writer - Fault Detection on Debug Break Detection"]
pub type FddbdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Running Mode"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DbgrunR {
        DbgrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Detection on Debug Break Detection"]
    #[inline(always)]
    pub fn fddbd(&self) -> FddbdR {
        FddbdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Running Mode"]
    #[inline(always)]
    pub fn dbgrun(&mut self) -> DbgrunW<DbgctrlSpec> {
        DbgrunW::new(self, 0)
    }
    #[doc = "Bit 2 - Fault Detection on Debug Break Detection"]
    #[inline(always)]
    pub fn fddbd(&mut self) -> FddbdW<DbgctrlSpec> {
        FddbdW::new(self, 2)
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
