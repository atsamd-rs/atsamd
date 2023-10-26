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
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type DSU__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type DMAC__R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type DMAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub type USB__R = crate::BitReader;
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub type USB__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<AHBMASK_SPEC, 3> {
        DSU__W::new(self)
    }
    #[doc = "Bit 4 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<AHBMASK_SPEC, 4> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 5 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<AHBMASK_SPEC, 5> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 6 - USB AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<AHBMASK_SPEC, 6> {
        USB__W::new(self)
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
#[doc = "`reset()` method sets AHBMASK to value 0x7f"]
impl crate::Resettable for AHBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
