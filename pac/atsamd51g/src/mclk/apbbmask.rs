#[doc = "Register `APBBMASK` reader"]
pub struct R(crate::R<APBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBBMASK` writer"]
pub struct W(crate::W<APBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_` reader - USB APB Clock Enable"]
pub type USB__R = crate::BitReader<bool>;
#[doc = "Field `USB_` writer - USB APB Clock Enable"]
pub type USB__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub type DSU__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type PORT__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub type HMATRIX__R = crate::BitReader<bool>;
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub type HMATRIX__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type SERCOM3__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type TCC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `TCC1_` reader - TCC1 APB Clock Enable"]
pub type TCC1__R = crate::BitReader<bool>;
#[doc = "Field `TCC1_` writer - TCC1 APB Clock Enable"]
pub type TCC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type TC2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type TC3__R = crate::BitReader<bool>;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type TC3__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
#[doc = "Field `RAMECC_` reader - RAMECC APB Clock Enable"]
pub type RAMECC__R = crate::BitReader<bool>;
#[doc = "Field `RAMECC_` writer - RAMECC APB Clock Enable"]
pub type RAMECC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBBMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<0> {
        USB__W::new(self)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<1> {
        DSU__W::new(self)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<2> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> PORT__W<4> {
        PORT__W::new(self)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> HMATRIX__W<6> {
        HMATRIX__W::new(self)
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<7> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<9> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<10> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<11> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> TCC1__W<12> {
        TCC1__W::new(self)
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<13> {
        TC2__W::new(self)
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> TC3__W<14> {
        TC3__W::new(self)
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramecc_(&mut self) -> RAMECC__W<16> {
        RAMECC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbmask](index.html) module"]
pub struct APBBMASK_SPEC;
impl crate::RegisterSpec for APBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbbmask::R](R) reader structure"]
impl crate::Readable for APBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbbmask::W](W) writer structure"]
impl crate::Writable for APBBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBBMASK to value 0x0001_8056"]
impl crate::Resettable for APBBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8056;
}
