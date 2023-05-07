#[doc = "Register `AHBMASK` reader"]
pub struct R(crate::R<AHBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMASK` writer"]
pub struct W(crate::W<AHBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMASK_SPEC>;
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
impl From<crate::W<AHBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
pub type HPB0__R = crate::BitReader<bool>;
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
pub type HPB0__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
pub type HPB1__R = crate::BitReader<bool>;
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
pub type HPB1__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
pub type HPB2__R = crate::BitReader<bool>;
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
pub type HPB2__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type DSU__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type DMAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub type USB__R = crate::BitReader<bool>;
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub type USB__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
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
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<0> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<1> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<2> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<3> {
        DSU__W::new(self)
    }
    #[doc = "Bit 4 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<4> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 5 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<5> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 6 - USB AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<6> {
        USB__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmask](index.html) module"]
pub struct AHBMASK_SPEC;
impl crate::RegisterSpec for AHBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmask::R](R) reader structure"]
impl crate::Readable for AHBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmask::W](W) writer structure"]
impl crate::Writable for AHBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBMASK to value 0x7f"]
impl crate::Resettable for AHBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
