#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `ENABLE` writer - ICM Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISABLE` writer - ICM Disable Register"]
pub type DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REHASH` writer - Recompute Internal Hash"]
pub type REHASH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RMDIS` writer - Region Monitoring Disable"]
pub type RMDIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RMEN` writer - Region Monitoring Enable"]
pub type RMEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl W {
    #[doc = "Bit 0 - ICM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRL_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - ICM Disable Register"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<CTRL_SPEC, 1> {
        DISABLE_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CTRL_SPEC, 2> {
        SWRST_W::new(self)
    }
    #[doc = "Bits 4:7 - Recompute Internal Hash"]
    #[inline(always)]
    #[must_use]
    pub fn rehash(&mut self) -> REHASH_W<CTRL_SPEC, 4> {
        REHASH_W::new(self)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rmdis(&mut self) -> RMDIS_W<CTRL_SPEC, 8> {
        RMDIS_W::new(self)
    }
    #[doc = "Bits 12:15 - Region Monitoring Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmen(&mut self) -> RMEN_W<CTRL_SPEC, 12> {
        RMEN_W::new(self)
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
#[doc = "Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
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
