#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCR_SPEC>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCR_SPEC>;
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASM` reader - ASM Restricted Operation Mode"]
pub type ASM_R = crate::BitReader;
#[doc = "Field `ASM` writer - ASM Restricted Operation Mode"]
pub type ASM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge"]
pub type CSA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSR` reader - Clock Stop Request"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Clock Stop Request"]
pub type CSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MON` reader - Bus Monitoring Mode"]
pub type MON_R = crate::BitReader;
#[doc = "Field `MON` writer - Bus Monitoring Mode"]
pub type MON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DAR_R = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEST` reader - Test Mode Enable"]
pub type TEST_R = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable"]
pub type TEST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDOE` reader - FD Operation Enable"]
pub type FDOE_R = crate::BitReader;
#[doc = "Field `FDOE` writer - FD Operation Enable"]
pub type FDOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRSE` reader - Bit Rate Switch Enable"]
pub type BRSE_R = crate::BitReader;
#[doc = "Field `BRSE` writer - Bit Rate Switch Enable"]
pub type BRSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable"]
pub type PXHD_R = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable"]
pub type PXHD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration"]
pub type EFBI_R = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration"]
pub type EFBI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXP` reader - Transmit Pause"]
pub type TXP_R = crate::BitReader;
#[doc = "Field `TXP` writer - Transmit Pause"]
pub type TXP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NISO` reader - Non ISO Operation"]
pub type NISO_R = crate::BitReader;
#[doc = "Field `NISO` writer - Non ISO Operation"]
pub type NISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn init(&mut self) -> INIT_W<CCCR_SPEC, 0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<CCCR_SPEC, 1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<CCCR_SPEC, 2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<CCCR_SPEC, 3> {
        CSA_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<CCCR_SPEC, 4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<CCCR_SPEC, 5> {
        MON_W::new(self)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<CCCR_SPEC, 6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<CCCR_SPEC, 7> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<CCCR_SPEC, 8> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<CCCR_SPEC, 9> {
        BRSE_W::new(self)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<CCCR_SPEC, 12> {
        PXHD_W::new(self)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<CCCR_SPEC, 13> {
        EFBI_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Pause"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<CCCR_SPEC, 14> {
        TXP_W::new(self)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<CCCR_SPEC, 15> {
        NISO_W::new(self)
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
#[doc = "CC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
