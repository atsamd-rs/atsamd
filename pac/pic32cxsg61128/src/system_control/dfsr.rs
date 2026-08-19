#[doc = "Register `DFSR` reader"]
pub type R = crate::R<DfsrSpec>;
#[doc = "Register `DFSR` writer"]
pub type W = crate::W<DfsrSpec>;
#[doc = "Field `HALTED` reader - "]
pub type HaltedR = crate::BitReader;
#[doc = "Field `HALTED` writer - "]
pub type HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPT` reader - "]
pub type BkptR = crate::BitReader;
#[doc = "Field `BKPT` writer - "]
pub type BkptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWTTRAP` reader - "]
pub type DwttrapR = crate::BitReader;
#[doc = "Field `DWTTRAP` writer - "]
pub type DwttrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCATCH` reader - "]
pub type VcatchR = crate::BitReader;
#[doc = "Field `VCATCH` writer - "]
pub type VcatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTERNAL` reader - "]
pub type ExternalR = crate::BitReader;
#[doc = "Field `EXTERNAL` writer - "]
pub type ExternalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn halted(&self) -> HaltedR {
        HaltedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bkpt(&self) -> BkptR {
        BkptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DwttrapR {
        DwttrapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vcatch(&self) -> VcatchR {
        VcatchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external(&self) -> ExternalR {
        ExternalR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HaltedW<DfsrSpec> {
        HaltedW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BkptW<DfsrSpec> {
        BkptW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DwttrapW<DfsrSpec> {
        DwttrapW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VcatchW<DfsrSpec> {
        VcatchW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> ExternalW<DfsrSpec> {
        ExternalW::new(self, 4)
    }
}
#[doc = "Debug Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsrSpec;
impl crate::RegisterSpec for DfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsr::R`](R) reader structure"]
impl crate::Readable for DfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsr::W`](W) writer structure"]
impl crate::Writable for DfsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DfsrSpec {
    const RESET_VALUE: u32 = 0;
}
