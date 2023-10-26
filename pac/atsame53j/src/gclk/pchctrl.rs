#[doc = "Register `PCHCTRL[%s]` reader"]
pub type R = crate::R<PCHCTRL_SPEC>;
#[doc = "Register `PCHCTRL[%s]` writer"]
pub type W = crate::W<PCHCTRL_SPEC>;
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GEN_R = crate::FieldReader<GENSELECT_A>;
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENSELECT_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
    #[doc = "8: Generic clock generator 8"]
    GCLK8 = 8,
    #[doc = "9: Generic clock generator 9"]
    GCLK9 = 9,
    #[doc = "10: Generic clock generator 10"]
    GCLK10 = 10,
    #[doc = "11: Generic clock generator 11"]
    GCLK11 = 11,
}
impl From<GENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GENSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GENSELECT_A {
    type Ux = u8;
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENSELECT_A> {
        match self.bits {
            0 => Some(GENSELECT_A::GCLK0),
            1 => Some(GENSELECT_A::GCLK1),
            2 => Some(GENSELECT_A::GCLK2),
            3 => Some(GENSELECT_A::GCLK3),
            4 => Some(GENSELECT_A::GCLK4),
            5 => Some(GENSELECT_A::GCLK5),
            6 => Some(GENSELECT_A::GCLK6),
            7 => Some(GENSELECT_A::GCLK7),
            8 => Some(GENSELECT_A::GCLK8),
            9 => Some(GENSELECT_A::GCLK9),
            10 => Some(GENSELECT_A::GCLK10),
            11 => Some(GENSELECT_A::GCLK11),
            _ => None,
        }
    }
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENSELECT_A::GCLK0
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENSELECT_A::GCLK1
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENSELECT_A::GCLK2
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENSELECT_A::GCLK3
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENSELECT_A::GCLK4
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENSELECT_A::GCLK5
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENSELECT_A::GCLK6
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENSELECT_A::GCLK7
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENSELECT_A::GCLK8
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn is_gclk9(&self) -> bool {
        *self == GENSELECT_A::GCLK9
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn is_gclk10(&self) -> bool {
        *self == GENSELECT_A::GCLK10
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn is_gclk11(&self) -> bool {
        *self == GENSELECT_A::GCLK11
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, GENSELECT_A>;
impl<'a, REG, const O: u8> GEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK8)
    }
    #[doc = "Generic clock generator 9"]
    #[inline(always)]
    pub fn gclk9(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK9)
    }
    #[doc = "Generic clock generator 10"]
    #[inline(always)]
    pub fn gclk10(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK10)
    }
    #[doc = "Generic clock generator 11"]
    #[inline(always)]
    pub fn gclk11(self) -> &'a mut crate::W<REG> {
        self.variant(GENSELECT_A::GCLK11)
    }
}
#[doc = "Field `CHEN` reader - Channel Enable"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel Enable"]
pub type CHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<PCHCTRL_SPEC, 0> {
        GEN_W::new(self)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<PCHCTRL_SPEC, 6> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<PCHCTRL_SPEC, 7> {
        WRTLOCK_W::new(self)
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
#[doc = "Peripheral Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pchctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pchctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCHCTRL_SPEC;
impl crate::RegisterSpec for PCHCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pchctrl::R`](R) reader structure"]
impl crate::Readable for PCHCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pchctrl::W`](W) writer structure"]
impl crate::Writable for PCHCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCHCTRL[%s]
to value 0"]
impl crate::Resettable for PCHCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
