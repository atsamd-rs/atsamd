#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<APBCMASK_SPEC>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<APBCMASK_SPEC>;
#[doc = "Field `TCC2_` reader - TCC2 APB Clock Enable"]
pub type TCC2__R = crate::BitReader;
#[doc = "Field `TCC2_` writer - TCC2 APB Clock Enable"]
pub type TCC2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC3_` reader - TCC3 APB Clock Enable"]
pub type TCC3__R = crate::BitReader;
#[doc = "Field `TCC3_` writer - TCC3 APB Clock Enable"]
pub type TCC3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub type TC4__R = crate::BitReader;
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub type TC4__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC5_` reader - TC5 APB Clock Enable"]
pub type TC5__R = crate::BitReader;
#[doc = "Field `TC5_` writer - TC5 APB Clock Enable"]
pub type TC5__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDEC_` reader - PDEC APB Clock Enable"]
pub type PDEC__R = crate::BitReader;
#[doc = "Field `PDEC_` writer - PDEC APB Clock Enable"]
pub type PDEC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AES_` reader - AES APB Clock Enable"]
pub type AES__R = crate::BitReader;
#[doc = "Field `AES_` writer - AES APB Clock Enable"]
pub type AES__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type TRNG__R = crate::BitReader;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type TRNG__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICM_` reader - ICM APB Clock Enable"]
pub type ICM__R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM APB Clock Enable"]
pub type ICM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI_` reader - QSPI APB Clock Enable"]
pub type QSPI__R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI APB Clock Enable"]
pub type QSPI__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type CCL__R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type CCL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> TCC2__W<APBCMASK_SPEC, 3> {
        TCC2__W::new(self)
    }
    #[doc = "Bit 4 - TCC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc3_(&mut self) -> TCC3__W<APBCMASK_SPEC, 4> {
        TCC3__W::new(self)
    }
    #[doc = "Bit 5 - TC4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc4_(&mut self) -> TC4__W<APBCMASK_SPEC, 5> {
        TC4__W::new(self)
    }
    #[doc = "Bit 6 - TC5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc5_(&mut self) -> TC5__W<APBCMASK_SPEC, 6> {
        TC5__W::new(self)
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdec_(&mut self) -> PDEC__W<APBCMASK_SPEC, 7> {
        PDEC__W::new(self)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<APBCMASK_SPEC, 8> {
        AC__W::new(self)
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> AES__W<APBCMASK_SPEC, 9> {
        AES__W::new(self)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<APBCMASK_SPEC, 10> {
        TRNG__W::new(self)
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icm_(&mut self) -> ICM__W<APBCMASK_SPEC, 11> {
        ICM__W::new(self)
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> QSPI__W<APBCMASK_SPEC, 13> {
        QSPI__W::new(self)
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> CCL__W<APBCMASK_SPEC, 14> {
        CCL__W::new(self)
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
#[doc = "APBC Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcmask::R`](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbcmask::W`](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0x2000"]
impl crate::Resettable for APBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
