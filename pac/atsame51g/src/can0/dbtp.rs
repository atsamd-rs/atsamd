#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DbtpSpec>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DbtpSpec>;
#[doc = "Field `DSJW` reader - Data (Re)Synchronization Jump Width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DSJW` writer - Data (Re)Synchronization Jump Width"]
pub type DsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point"]
pub type Dtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point"]
pub type Dtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data Baud Rate Prescaler"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data Baud Rate Prescaler"]
pub type DbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - Tranceiver Delay Compensation"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TDC` writer - Tranceiver Delay Compensation"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Tranceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DsjwW<DbtpSpec> {
        DsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> Dtseg2W<DbtpSpec> {
        Dtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> Dtseg1W<DbtpSpec> {
        Dtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DbrpW<DbtpSpec> {
        DbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - Tranceiver Delay Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<DbtpSpec> {
        TdcW::new(self, 23)
    }
}
#[doc = "Fast Bit Timing and Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbtpSpec;
impl crate::RegisterSpec for DbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DbtpSpec {
    const RESET_VALUE: u32 = 0x0a33;
}
