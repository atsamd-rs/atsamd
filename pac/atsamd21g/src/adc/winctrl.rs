#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WINCTRL_SPEC>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WINCTRL_SPEC>;
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WINMODE_R = crate::FieldReader<WINMODESELECT_A>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINMODESELECT_A {
    #[doc = "0: No window mode (default)"]
    DISABLE = 0,
    #[doc = "1: Mode 1: RESULT > WINLT"]
    MODE1 = 1,
    #[doc = "2: Mode 2: RESULT &lt; WINUT"]
    MODE2 = 2,
    #[doc = "3: Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    MODE3 = 3,
    #[doc = "4: Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    MODE4 = 4,
}
impl From<WINMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINMODESELECT_A {
    type Ux = u8;
}
impl WINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WINMODESELECT_A> {
        match self.bits {
            0 => Some(WINMODESELECT_A::DISABLE),
            1 => Some(WINMODESELECT_A::MODE1),
            2 => Some(WINMODESELECT_A::MODE2),
            3 => Some(WINMODESELECT_A::MODE3),
            4 => Some(WINMODESELECT_A::MODE4),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WINMODESELECT_A::DISABLE
    }
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WINMODESELECT_A::MODE1
    }
    #[doc = "Mode 2: RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WINMODESELECT_A::MODE2
    }
    #[doc = "Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WINMODESELECT_A::MODE3
    }
    #[doc = "Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == WINMODESELECT_A::MODE4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WINMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WINMODESELECT_A>;
impl<'a, REG, const O: u8> WINMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::DISABLE)
    }
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE1)
    }
    #[doc = "Mode 2: RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE2)
    }
    #[doc = "Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE3)
    }
    #[doc = "Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODESELECT_A::MODE4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<WINCTRL_SPEC, 0> {
        WINMODE_W::new(self)
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
#[doc = "Window Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINCTRL_SPEC;
impl crate::RegisterSpec for WINCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`winctrl::R`](R) reader structure"]
impl crate::Readable for WINCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winctrl::W`](W) writer structure"]
impl crate::Writable for WINCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WINCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
