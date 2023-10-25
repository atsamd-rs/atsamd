#[doc = "Register `SRR` reader"]
pub type R = crate::R<SRR_SPEC>;
#[doc = "Register `SRR` writer"]
pub type W = crate::W<SRR_SPEC>;
#[doc = "Field `SWRSTALL` reader - Software Reset For All"]
pub type SWRSTALL_R = crate::BitReader<SWRSTALLSELECT_A>;
#[doc = "Software Reset For All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTALLSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTALLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTALLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRSTALLSELECT_A {
        match self.bits {
            false => SWRSTALLSELECT_A::WORK,
            true => SWRSTALLSELECT_A::RESET,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTALLSELECT_A::WORK
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTALLSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTALL` writer - Software Reset For All"]
pub type SWRSTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWRSTALLSELECT_A>;
impl<'a, REG, const O: u8> SWRSTALL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTALLSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTALLSELECT_A::RESET)
    }
}
#[doc = "Field `SWRSTCMD` reader - Software Reset For CMD Line"]
pub type SWRSTCMD_R = crate::BitReader<SWRSTCMDSELECT_A>;
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTCMDSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTCMDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTCMDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRSTCMDSELECT_A {
        match self.bits {
            false => SWRSTCMDSELECT_A::WORK,
            true => SWRSTCMDSELECT_A::RESET,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTCMDSELECT_A::WORK
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTCMDSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTCMD` writer - Software Reset For CMD Line"]
pub type SWRSTCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWRSTCMDSELECT_A>;
impl<'a, REG, const O: u8> SWRSTCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTCMDSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTCMDSELECT_A::RESET)
    }
}
#[doc = "Field `SWRSTDAT` reader - Software Reset For DAT Line"]
pub type SWRSTDAT_R = crate::BitReader<SWRSTDATSELECT_A>;
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTDATSELECT_A {
    #[doc = "0: Work"]
    WORK = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<SWRSTDATSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWRSTDATSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRSTDAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRSTDATSELECT_A {
        match self.bits {
            false => SWRSTDATSELECT_A::WORK,
            true => SWRSTDATSELECT_A::RESET,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == SWRSTDATSELECT_A::WORK
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRSTDATSELECT_A::RESET
    }
}
#[doc = "Field `SWRSTDAT` writer - Software Reset For DAT Line"]
pub type SWRSTDAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWRSTDATSELECT_A>;
impl<'a, REG, const O: u8> SWRSTDAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTDATSELECT_A::WORK)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRSTDATSELECT_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&self) -> SWRSTALL_R {
        SWRSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SWRSTCMD_R {
        SWRSTCMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SWRSTDAT_R {
        SWRSTDAT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    #[must_use]
    pub fn swrstall(&mut self) -> SWRSTALL_W<SRR_SPEC, 0> {
        SWRSTALL_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstcmd(&mut self) -> SWRSTCMD_W<SRR_SPEC, 1> {
        SWRSTCMD_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstdat(&mut self) -> SWRSTDAT_W<SRR_SPEC, 2> {
        SWRSTDAT_W::new(self)
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
#[doc = "Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
