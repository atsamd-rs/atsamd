#[doc = "Register `STDBYCFG` reader"]
pub type R = crate::R<STDBYCFG_SPEC>;
#[doc = "Register `STDBYCFG` writer"]
pub type W = crate::W<STDBYCFG_SPEC>;
#[doc = "Field `RAMCFG` reader - Ram Configuration"]
pub type RAMCFG_R = crate::FieldReader<RAMCFGSELECT_A>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMCFGSELECT_A {
    #[doc = "0: All the system RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 32Kbytes of the system RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the system RAM is turned OFF"]
    OFF = 2,
}
impl From<RAMCFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMCFGSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMCFGSELECT_A {
    type Ux = u8;
}
impl RAMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMCFGSELECT_A> {
        match self.bits {
            0 => Some(RAMCFGSELECT_A::RET),
            1 => Some(RAMCFGSELECT_A::PARTIAL),
            2 => Some(RAMCFGSELECT_A::OFF),
            _ => None,
        }
    }
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == RAMCFGSELECT_A::RET
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == RAMCFGSELECT_A::PARTIAL
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMCFGSELECT_A::OFF
    }
}
#[doc = "Field `RAMCFG` writer - Ram Configuration"]
pub type RAMCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, RAMCFGSELECT_A>;
impl<'a, REG, const O: u8> RAMCFG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGSELECT_A::RET)
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGSELECT_A::PARTIAL)
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(RAMCFGSELECT_A::OFF)
    }
}
#[doc = "Field `FASTWKUP` reader - Fast Wakeup"]
pub type FASTWKUP_R = crate::FieldReader<FASTWKUPSELECT_A>;
#[doc = "Fast Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FASTWKUPSELECT_A {
    #[doc = "0: Fast Wakeup is disabled"]
    NO = 0,
    #[doc = "1: Fast Wakeup is enabled on NVM"]
    NVM = 1,
    #[doc = "2: Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    MAINVREG = 2,
    #[doc = "3: Fast Wakeup is enabled on both NVM and MAINVREG"]
    BOTH = 3,
}
impl From<FASTWKUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FASTWKUPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FASTWKUPSELECT_A {
    type Ux = u8;
}
impl FASTWKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FASTWKUPSELECT_A {
        match self.bits {
            0 => FASTWKUPSELECT_A::NO,
            1 => FASTWKUPSELECT_A::NVM,
            2 => FASTWKUPSELECT_A::MAINVREG,
            3 => FASTWKUPSELECT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Fast Wakeup is disabled"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FASTWKUPSELECT_A::NO
    }
    #[doc = "Fast Wakeup is enabled on NVM"]
    #[inline(always)]
    pub fn is_nvm(&self) -> bool {
        *self == FASTWKUPSELECT_A::NVM
    }
    #[doc = "Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    #[inline(always)]
    pub fn is_mainvreg(&self) -> bool {
        *self == FASTWKUPSELECT_A::MAINVREG
    }
    #[doc = "Fast Wakeup is enabled on both NVM and MAINVREG"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == FASTWKUPSELECT_A::BOTH
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wakeup"]
pub type FASTWKUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, FASTWKUPSELECT_A>;
impl<'a, REG, const O: u8> FASTWKUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fast Wakeup is disabled"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(FASTWKUPSELECT_A::NO)
    }
    #[doc = "Fast Wakeup is enabled on NVM"]
    #[inline(always)]
    pub fn nvm(self) -> &'a mut crate::W<REG> {
        self.variant(FASTWKUPSELECT_A::NVM)
    }
    #[doc = "Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    #[inline(always)]
    pub fn mainvreg(self) -> &'a mut crate::W<REG> {
        self.variant(FASTWKUPSELECT_A::MAINVREG)
    }
    #[doc = "Fast Wakeup is enabled on both NVM and MAINVREG"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(FASTWKUPSELECT_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfg(&mut self) -> RAMCFG_W<STDBYCFG_SPEC, 0> {
        RAMCFG_W::new(self)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fastwkup(&mut self) -> FASTWKUP_W<STDBYCFG_SPEC, 4> {
        FASTWKUP_W::new(self)
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
#[doc = "Standby Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stdbycfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stdbycfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STDBYCFG_SPEC;
impl crate::RegisterSpec for STDBYCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stdbycfg::R`](R) reader structure"]
impl crate::Readable for STDBYCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stdbycfg::W`](W) writer structure"]
impl crate::Writable for STDBYCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STDBYCFG to value 0"]
impl crate::Resettable for STDBYCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
