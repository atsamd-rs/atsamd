#[doc = "Register `CHINTFLAG` reader"]
pub type R = crate::R<ChintflagSpec>;
#[doc = "Register `CHINTFLAG` writer"]
pub type W = crate::W<ChintflagSpec>;
#[doc = "Field `TERR` reader - Channel Transfer Error"]
pub type TerrR = crate::BitReader;
#[doc = "Field `TERR` writer - Channel Transfer Error"]
pub type TerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMPL` reader - Channel Transfer Complete"]
pub type TcmplR = crate::BitReader;
#[doc = "Field `TCMPL` writer - Channel Transfer Complete"]
pub type TcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TcmplR {
        TcmplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TerrW<ChintflagSpec> {
        TerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpl(&mut self) -> TcmplW<ChintflagSpec> {
        TcmplW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SuspW<ChintflagSpec> {
        SuspW::new(self, 2)
    }
}
#[doc = "Channel Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintflagSpec;
impl crate::RegisterSpec for ChintflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chintflag::R`](R) reader structure"]
impl crate::Readable for ChintflagSpec {}
#[doc = "`write(|w| ..)` method takes [`chintflag::W`](W) writer structure"]
impl crate::Writable for ChintflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHINTFLAG to value 0"]
impl crate::Resettable for ChintflagSpec {
    const RESET_VALUE: u8 = 0;
}
