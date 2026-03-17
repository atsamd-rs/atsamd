#[doc = "Register `INTFLAGAHB` reader"]
pub type R = crate::R<IntflagahbSpec>;
#[doc = "Register `INTFLAGAHB` writer"]
pub type W = crate::W<IntflagahbSpec>;
#[doc = "Field `FLASH_` reader - FLASH"]
pub type Flash_R = crate::BitReader;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type Flash_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_ALT_` reader - FLASH_ALT"]
pub type FlashAlt_R = crate::BitReader;
#[doc = "Field `FLASH_ALT_` writer - FLASH_ALT"]
pub type FlashAlt_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEPROM_` reader - SEEPROM"]
pub type Seeprom_R = crate::BitReader;
#[doc = "Field `SEEPROM_` writer - SEEPROM"]
pub type Seeprom_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCM4S_` reader - RAMCM4S"]
pub type Ramcm4s_R = crate::BitReader;
#[doc = "Field `RAMCM4S_` writer - RAMCM4S"]
pub type Ramcm4s_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMPPPDSU_` reader - RAMPPPDSU"]
pub type Rampppdsu_R = crate::BitReader;
#[doc = "Field `RAMPPPDSU_` writer - RAMPPPDSU"]
pub type Rampppdsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMDMAWR_` reader - RAMDMAWR"]
pub type Ramdmawr_R = crate::BitReader;
#[doc = "Field `RAMDMAWR_` writer - RAMDMAWR"]
pub type Ramdmawr_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMDMACICM_` reader - RAMDMACICM"]
pub type Ramdmacicm_R = crate::BitReader;
#[doc = "Field `RAMDMACICM_` writer - RAMDMACICM"]
pub type Ramdmacicm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB0_` reader - HPB0"]
pub type Hpb0_R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0"]
pub type Hpb0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB1_` reader - HPB1"]
pub type Hpb1_R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1"]
pub type Hpb1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB2_` reader - HPB2"]
pub type Hpb2_R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2"]
pub type Hpb2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB3_` reader - HPB3"]
pub type Hpb3_R = crate::BitReader;
#[doc = "Field `HPB3_` writer - HPB3"]
pub type Hpb3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUKCC_` reader - PUKCC"]
pub type Pukcc_R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC"]
pub type Pukcc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDHC0_` reader - SDHC0"]
pub type Sdhc0_R = crate::BitReader;
#[doc = "Field `SDHC0_` writer - SDHC0"]
pub type Sdhc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDHC1_` reader - SDHC1"]
pub type Sdhc1_R = crate::BitReader;
#[doc = "Field `SDHC1_` writer - SDHC1"]
pub type Sdhc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_` reader - QSPI"]
pub type Qspi_R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI"]
pub type Qspi_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> Flash_R {
        Flash_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    pub fn flash_alt_(&self) -> FlashAlt_R {
        FlashAlt_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    pub fn seeprom_(&self) -> Seeprom_R {
        Seeprom_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    pub fn ramcm4s_(&self) -> Ramcm4s_R {
        Ramcm4s_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    pub fn rampppdsu_(&self) -> Rampppdsu_R {
        Rampppdsu_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    pub fn ramdmawr_(&self) -> Ramdmawr_R {
        Ramdmawr_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    pub fn ramdmacicm_(&self) -> Ramdmacicm_R {
        Ramdmacicm_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> Hpb0_R {
        Hpb0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> Hpb1_R {
        Hpb1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> Hpb2_R {
        Hpb2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&self) -> Hpb3_R {
        Hpb3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> Pukcc_R {
        Pukcc_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> Sdhc0_R {
        Sdhc0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    pub fn sdhc1_(&self) -> Sdhc1_R {
        Sdhc1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> Qspi_R {
        Qspi_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash_(&mut self) -> Flash_W<IntflagahbSpec> {
        Flash_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    #[must_use]
    pub fn flash_alt_(&mut self) -> FlashAlt_W<IntflagahbSpec> {
        FlashAlt_W::new(self, 1)
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    #[must_use]
    pub fn seeprom_(&mut self) -> Seeprom_W<IntflagahbSpec> {
        Seeprom_W::new(self, 2)
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    #[must_use]
    pub fn ramcm4s_(&mut self) -> Ramcm4s_W<IntflagahbSpec> {
        Ramcm4s_W::new(self, 3)
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    #[must_use]
    pub fn rampppdsu_(&mut self) -> Rampppdsu_W<IntflagahbSpec> {
        Rampppdsu_W::new(self, 4)
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    #[must_use]
    pub fn ramdmawr_(&mut self) -> Ramdmawr_W<IntflagahbSpec> {
        Ramdmawr_W::new(self, 5)
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    #[must_use]
    pub fn ramdmacicm_(&mut self) -> Ramdmacicm_W<IntflagahbSpec> {
        Ramdmacicm_W::new(self, 6)
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> Hpb0_W<IntflagahbSpec> {
        Hpb0_W::new(self, 7)
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> Hpb1_W<IntflagahbSpec> {
        Hpb1_W::new(self, 8)
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> Hpb2_W<IntflagahbSpec> {
        Hpb2_W::new(self, 9)
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> Hpb3_W<IntflagahbSpec> {
        Hpb3_W::new(self, 10)
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> Pukcc_W<IntflagahbSpec> {
        Pukcc_W::new(self, 11)
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc0_(&mut self) -> Sdhc0_W<IntflagahbSpec> {
        Sdhc0_W::new(self, 12)
    }
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc1_(&mut self) -> Sdhc1_W<IntflagahbSpec> {
        Sdhc1_W::new(self, 13)
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> Qspi_W<IntflagahbSpec> {
        Qspi_W::new(self, 14)
    }
}
#[doc = "Bridge interrupt flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagahbSpec;
impl crate::RegisterSpec for IntflagahbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagahb::R`](R) reader structure"]
impl crate::Readable for IntflagahbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagahb::W`](W) writer structure"]
impl crate::Writable for IntflagahbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for IntflagahbSpec {
    const RESET_VALUE: u32 = 0;
}
