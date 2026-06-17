#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `TRG` reader - Retrigger"]
pub type TRG_R = crate::BitReader<bool>;
#[doc = "Field `TRG` writer - Retrigger"]
pub type TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `CNT` reader - Counter"]
pub type CNT_R = crate::BitReader<bool>;
#[doc = "Field `CNT` writer - Counter"]
pub type CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `ERR` reader - Error"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - Error"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `UFS` reader - Non-Recoverable Update Fault"]
pub type UFS_R = crate::BitReader<bool>;
#[doc = "Field `UFS` writer - Non-Recoverable Update Fault"]
pub type UFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault"]
pub type DFS_R = crate::BitReader<bool>;
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault"]
pub type DFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `FAULTA` reader - Recoverable Fault A"]
pub type FAULTA_R = crate::BitReader<bool>;
#[doc = "Field `FAULTA` writer - Recoverable Fault A"]
pub type FAULTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `FAULTB` reader - Recoverable Fault B"]
pub type FAULTB_R = crate::BitReader<bool>;
#[doc = "Field `FAULTB` writer - Recoverable Fault B"]
pub type FAULTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0"]
pub type FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0"]
pub type FAULT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1"]
pub type FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1"]
pub type FAULT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `MC0` reader - Match or Capture 0"]
pub type MC0_R = crate::BitReader<bool>;
#[doc = "Field `MC0` writer - Match or Capture 0"]
pub type MC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `MC1` reader - Match or Capture 1"]
pub type MC1_R = crate::BitReader<bool>;
#[doc = "Field `MC1` writer - Match or Capture 1"]
pub type MC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `MC2` reader - Match or Capture 2"]
pub type MC2_R = crate::BitReader<bool>;
#[doc = "Field `MC2` writer - Match or Capture 2"]
pub type MC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `MC3` reader - Match or Capture 3"]
pub type MC3_R = crate::BitReader<bool>;
#[doc = "Field `MC3` writer - Match or Capture 3"]
pub type MC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault"]
    #[inline(always)]
    pub fn ufs(&self) -> UFS_R {
        UFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    pub fn mc2(&self) -> MC2_R {
        MC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    pub fn mc3(&self) -> MC3_R {
        MC3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TRG_W<1> {
        TRG_W::new(self)
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<2> {
        CNT_W::new(self)
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<3> {
        ERR_W::new(self)
    }
    #[doc = "Bit 10 - Non-Recoverable Update Fault"]
    #[inline(always)]
    #[must_use]
    pub fn ufs(&mut self) -> UFS_W<10> {
        UFS_W::new(self)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<11> {
        DFS_W::new(self)
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    #[must_use]
    pub fn faulta(&mut self) -> FAULTA_W<12> {
        FAULTA_W::new(self)
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    #[must_use]
    pub fn faultb(&mut self) -> FAULTB_W<13> {
        FAULTB_W::new(self)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    #[must_use]
    pub fn fault0(&mut self) -> FAULT0_W<14> {
        FAULT0_W::new(self)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<15> {
        FAULT1_W::new(self)
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> MC0_W<16> {
        MC0_W::new(self)
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<17> {
        MC1_W::new(self)
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    #[must_use]
    pub fn mc2(&mut self) -> MC2_W<18> {
        MC2_W::new(self)
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    #[must_use]
    pub fn mc3(&mut self) -> MC3_W<19> {
        MC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
