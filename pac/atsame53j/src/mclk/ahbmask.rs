#[doc = "Register `AHBMASK` reader"]
pub type R = crate::R<AHBMASK_SPEC>;
#[doc = "Register `AHBMASK` writer"]
pub type W = crate::W<AHBMASK_SPEC>;
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
pub type HPB0__R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
pub type HPB0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
pub type HPB1__R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
pub type HPB1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
pub type HPB2__R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
pub type HPB2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB3_` reader - HPB3 AHB Clock Mask"]
pub type HPB3__R = crate::BitReader;
#[doc = "Field `HPB3_` writer - HPB3 AHB Clock Mask"]
pub type HPB3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type DSU__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HMATRIX_` reader - HMATRIX AHB Clock Mask"]
pub type HMATRIX__R = crate::BitReader;
#[doc = "Field `HMATRIX_` writer - HMATRIX AHB Clock Mask"]
pub type HMATRIX__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub type HSRAM__R = crate::BitReader;
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub type HSRAM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMCC_` reader - CMCC AHB Clock Mask"]
pub type CMCC__R = crate::BitReader;
#[doc = "Field `CMCC_` writer - CMCC AHB Clock Mask"]
pub type CMCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type DMAC__R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type DMAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub type USB__R = crate::BitReader;
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub type USB__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKUPRAM_` reader - BKUPRAM AHB Clock Mask"]
pub type BKUPRAM__R = crate::BitReader;
#[doc = "Field `BKUPRAM_` writer - BKUPRAM AHB Clock Mask"]
pub type BKUPRAM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type PAC__R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type PAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI_` reader - QSPI AHB Clock Mask"]
pub type QSPI__R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI AHB Clock Mask"]
pub type QSPI__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GMAC_` reader - GMAC AHB Clock Mask"]
pub type GMAC__R = crate::BitReader;
#[doc = "Field `GMAC_` writer - GMAC AHB Clock Mask"]
pub type GMAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDHC0_` reader - SDHC0 AHB Clock Mask"]
pub type SDHC0__R = crate::BitReader;
#[doc = "Field `SDHC0_` writer - SDHC0 AHB Clock Mask"]
pub type SDHC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICM_` reader - ICM AHB Clock Mask"]
pub type ICM__R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM AHB Clock Mask"]
pub type ICM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PUKCC_` reader - PUKCC AHB Clock Mask"]
pub type PUKCC__R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC AHB Clock Mask"]
pub type PUKCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI_2X_` reader - QSPI_2X AHB Clock Mask"]
pub type QSPI_2X__R = crate::BitReader;
#[doc = "Field `QSPI_2X_` writer - QSPI_2X AHB Clock Mask"]
pub type QSPI_2X__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_SMEEPROM_` reader - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub type NVMCTRL_SMEEPROM__R = crate::BitReader;
#[doc = "Field `NVMCTRL_SMEEPROM_` writer - NVMCTRL_SMEEPROM AHB Clock Mask"]
pub type NVMCTRL_SMEEPROM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_CACHE_` reader - NVMCTRL_CACHE AHB Clock Mask"]
pub type NVMCTRL_CACHE__R = crate::BitReader;
#[doc = "Field `NVMCTRL_CACHE_` writer - NVMCTRL_CACHE AHB Clock Mask"]
pub type NVMCTRL_CACHE__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> HSRAM__R {
        HSRAM__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    pub fn cmcc_(&self) -> CMCC__R {
        CMCC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn bkupram_(&self) -> BKUPRAM__R {
        BKUPRAM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn gmac_(&self) -> GMAC__R {
        GMAC__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> SDHC0__R {
        SDHC0__R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    pub fn qspi_2x_(&self) -> QSPI_2X__R {
        QSPI_2X__R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_smeeprom_(&self) -> NVMCTRL_SMEEPROM__R {
        NVMCTRL_SMEEPROM__R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_cache_(&self) -> NVMCTRL_CACHE__R {
        NVMCTRL_CACHE__R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<AHBMASK_SPEC, 0> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<AHBMASK_SPEC, 1> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<AHBMASK_SPEC, 2> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> HPB3__W<AHBMASK_SPEC, 3> {
        HPB3__W::new(self)
    }
    #[doc = "Bit 4 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<AHBMASK_SPEC, 4> {
        DSU__W::new(self)
    }
    #[doc = "Bit 5 - HMATRIX AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> HMATRIX__W<AHBMASK_SPEC, 5> {
        HMATRIX__W::new(self)
    }
    #[doc = "Bit 6 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<AHBMASK_SPEC, 6> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 7 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_(&mut self) -> HSRAM__W<AHBMASK_SPEC, 7> {
        HSRAM__W::new(self)
    }
    #[doc = "Bit 8 - CMCC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn cmcc_(&mut self) -> CMCC__W<AHBMASK_SPEC, 8> {
        CMCC__W::new(self)
    }
    #[doc = "Bit 9 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<AHBMASK_SPEC, 9> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 10 - USB AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<AHBMASK_SPEC, 10> {
        USB__W::new(self)
    }
    #[doc = "Bit 11 - BKUPRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bkupram_(&mut self) -> BKUPRAM__W<AHBMASK_SPEC, 11> {
        BKUPRAM__W::new(self)
    }
    #[doc = "Bit 12 - PAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<AHBMASK_SPEC, 12> {
        PAC__W::new(self)
    }
    #[doc = "Bit 13 - QSPI AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> QSPI__W<AHBMASK_SPEC, 13> {
        QSPI__W::new(self)
    }
    #[doc = "Bit 14 - GMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_(&mut self) -> GMAC__W<AHBMASK_SPEC, 14> {
        GMAC__W::new(self)
    }
    #[doc = "Bit 15 - SDHC0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc0_(&mut self) -> SDHC0__W<AHBMASK_SPEC, 15> {
        SDHC0__W::new(self)
    }
    #[doc = "Bit 19 - ICM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn icm_(&mut self) -> ICM__W<AHBMASK_SPEC, 19> {
        ICM__W::new(self)
    }
    #[doc = "Bit 20 - PUKCC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> PUKCC__W<AHBMASK_SPEC, 20> {
        PUKCC__W::new(self)
    }
    #[doc = "Bit 21 - QSPI_2X AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_2x_(&mut self) -> QSPI_2X__W<AHBMASK_SPEC, 21> {
        QSPI_2X__W::new(self)
    }
    #[doc = "Bit 22 - NVMCTRL_SMEEPROM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_smeeprom_(&mut self) -> NVMCTRL_SMEEPROM__W<AHBMASK_SPEC, 22> {
        NVMCTRL_SMEEPROM__W::new(self)
    }
    #[doc = "Bit 23 - NVMCTRL_CACHE AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_cache_(&mut self) -> NVMCTRL_CACHE__W<AHBMASK_SPEC, 23> {
        NVMCTRL_CACHE__W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBMASK_SPEC;
impl crate::RegisterSpec for AHBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmask::R`](R) reader structure"]
impl crate::Readable for AHBMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbmask::W`](W) writer structure"]
impl crate::Writable for AHBMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBMASK to value 0x00ff_ffff"]
impl crate::Resettable for AHBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_ffff;
}
