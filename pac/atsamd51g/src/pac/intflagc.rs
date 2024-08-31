#[doc = "Register `INTFLAGC` reader"]
pub type R = crate::R<IntflagcSpec>;
#[doc = "Register `INTFLAGC` writer"]
pub type W = crate::W<IntflagcSpec>;
#[doc = "Field `TCC2_` reader - TCC2"]
pub type Tcc2_R = crate::BitReader;
#[doc = "Field `TCC2_` writer - TCC2"]
pub type Tcc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEC_` reader - PDEC"]
pub type Pdec_R = crate::BitReader;
#[doc = "Field `PDEC_` writer - PDEC"]
pub type Pdec_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC_` reader - AC"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AC_` writer - AC"]
pub type Ac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_` reader - AES"]
pub type Aes_R = crate::BitReader;
#[doc = "Field `AES_` writer - AES"]
pub type Aes_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_` reader - TRNG"]
pub type Trng_R = crate::BitReader;
#[doc = "Field `TRNG_` writer - TRNG"]
pub type Trng_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_` reader - ICM"]
pub type Icm_R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM"]
pub type Icm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUKCC_` reader - PUKCC"]
pub type Pukcc_R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC"]
pub type Pukcc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_` reader - QSPI"]
pub type Qspi_R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI"]
pub type Qspi_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCL_` reader - CCL"]
pub type Ccl_R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL"]
pub type Ccl_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> Tcc2_R {
        Tcc2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    pub fn pdec_(&self) -> Pdec_R {
        Pdec_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    pub fn aes_(&self) -> Aes_R {
        Aes_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    pub fn trng_(&self) -> Trng_R {
        Trng_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    pub fn icm_(&self) -> Icm_R {
        Icm_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> Pukcc_R {
        Pukcc_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> Qspi_R {
        Qspi_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> Tcc2_W<IntflagcSpec> {
        Tcc2_W::new(self, 3)
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    #[must_use]
    pub fn pdec_(&mut self) -> Pdec_W<IntflagcSpec> {
        Pdec_W::new(self, 7)
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> Ac_W<IntflagcSpec> {
        Ac_W::new(self, 8)
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> Aes_W<IntflagcSpec> {
        Aes_W::new(self, 9)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> Trng_W<IntflagcSpec> {
        Trng_W::new(self, 10)
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    #[must_use]
    pub fn icm_(&mut self) -> Icm_W<IntflagcSpec> {
        Icm_W::new(self, 11)
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> Pukcc_W<IntflagcSpec> {
        Pukcc_W::new(self, 12)
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> Qspi_W<IntflagcSpec> {
        Qspi_W::new(self, 13)
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> Ccl_W<IntflagcSpec> {
        Ccl_W::new(self, 14)
    }
}
#[doc = "Peripheral interrupt flag status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagcSpec;
impl crate::RegisterSpec for IntflagcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagc::R`](R) reader structure"]
impl crate::Readable for IntflagcSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagc::W`](W) writer structure"]
impl crate::Writable for IntflagcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for IntflagcSpec {
    const RESET_VALUE: u32 = 0;
}
