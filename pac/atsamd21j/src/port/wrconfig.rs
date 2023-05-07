#[doc = "Register `WRCONFIG%s` writer"]
pub struct W(crate::W<WRCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRCONFIG_SPEC>;
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
impl From<crate::W<WRCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINMASK` writer - Pin Mask for Multiple Pin Configuration"]
pub type PINMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRCONFIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `PMUXEN` writer - Peripheral Multiplexer Enable"]
pub type PMUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `INEN` writer - Input Enable"]
pub type INEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `PULLEN` writer - Pull Enable"]
pub type PULLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `DRVSTR` writer - Output Driver Strength Selection"]
pub type DRVSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `PMUX` writer - Peripheral Multiplexing"]
pub type PMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRCONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `WRPMUX` writer - Write PMUX"]
pub type WRPMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `WRPINCFG` writer - Write PINCFG"]
pub type WRPINCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
#[doc = "Field `HWSEL` writer - Half-Word Select"]
pub type HWSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRCONFIG_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:15 - Pin Mask for Multiple Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pinmask(&mut self) -> PINMASK_W<0> {
        PINMASK_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuxen(&mut self) -> PMUXEN_W<16> {
        PMUXEN_W::new(self)
    }
    #[doc = "Bit 17 - Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> INEN_W<17> {
        INEN_W::new(self)
    }
    #[doc = "Bit 18 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pullen(&mut self) -> PULLEN_W<18> {
        PULLEN_W::new(self)
    }
    #[doc = "Bit 22 - Output Driver Strength Selection"]
    #[inline(always)]
    #[must_use]
    pub fn drvstr(&mut self) -> DRVSTR_W<22> {
        DRVSTR_W::new(self)
    }
    #[doc = "Bits 24:27 - Peripheral Multiplexing"]
    #[inline(always)]
    #[must_use]
    pub fn pmux(&mut self) -> PMUX_W<24> {
        PMUX_W::new(self)
    }
    #[doc = "Bit 28 - Write PMUX"]
    #[inline(always)]
    #[must_use]
    pub fn wrpmux(&mut self) -> WRPMUX_W<28> {
        WRPMUX_W::new(self)
    }
    #[doc = "Bit 30 - Write PINCFG"]
    #[inline(always)]
    #[must_use]
    pub fn wrpincfg(&mut self) -> WRPINCFG_W<30> {
        WRPINCFG_W::new(self)
    }
    #[doc = "Bit 31 - Half-Word Select"]
    #[inline(always)]
    #[must_use]
    pub fn hwsel(&mut self) -> HWSEL_W<31> {
        HWSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Configuration\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrconfig](index.html) module"]
pub struct WRCONFIG_SPEC;
impl crate::RegisterSpec for WRCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wrconfig::W](W) writer structure"]
impl crate::Writable for WRCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRCONFIG%s to value 0"]
impl crate::Resettable for WRCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
