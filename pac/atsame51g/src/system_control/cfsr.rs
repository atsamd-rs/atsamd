#[doc = "Register `CFSR` reader"]
pub struct R(crate::R<CFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFSR` writer"]
pub struct W(crate::W<CFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFSR_SPEC>;
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
impl From<crate::W<CFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IACCVIOL` reader - Instruction access violation"]
pub type IACCVIOL_R = crate::BitReader<bool>;
#[doc = "Field `IACCVIOL` writer - Instruction access violation"]
pub type IACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `DACCVIOL` reader - Data access violation"]
pub type DACCVIOL_R = crate::BitReader<bool>;
#[doc = "Field `DACCVIOL` writer - Data access violation"]
pub type DACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MUNSTKERR` reader - MemManage Fault on unstacking for exception return"]
pub type MUNSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `MUNSTKERR` writer - MemManage Fault on unstacking for exception return"]
pub type MUNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MSTKERR` reader - MemManage Fault on stacking for exception entry"]
pub type MSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `MSTKERR` writer - MemManage Fault on stacking for exception entry"]
pub type MSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MLSPERR` reader - MemManager Fault occured during FP lazy state preservation"]
pub type MLSPERR_R = crate::BitReader<bool>;
#[doc = "Field `MLSPERR` writer - MemManager Fault occured during FP lazy state preservation"]
pub type MLSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `MMARVALID` reader - MemManage Fault Address Register valid"]
pub type MMARVALID_R = crate::BitReader<bool>;
#[doc = "Field `MMARVALID` writer - MemManage Fault Address Register valid"]
pub type MMARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `IBUSERR` reader - Instruction bus error"]
pub type IBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `IBUSERR` writer - Instruction bus error"]
pub type IBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `PRECISERR` reader - Precise data bus error"]
pub type PRECISERR_R = crate::BitReader<bool>;
#[doc = "Field `PRECISERR` writer - Precise data bus error"]
pub type PRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `IMPRECISERR` reader - Imprecise data bus error"]
pub type IMPRECISERR_R = crate::BitReader<bool>;
#[doc = "Field `IMPRECISERR` writer - Imprecise data bus error"]
pub type IMPRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `UNSTKERR` reader - BusFault on unstacking for exception return"]
pub type UNSTKERR_R = crate::BitReader<bool>;
#[doc = "Field `UNSTKERR` writer - BusFault on unstacking for exception return"]
pub type UNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `STKERR` reader - BusFault on stacking for exception entry"]
pub type STKERR_R = crate::BitReader<bool>;
#[doc = "Field `STKERR` writer - BusFault on stacking for exception entry"]
pub type STKERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `LSPERR` reader - BusFault occured during FP lazy state preservation"]
pub type LSPERR_R = crate::BitReader<bool>;
#[doc = "Field `LSPERR` writer - BusFault occured during FP lazy state preservation"]
pub type LSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `BFARVALID` reader - BusFault Address Register valid"]
pub type BFARVALID_R = crate::BitReader<bool>;
#[doc = "Field `BFARVALID` writer - BusFault Address Register valid"]
pub type BFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `UNDEFINSTR` reader - Undefined instruction UsageFault"]
pub type UNDEFINSTR_R = crate::BitReader<bool>;
#[doc = "Field `UNDEFINSTR` writer - Undefined instruction UsageFault"]
pub type UNDEFINSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `INVSTATE` reader - Invalid state UsageFault"]
pub type INVSTATE_R = crate::BitReader<bool>;
#[doc = "Field `INVSTATE` writer - Invalid state UsageFault"]
pub type INVSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `INVPC` reader - Invalid PC load UsageFault"]
pub type INVPC_R = crate::BitReader<bool>;
#[doc = "Field `INVPC` writer - Invalid PC load UsageFault"]
pub type INVPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `NOCP` reader - No coprocessor UsageFault"]
pub type NOCP_R = crate::BitReader<bool>;
#[doc = "Field `NOCP` writer - No coprocessor UsageFault"]
pub type NOCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `UNALIGNED` reader - Unaligned access UsageFault"]
pub type UNALIGNED_R = crate::BitReader<bool>;
#[doc = "Field `UNALIGNED` writer - Unaligned access UsageFault"]
pub type UNALIGNED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
#[doc = "Field `DIVBYZERO` reader - Divide by zero UsageFault"]
pub type DIVBYZERO_R = crate::BitReader<bool>;
#[doc = "Field `DIVBYZERO` writer - Divide by zero UsageFault"]
pub type DIVBYZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Instruction access violation"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data access violation"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - MemManage Fault on unstacking for exception return"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MemManage Fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManager Fault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register valid"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for exception return"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BusFault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register valid"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation"]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IACCVIOL_W<0> {
        IACCVIOL_W::new(self)
    }
    #[doc = "Bit 1 - Data access violation"]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DACCVIOL_W<1> {
        DACCVIOL_W::new(self)
    }
    #[doc = "Bit 3 - MemManage Fault on unstacking for exception return"]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W<3> {
        MUNSTKERR_W::new(self)
    }
    #[doc = "Bit 4 - MemManage Fault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MSTKERR_W<4> {
        MSTKERR_W::new(self)
    }
    #[doc = "Bit 5 - MemManager Fault occured during FP lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn mlsperr(&mut self) -> MLSPERR_W<5> {
        MLSPERR_W::new(self)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register valid"]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MMARVALID_W<7> {
        MMARVALID_W::new(self)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IBUSERR_W<8> {
        IBUSERR_W::new(self)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PRECISERR_W<9> {
        PRECISERR_W::new(self)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W<10> {
        IMPRECISERR_W::new(self)
    }
    #[doc = "Bit 11 - BusFault on unstacking for exception return"]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UNSTKERR_W<11> {
        UNSTKERR_W::new(self)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> STKERR_W<12> {
        STKERR_W::new(self)
    }
    #[doc = "Bit 13 - BusFault occured during FP lazy state preservation"]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LSPERR_W<13> {
        LSPERR_W::new(self)
    }
    #[doc = "Bit 15 - BusFault Address Register valid"]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BFARVALID_W<15> {
        BFARVALID_W::new(self)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W<16> {
        UNDEFINSTR_W::new(self)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invstate(&mut self) -> INVSTATE_W<17> {
        INVSTATE_W::new(self)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn invpc(&mut self) -> INVPC_W<18> {
        INVPC_W::new(self)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NOCP_W<19> {
        NOCP_W::new(self)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn unaligned(&mut self) -> UNALIGNED_W<24> {
        UNALIGNED_W::new(self)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W<25> {
        DIVBYZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurable Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](index.html) module"]
pub struct CFSR_SPEC;
impl crate::RegisterSpec for CFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsr::R](R) reader structure"]
impl crate::Readable for CFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfsr::W](W) writer structure"]
impl crate::Writable for CFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
