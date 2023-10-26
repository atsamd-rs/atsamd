#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DBTP_SPEC>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DBTP_SPEC>;
#[doc = "Field `DSJW` reader - Data (Re)Synchronization Jump Width"]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DSJW` writer - Data (Re)Synchronization Jump Width"]
pub type DSJW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point"]
pub type DTSEG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point"]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point"]
pub type DTSEG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DBRP` reader - Data Baud Rate Prescaler"]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data Baud Rate Prescaler"]
pub type DBRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TDC` reader - Tranceiver Delay Compensation"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TDC` writer - Tranceiver Delay Compensation"]
pub type TDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Tranceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<DBTP_SPEC, 0> {
        DSJW_W::new(self)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<DBTP_SPEC, 4> {
        DTSEG2_W::new(self)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<DBTP_SPEC, 8> {
        DTSEG1_W::new(self)
    }
    #[doc = "Bits 16:20 - Data Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<DBTP_SPEC, 16> {
        DBRP_W::new(self)
    }
    #[doc = "Bit 23 - Tranceiver Delay Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<DBTP_SPEC, 23> {
        TDC_W::new(self)
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
#[doc = "Fast Bit Timing and Prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBTP_SPEC;
impl crate::RegisterSpec for DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DBTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DBTP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a33;
}
