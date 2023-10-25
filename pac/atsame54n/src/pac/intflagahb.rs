#[doc = "Register `INTFLAGAHB` reader"]
pub type R = crate::R<INTFLAGAHB_SPEC>;
#[doc = "Register `INTFLAGAHB` writer"]
pub type W = crate::W<INTFLAGAHB_SPEC>;
#[doc = "Field `FLASH_` reader - FLASH"]
pub type FLASH__R = crate::BitReader;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type FLASH__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_ALT_` reader - FLASH_ALT"]
pub type FLASH_ALT__R = crate::BitReader;
#[doc = "Field `FLASH_ALT_` writer - FLASH_ALT"]
pub type FLASH_ALT__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEEPROM_` reader - SEEPROM"]
pub type SEEPROM__R = crate::BitReader;
#[doc = "Field `SEEPROM_` writer - SEEPROM"]
pub type SEEPROM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMCM4S_` reader - RAMCM4S"]
pub type RAMCM4S__R = crate::BitReader;
#[doc = "Field `RAMCM4S_` writer - RAMCM4S"]
pub type RAMCM4S__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMPPPDSU_` reader - RAMPPPDSU"]
pub type RAMPPPDSU__R = crate::BitReader;
#[doc = "Field `RAMPPPDSU_` writer - RAMPPPDSU"]
pub type RAMPPPDSU__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMDMAWR_` reader - RAMDMAWR"]
pub type RAMDMAWR__R = crate::BitReader;
#[doc = "Field `RAMDMAWR_` writer - RAMDMAWR"]
pub type RAMDMAWR__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMDMACICM_` reader - RAMDMACICM"]
pub type RAMDMACICM__R = crate::BitReader;
#[doc = "Field `RAMDMACICM_` writer - RAMDMACICM"]
pub type RAMDMACICM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB0_` reader - HPB0"]
pub type HPB0__R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0"]
pub type HPB0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB1_` reader - HPB1"]
pub type HPB1__R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1"]
pub type HPB1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB2_` reader - HPB2"]
pub type HPB2__R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2"]
pub type HPB2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPB3_` reader - HPB3"]
pub type HPB3__R = crate::BitReader;
#[doc = "Field `HPB3_` writer - HPB3"]
pub type HPB3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PUKCC_` reader - PUKCC"]
pub type PUKCC__R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC"]
pub type PUKCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDHC0_` reader - SDHC0"]
pub type SDHC0__R = crate::BitReader;
#[doc = "Field `SDHC0_` writer - SDHC0"]
pub type SDHC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDHC1_` reader - SDHC1"]
pub type SDHC1__R = crate::BitReader;
#[doc = "Field `SDHC1_` writer - SDHC1"]
pub type SDHC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI_` reader - QSPI"]
pub type QSPI__R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI"]
pub type QSPI__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKUPRAM_` reader - BKUPRAM"]
pub type BKUPRAM__R = crate::BitReader;
#[doc = "Field `BKUPRAM_` writer - BKUPRAM"]
pub type BKUPRAM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    pub fn flash_alt_(&self) -> FLASH_ALT__R {
        FLASH_ALT__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    pub fn seeprom_(&self) -> SEEPROM__R {
        SEEPROM__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    pub fn ramcm4s_(&self) -> RAMCM4S__R {
        RAMCM4S__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    pub fn rampppdsu_(&self) -> RAMPPPDSU__R {
        RAMPPPDSU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    pub fn ramdmawr_(&self) -> RAMDMAWR__R {
        RAMDMAWR__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    pub fn ramdmacicm_(&self) -> RAMDMACICM__R {
        RAMDMACICM__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    pub fn sdhc0_(&self) -> SDHC0__R {
        SDHC0__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    pub fn sdhc1_(&self) -> SDHC1__R {
        SDHC1__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BKUPRAM"]
    #[inline(always)]
    pub fn bkupram_(&self) -> BKUPRAM__R {
        BKUPRAM__R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash_(&mut self) -> FLASH__W<INTFLAGAHB_SPEC, 0> {
        FLASH__W::new(self)
    }
    #[doc = "Bit 1 - FLASH_ALT"]
    #[inline(always)]
    #[must_use]
    pub fn flash_alt_(&mut self) -> FLASH_ALT__W<INTFLAGAHB_SPEC, 1> {
        FLASH_ALT__W::new(self)
    }
    #[doc = "Bit 2 - SEEPROM"]
    #[inline(always)]
    #[must_use]
    pub fn seeprom_(&mut self) -> SEEPROM__W<INTFLAGAHB_SPEC, 2> {
        SEEPROM__W::new(self)
    }
    #[doc = "Bit 3 - RAMCM4S"]
    #[inline(always)]
    #[must_use]
    pub fn ramcm4s_(&mut self) -> RAMCM4S__W<INTFLAGAHB_SPEC, 3> {
        RAMCM4S__W::new(self)
    }
    #[doc = "Bit 4 - RAMPPPDSU"]
    #[inline(always)]
    #[must_use]
    pub fn rampppdsu_(&mut self) -> RAMPPPDSU__W<INTFLAGAHB_SPEC, 4> {
        RAMPPPDSU__W::new(self)
    }
    #[doc = "Bit 5 - RAMDMAWR"]
    #[inline(always)]
    #[must_use]
    pub fn ramdmawr_(&mut self) -> RAMDMAWR__W<INTFLAGAHB_SPEC, 5> {
        RAMDMAWR__W::new(self)
    }
    #[doc = "Bit 6 - RAMDMACICM"]
    #[inline(always)]
    #[must_use]
    pub fn ramdmacicm_(&mut self) -> RAMDMACICM__W<INTFLAGAHB_SPEC, 6> {
        RAMDMACICM__W::new(self)
    }
    #[doc = "Bit 7 - HPB0"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<INTFLAGAHB_SPEC, 7> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 8 - HPB1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<INTFLAGAHB_SPEC, 8> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 9 - HPB2"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<INTFLAGAHB_SPEC, 9> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 10 - HPB3"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> HPB3__W<INTFLAGAHB_SPEC, 10> {
        HPB3__W::new(self)
    }
    #[doc = "Bit 11 - PUKCC"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> PUKCC__W<INTFLAGAHB_SPEC, 11> {
        PUKCC__W::new(self)
    }
    #[doc = "Bit 12 - SDHC0"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc0_(&mut self) -> SDHC0__W<INTFLAGAHB_SPEC, 12> {
        SDHC0__W::new(self)
    }
    #[doc = "Bit 13 - SDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc1_(&mut self) -> SDHC1__W<INTFLAGAHB_SPEC, 13> {
        SDHC1__W::new(self)
    }
    #[doc = "Bit 14 - QSPI"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> QSPI__W<INTFLAGAHB_SPEC, 14> {
        QSPI__W::new(self)
    }
    #[doc = "Bit 15 - BKUPRAM"]
    #[inline(always)]
    #[must_use]
    pub fn bkupram_(&mut self) -> BKUPRAM__W<INTFLAGAHB_SPEC, 15> {
        BKUPRAM__W::new(self)
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
#[doc = "Bridge interrupt flag status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagahb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagahb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagahb::R`](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflagahb::W`](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
