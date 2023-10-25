#[doc = "Register `INTFLAGC` reader"]
pub type R = crate::R<INTFLAGC_SPEC>;
#[doc = "Register `INTFLAGC` writer"]
pub type W = crate::W<INTFLAGC_SPEC>;
#[doc = "Field `CAN0_` reader - CAN0"]
pub type CAN0__R = crate::BitReader;
#[doc = "Field `CAN0_` writer - CAN0"]
pub type CAN0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1_` reader - CAN1"]
pub type CAN1__R = crate::BitReader;
#[doc = "Field `CAN1_` writer - CAN1"]
pub type CAN1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC2_` reader - TCC2"]
pub type TCC2__R = crate::BitReader;
#[doc = "Field `TCC2_` writer - TCC2"]
pub type TCC2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC3_` reader - TCC3"]
pub type TCC3__R = crate::BitReader;
#[doc = "Field `TCC3_` writer - TCC3"]
pub type TCC3__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC4_` reader - TC4"]
pub type TC4__R = crate::BitReader;
#[doc = "Field `TC4_` writer - TC4"]
pub type TC4__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC5_` reader - TC5"]
pub type TC5__R = crate::BitReader;
#[doc = "Field `TC5_` writer - TC5"]
pub type TC5__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDEC_` reader - PDEC"]
pub type PDEC__R = crate::BitReader;
#[doc = "Field `PDEC_` writer - PDEC"]
pub type PDEC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AC_` reader - AC"]
pub type AC__R = crate::BitReader;
#[doc = "Field `AC_` writer - AC"]
pub type AC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AES_` reader - AES"]
pub type AES__R = crate::BitReader;
#[doc = "Field `AES_` writer - AES"]
pub type AES__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRNG_` reader - TRNG"]
pub type TRNG__R = crate::BitReader;
#[doc = "Field `TRNG_` writer - TRNG"]
pub type TRNG__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICM_` reader - ICM"]
pub type ICM__R = crate::BitReader;
#[doc = "Field `ICM_` writer - ICM"]
pub type ICM__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PUKCC_` reader - PUKCC"]
pub type PUKCC__R = crate::BitReader;
#[doc = "Field `PUKCC_` writer - PUKCC"]
pub type PUKCC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI_` reader - QSPI"]
pub type QSPI__R = crate::BitReader;
#[doc = "Field `QSPI_` writer - QSPI"]
pub type QSPI__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCL_` reader - CCL"]
pub type CCL__R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL"]
pub type CCL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAN0"]
    #[inline(always)]
    pub fn can0_(&self) -> CAN0__R {
        CAN0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN1"]
    #[inline(always)]
    pub fn can1_(&self) -> CAN1__R {
        CAN1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC4"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC5"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    pub fn pukcc_(&self) -> PUKCC__R {
        PUKCC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN0"]
    #[inline(always)]
    #[must_use]
    pub fn can0_(&mut self) -> CAN0__W<INTFLAGC_SPEC, 0> {
        CAN0__W::new(self)
    }
    #[doc = "Bit 1 - CAN1"]
    #[inline(always)]
    #[must_use]
    pub fn can1_(&mut self) -> CAN1__W<INTFLAGC_SPEC, 1> {
        CAN1__W::new(self)
    }
    #[doc = "Bit 3 - TCC2"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> TCC2__W<INTFLAGC_SPEC, 3> {
        TCC2__W::new(self)
    }
    #[doc = "Bit 4 - TCC3"]
    #[inline(always)]
    #[must_use]
    pub fn tcc3_(&mut self) -> TCC3__W<INTFLAGC_SPEC, 4> {
        TCC3__W::new(self)
    }
    #[doc = "Bit 5 - TC4"]
    #[inline(always)]
    #[must_use]
    pub fn tc4_(&mut self) -> TC4__W<INTFLAGC_SPEC, 5> {
        TC4__W::new(self)
    }
    #[doc = "Bit 6 - TC5"]
    #[inline(always)]
    #[must_use]
    pub fn tc5_(&mut self) -> TC5__W<INTFLAGC_SPEC, 6> {
        TC5__W::new(self)
    }
    #[doc = "Bit 7 - PDEC"]
    #[inline(always)]
    #[must_use]
    pub fn pdec_(&mut self) -> PDEC__W<INTFLAGC_SPEC, 7> {
        PDEC__W::new(self)
    }
    #[doc = "Bit 8 - AC"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<INTFLAGC_SPEC, 8> {
        AC__W::new(self)
    }
    #[doc = "Bit 9 - AES"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> AES__W<INTFLAGC_SPEC, 9> {
        AES__W::new(self)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<INTFLAGC_SPEC, 10> {
        TRNG__W::new(self)
    }
    #[doc = "Bit 11 - ICM"]
    #[inline(always)]
    #[must_use]
    pub fn icm_(&mut self) -> ICM__W<INTFLAGC_SPEC, 11> {
        ICM__W::new(self)
    }
    #[doc = "Bit 12 - PUKCC"]
    #[inline(always)]
    #[must_use]
    pub fn pukcc_(&mut self) -> PUKCC__W<INTFLAGC_SPEC, 12> {
        PUKCC__W::new(self)
    }
    #[doc = "Bit 13 - QSPI"]
    #[inline(always)]
    #[must_use]
    pub fn qspi_(&mut self) -> QSPI__W<INTFLAGC_SPEC, 13> {
        QSPI__W::new(self)
    }
    #[doc = "Bit 14 - CCL"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> CCL__W<INTFLAGC_SPEC, 14> {
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
#[doc = "Peripheral interrupt flag status - Bridge C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflagc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflagc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGC_SPEC;
impl crate::RegisterSpec for INTFLAGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagc::R`](R) reader structure"]
impl crate::Readable for INTFLAGC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflagc::W`](W) writer structure"]
impl crate::Writable for INTFLAGC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for INTFLAGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
