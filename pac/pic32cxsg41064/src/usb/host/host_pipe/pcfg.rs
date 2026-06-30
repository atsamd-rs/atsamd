#[doc = "Register `PCFG` reader"]
pub type R = crate::R<PcfgSpec>;
#[doc = "Register `PCFG` writer"]
pub type W = crate::W<PcfgSpec>;
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PtokenR = crate::FieldReader;
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PtokenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BK` reader - Pipe Bank"]
pub type BkR = crate::BitReader;
#[doc = "Field `BK` writer - Pipe Bank"]
pub type BkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PtypeR = crate::FieldReader;
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PtypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PtokenR {
        PtokenR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&self) -> BkR {
        BkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PtypeR {
        PtypeR::new((self.bits >> 3) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PtokenW<PcfgSpec> {
        PtokenW::new(self, 0)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    #[must_use]
    pub fn bk(&mut self) -> BkW<PcfgSpec> {
        BkW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PtypeW<PcfgSpec> {
        PtypeW::new(self, 3)
    }
}
#[doc = "HOST_PIPE End Point Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcfgSpec;
impl crate::RegisterSpec for PcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcfg::R`](R) reader structure"]
impl crate::Readable for PcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pcfg::W`](W) writer structure"]
impl crate::Writable for PcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PCFG to value 0"]
impl crate::Resettable for PcfgSpec {
    const RESET_VALUE: u8 = 0;
}
