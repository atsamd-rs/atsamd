#[doc = "Register `AHBMASK` reader"]
pub type R = crate::R<AhbmaskSpec>;
#[doc = "Register `AHBMASK` writer"]
pub type W = crate::W<AhbmaskSpec>;
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
pub type Hpb0_R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
pub type Hpb0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
pub type Hpb1_R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
pub type Hpb1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
pub type Hpb2_R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
pub type Hpb2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB3_` reader - HPB3 AHB Clock Mask"]
pub type Hpb3_R = crate::BitReader;
#[doc = "Field `HPB3_` writer - HPB3 AHB Clock Mask"]
pub type Hpb3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIX_` reader - HMATRIX AHB Clock Mask"]
pub type Hmatrix_R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX AHB Clock Mask"]
pub type Hmatrix_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub type Hsram_R = crate::BitReader;
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub type Hsram_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCC_` reader - CMCC AHB Clock Mask"]
pub type Cmcc_R = crate::BitReader;
#[doc = "Field `CMCC_` writer - CMCC AHB Clock Mask"]
pub type Cmcc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type Dmac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub type Usb_R = crate::BitReader;
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub type Usb_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKUPRAM_` reader - BKUPRAM AHB Clock Mask"]
pub type Bkupram_R = crate::BitReader;
#[doc = "Field `BKUPRAM_` writer - BKUPRAM AHB Clock Mask"]
pub type Bkupram_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type Pac_R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type Pac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_` reader - QSPI AHB Clock Mask"]
pub type Qspi_R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI AHB Clock Mask"]
pub type Qspi_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDHC0_` reader - SDHC0 AHB Clock Mask"]
pub type Sdhc0_R = crate::BitReader;
#[doc = "Field `SDHC0_` writer - SDHC0 AHB Clock Mask"]
pub type Sdhc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDHC1_` reader - SDHC1 AHB Clock Mask"]
pub type Sdhc1_R = crate::BitReader;
#[doc = "Field `SDHC1_` writer - SDHC1 AHB Clock Mask"]
pub type Sdhc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_` reader - ICM AHB Clock Mask"]
pub type Icm_R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM AHB Clock Mask"]
pub type Icm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUKCC_` reader - PUKCC AHB Clock Mask"]
pub type Pukcc_R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC AHB Clock Mask"]
pub type Pukcc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_2X_` reader - QSPI_2X AHB Clock Mask"]
pub type Qspi2x_R = crate::BitReader;
#[doc = "Field `QSPI_2X_` writer - QSPI_2X AHB Clock Mask"]
pub type Qspi2x_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_SMEEPROM_` reader - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub type NvmctrlSmeeprom_R = crate::BitReader;
#[doc = "Field `NVMCTRL_SMEEPROM_` writer - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub type NvmctrlSmeeprom_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_CACHE_` reader - NVMCTRL_CACHE AHB Clock Mask"]
pub type NvmctrlCache_R = crate::BitReader;
#[doc = "Field `NVMCTRL_CACHE_` writer - NVMCTRL_CACHE AHB Clock Mask"]
pub type NvmctrlCache_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> Hpb0_R {
        Hpb0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> Hpb1_R {
        Hpb1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> Hpb2_R {
        Hpb2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&self) -> Hpb3_R {
        Hpb3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> Hmatrix_R {
        Hmatrix_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> Hsram_R {
        Hsram_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    pub fn cmcc_(&self) -> Cmcc_R {
        Cmcc_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&self) -> Usb_R {
        Usb_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn bkupram_(&self) -> Bkupram_R {
        Bkupram_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> Pac_R {
        Pac_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_(&self) -> Qspi_R {
        Qspi_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> Sdhc0_R {
        Sdhc0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDHC1 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc1_(&self) -> Sdhc1_R {
        Sdhc1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    pub fn icm_(&self) -> Icm_R {
        Icm_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    pub fn pukcc_(&self) -> Pukcc_R {
        Pukcc_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_2x_(&self) -> Qspi2x_R {
        Qspi2x_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_smeeprom_(&self) -> NvmctrlSmeeprom_R {
        NvmctrlSmeeprom_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_cache_(&self) -> NvmctrlCache_R {
        NvmctrlCache_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> Hpb0_W<AhbmaskSpec> {
        Hpb0_W::new(self, 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> Hpb1_W<AhbmaskSpec> {
        Hpb1_W::new(self, 1)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> Hpb2_W<AhbmaskSpec> {
        Hpb2_W::new(self, 2)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> Hpb3_W<AhbmaskSpec> {
        Hpb3_W::new(self, 3)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> Dsu_W<AhbmaskSpec> {
        Dsu_W::new(self, 4)
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> Hmatrix_W<AhbmaskSpec> {
        Hmatrix_W::new(self, 5)
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<AhbmaskSpec> {
        Nvmctrl_W::new(self, 6)
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_(&mut self) -> Hsram_W<AhbmaskSpec> {
        Hsram_W::new(self, 7)
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmcc_(&mut self) -> Cmcc_W<AhbmaskSpec> {
        Cmcc_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> Dmac_W<AhbmaskSpec> {
        Dmac_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> Usb_W<AhbmaskSpec> {
        Usb_W::new(self, 10)
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bkupram_(&mut self) -> Bkupram_W<AhbmaskSpec> {
        Bkupram_W::new(self, 11)
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> Pac_W<AhbmaskSpec> {
        Pac_W::new(self, 12)
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> Qspi_W<AhbmaskSpec> {
        Qspi_W::new(self, 13)
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc0_(&mut self) -> Sdhc0_W<AhbmaskSpec> {
        Sdhc0_W::new(self, 15)
    }
    #[doc = "Bit 16 - SDHC1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc1_(&mut self) -> Sdhc1_W<AhbmaskSpec> {
        Sdhc1_W::new(self, 16)
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn icm_(&mut self) -> Icm_W<AhbmaskSpec> {
        Icm_W::new(self, 19)
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> Pukcc_W<AhbmaskSpec> {
        Pukcc_W::new(self, 20)
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_2x_(&mut self) -> Qspi2x_W<AhbmaskSpec> {
        Qspi2x_W::new(self, 21)
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_smeeprom_(&mut self) -> NvmctrlSmeeprom_W<AhbmaskSpec> {
        NvmctrlSmeeprom_W::new(self, 22)
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_cache_(&mut self) -> NvmctrlCache_W<AhbmaskSpec> {
        NvmctrlCache_W::new(self, 23)
    }
}
#[doc = "AHB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmaskSpec;
impl crate::RegisterSpec for AhbmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmask::R`](R) reader structure"]
impl crate::Readable for AhbmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmask::W`](W) writer structure"]
impl crate::Writable for AhbmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMASK to value 0x00ff_ffff"]
impl crate::Resettable for AhbmaskSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
