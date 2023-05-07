#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader<bool>;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `ASM` reader - ASM Restricted Operation Mode"]
pub type ASM_R = crate::BitReader<bool>;
#[doc = "Field `ASM` writer - ASM Restricted Operation Mode"]
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge"]
pub type CSA_R = crate::BitReader<bool>;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge"]
pub type CSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CSR` reader - Clock Stop Request"]
pub type CSR_R = crate::BitReader<bool>;
#[doc = "Field `CSR` writer - Clock Stop Request"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `MON` reader - Bus Monitoring Mode"]
pub type MON_R = crate::BitReader<bool>;
#[doc = "Field `MON` writer - Bus Monitoring Mode"]
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DAR_R = crate::BitReader<bool>;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `TEST` reader - Test Mode Enable"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Test Mode Enable"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `FDOE` reader - FD Operation Enable"]
pub type FDOE_R = crate::BitReader<bool>;
#[doc = "Field `FDOE` writer - FD Operation Enable"]
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `BRSE` reader - Bit Rate Switch Enable"]
pub type BRSE_R = crate::BitReader<bool>;
#[doc = "Field `BRSE` writer - Bit Rate Switch Enable"]
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable"]
pub type PXHD_R = crate::BitReader<bool>;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable"]
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration"]
pub type EFBI_R = crate::BitReader<bool>;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration"]
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `TXP` reader - Transmit Pause"]
pub type TXP_R = crate::BitReader<bool>;
#[doc = "Field `TXP` writer - Transmit Pause"]
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `NISO` reader - Non ISO Operation"]
pub type NISO_R = crate::BitReader<bool>;
#[doc = "Field `NISO` writer - Non ISO Operation"]
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<3> {
        CSA_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Pause"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
