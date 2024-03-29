#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC` writer - 32-bit Cyclic Redundancy Check"]
pub type CRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MBIST` writer - Memory Built-In Self-Test"]
pub type MBIST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CE` writer - Chip Erase"]
pub type CE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRL_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 2 - 32-bit Cyclic Redundancy Check"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<CTRL_SPEC, 2> {
        CRC_W::new(self)
    }
    #[doc = "Bit 3 - Memory Built-In Self-Test"]
    #[inline(always)]
    #[must_use]
    pub fn mbist(&mut self) -> MBIST_W<CTRL_SPEC, 3> {
        MBIST_W::new(self)
    }
    #[doc = "Bit 4 - Chip Erase"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CE_W<CTRL_SPEC, 4> {
        CE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
