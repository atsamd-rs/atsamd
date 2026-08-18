#[doc = "Register `STATUSB` reader"]
pub struct R(crate::R<STATUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_` reader - USB APB Protect Enable"]
pub type USB__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` reader - DSU APB Protect Enable"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Protect Enable"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `PORT_` reader - PORT APB Protect Enable"]
pub type PORT__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `MTB_` reader - MTB APB Protect Enable"]
pub type MTB__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USB APB Protect Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTB APB Protect Enable"]
    #[inline(always)]
    pub fn mtb_(&self) -> MTB__R {
        MTB__R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](index.html) module"]
pub struct STATUSB_SPEC;
impl crate::RegisterSpec for STATUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusb::R](R) reader structure"]
impl crate::Readable for STATUSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSB to value 0x02"]
impl crate::Resettable for STATUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
