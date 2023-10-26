#[doc = "Register `DBGR` reader"]
pub type R = crate::R<DBGR_SPEC>;
#[doc = "Register `DBGR` writer"]
pub type W = crate::W<DBGR_SPEC>;
#[doc = "Field `NIDBG` reader - Non-intrusive debug enable"]
pub type NIDBG_R = crate::BitReader<NIDBGSELECT_A>;
#[doc = "Non-intrusive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NIDBGSELECT_A {
    #[doc = "0: Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    IDBG = 0,
    #[doc = "1: Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    NIDBG = 1,
}
impl From<NIDBGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NIDBGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NIDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NIDBGSELECT_A {
        match self.bits {
            false => NIDBGSELECT_A::IDBG,
            true => NIDBGSELECT_A::NIDBG,
        }
    }
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn is_idbg(&self) -> bool {
        *self == NIDBGSELECT_A::IDBG
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn is_nidbg(&self) -> bool {
        *self == NIDBGSELECT_A::NIDBG
    }
}
#[doc = "Field `NIDBG` writer - Non-intrusive debug enable"]
pub type NIDBG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NIDBGSELECT_A>;
impl<'a, REG, const O: u8> NIDBG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debugging is intrusive (reads of BDPR from debugger are considered and increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn idbg(self) -> &'a mut crate::W<REG> {
        self.variant(NIDBGSELECT_A::IDBG)
    }
    #[doc = "Debugging is not intrusive (reads of BDPR from debugger are discarded and do not increment the internal buffer pointer)"]
    #[inline(always)]
    pub fn nidbg(self) -> &'a mut crate::W<REG> {
        self.variant(NIDBGSELECT_A::NIDBG)
    }
}
impl R {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    pub fn nidbg(&self) -> NIDBG_R {
        NIDBG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-intrusive debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn nidbg(&mut self) -> NIDBG_W<DBGR_SPEC, 0> {
        NIDBG_W::new(self)
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
#[doc = "Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGR_SPEC;
impl crate::RegisterSpec for DBGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgr::R`](R) reader structure"]
impl crate::Readable for DBGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgr::W`](W) writer structure"]
impl crate::Writable for DBGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGR to value 0"]
impl crate::Resettable for DBGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
