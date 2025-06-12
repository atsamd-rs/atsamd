#[doc = "Register `DFLLSYNC` reader"]
pub type R = crate::R<DfllsyncSpec>;
#[doc = "Register `DFLLSYNC` writer"]
pub type W = crate::W<DfllsyncSpec>;
#[doc = "Field `READREQ` writer - Read Request"]
pub type ReadreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 7 - Read Request"]
    #[inline(always)]
    pub fn readreq(&mut self) -> ReadreqW<DfllsyncSpec> {
        ReadreqW::new(self, 7)
    }
}
#[doc = "DFLL48M Synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllsyncSpec;
impl crate::RegisterSpec for DfllsyncSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dfllsync::R`](R) reader structure"]
impl crate::Readable for DfllsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllsync::W`](W) writer structure"]
impl crate::Writable for DfllsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLSYNC to value 0"]
impl crate::Resettable for DfllsyncSpec {}
