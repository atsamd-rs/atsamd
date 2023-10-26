#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WINCTRL_SPEC>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WINCTRL_SPEC>;
#[doc = "Field `WEN0` reader - Window 0 Mode Enable"]
pub type WEN0_R = crate::BitReader;
#[doc = "Field `WEN0` writer - Window 0 Mode Enable"]
pub type WEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WINTSEL0` reader - Window 0 Interrupt Selection"]
pub type WINTSEL0_R = crate::FieldReader<WINTSEL0SELECT_A>;
#[doc = "Window 0 Interrupt Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WINTSEL0SELECT_A {
    #[doc = "0: Interrupt on signal above window"]
    ABOVE = 0,
    #[doc = "1: Interrupt on signal inside window"]
    INSIDE = 1,
    #[doc = "2: Interrupt on signal below window"]
    BELOW = 2,
    #[doc = "3: Interrupt on signal outside window"]
    OUTSIDE = 3,
}
impl From<WINTSEL0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WINTSEL0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WINTSEL0SELECT_A {
    type Ux = u8;
}
impl WINTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WINTSEL0SELECT_A {
        match self.bits {
            0 => WINTSEL0SELECT_A::ABOVE,
            1 => WINTSEL0SELECT_A::INSIDE,
            2 => WINTSEL0SELECT_A::BELOW,
            3 => WINTSEL0SELECT_A::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINTSEL0SELECT_A::ABOVE
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINTSEL0SELECT_A::INSIDE
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINTSEL0SELECT_A::BELOW
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINTSEL0SELECT_A::OUTSIDE
    }
}
#[doc = "Field `WINTSEL0` writer - Window 0 Interrupt Selection"]
pub type WINTSEL0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, WINTSEL0SELECT_A>;
impl<'a, REG, const O: u8> WINTSEL0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on signal above window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(WINTSEL0SELECT_A::ABOVE)
    }
    #[doc = "Interrupt on signal inside window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(WINTSEL0SELECT_A::INSIDE)
    }
    #[doc = "Interrupt on signal below window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(WINTSEL0SELECT_A::BELOW)
    }
    #[doc = "Interrupt on signal outside window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(WINTSEL0SELECT_A::OUTSIDE)
    }
}
impl R {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    pub fn wen0(&self) -> WEN0_R {
        WEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    pub fn wintsel0(&self) -> WINTSEL0_R {
        WINTSEL0_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Window 0 Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen0(&mut self) -> WEN0_W<WINCTRL_SPEC, 0> {
        WEN0_W::new(self)
    }
    #[doc = "Bits 1:2 - Window 0 Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wintsel0(&mut self) -> WINTSEL0_W<WINCTRL_SPEC, 1> {
        WINTSEL0_W::new(self)
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
#[doc = "Window Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
