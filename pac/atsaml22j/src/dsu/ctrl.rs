#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `CRC` writer - 32-bit Cyclic Redundancy Code"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `MBIST` writer - Memory built-in self-test"]
pub type MBIST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `CE` writer - Chip-Erase"]
pub type CE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `ARR` writer - Auxiliary Row Read"]
pub type ARR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `SMSA` writer - Start Memory Stream Access"]
pub type SMSA_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 2 - 32-bit Cyclic Redundancy Code"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<2> {
        CRC_W::new(self)
    }
    #[doc = "Bit 3 - Memory built-in self-test"]
    #[inline(always)]
    #[must_use]
    pub fn mbist(&mut self) -> MBIST_W<3> {
        MBIST_W::new(self)
    }
    #[doc = "Bit 4 - Chip-Erase"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CE_W<4> {
        CE_W::new(self)
    }
    #[doc = "Bit 6 - Auxiliary Row Read"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<6> {
        ARR_W::new(self)
    }
    #[doc = "Bit 7 - Start Memory Stream Access"]
    #[inline(always)]
    #[must_use]
    pub fn smsa(&mut self) -> SMSA_W<7> {
        SMSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
