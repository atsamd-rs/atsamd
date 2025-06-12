#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<ApbcmaskSpec>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<ApbcmaskSpec>;
#[doc = "Field `TCC2_` reader - TCC2 APB Clock Enable"]
pub type Tcc2_R = crate::BitReader;
#[doc = "Field `TCC2_` writer - TCC2 APB Clock Enable"]
pub type Tcc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEC_` reader - PDEC APB Clock Enable"]
pub type Pdec_R = crate::BitReader;
#[doc = "Field `PDEC_` writer - PDEC APB Clock Enable"]
pub type Pdec_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type Ac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_` reader - AES APB Clock Enable"]
pub type Aes_R = crate::BitReader;
#[doc = "Field `AES_` writer - AES APB Clock Enable"]
pub type Aes_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type Trng_R = crate::BitReader;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type Trng_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_` reader - ICM APB Clock Enable"]
pub type Icm_R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM APB Clock Enable"]
pub type Icm_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPI_` reader - QSPI APB Clock Enable"]
pub type Qspi_R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI APB Clock Enable"]
pub type Qspi_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type Ccl_R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type Ccl_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> Tcc2_R {
        Tcc2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> Pdec_R {
        Pdec_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> Aes_R {
        Aes_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> Trng_R {
        Trng_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> Icm_R {
        Icm_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> Qspi_R {
        Qspi_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&mut self) -> Tcc2_W<ApbcmaskSpec> {
        Tcc2_W::new(self, 3)
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    pub fn pdec_(&mut self) -> Pdec_W<ApbcmaskSpec> {
        Pdec_W::new(self, 7)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&mut self) -> Ac_W<ApbcmaskSpec> {
        Ac_W::new(self, 8)
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&mut self) -> Aes_W<ApbcmaskSpec> {
        Aes_W::new(self, 9)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&mut self) -> Trng_W<ApbcmaskSpec> {
        Trng_W::new(self, 10)
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    pub fn icm_(&mut self) -> Icm_W<ApbcmaskSpec> {
        Icm_W::new(self, 11)
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    pub fn qspi_(&mut self) -> Qspi_W<ApbcmaskSpec> {
        Qspi_W::new(self, 13)
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> Ccl_W<ApbcmaskSpec> {
        Ccl_W::new(self, 14)
    }
}
#[doc = "APBC Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcmaskSpec;
impl crate::RegisterSpec for ApbcmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcmask::R`](R) reader structure"]
impl crate::Readable for ApbcmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcmask::W`](W) writer structure"]
impl crate::Writable for ApbcmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBCMASK to value 0x2000"]
impl crate::Resettable for ApbcmaskSpec {
    const RESET_VALUE: u32 = 0x2000;
}
