#[doc = "Register `REFCTRL` reader"]
pub type R = crate::R<REFCTRL_SPEC>;
#[doc = "Register `REFCTRL` writer"]
pub type W = crate::W<REFCTRL_SPEC>;
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<REFSELSELECT_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSELSELECT_A {
    #[doc = "0: 1.0V voltage reference"]
    INT1V = 0,
    #[doc = "1: 1/1.48 VDDANA"]
    INTVCC0 = 1,
    #[doc = "2: 1/2 VDDANA (only for VDDANA > 2.0V)"]
    INTVCC1 = 2,
    #[doc = "3: External reference"]
    AREFA = 3,
    #[doc = "4: External reference"]
    AREFB = 4,
}
impl From<REFSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFSELSELECT_A {
    type Ux = u8;
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REFSELSELECT_A> {
        match self.bits {
            0 => Some(REFSELSELECT_A::INT1V),
            1 => Some(REFSELSELECT_A::INTVCC0),
            2 => Some(REFSELSELECT_A::INTVCC1),
            3 => Some(REFSELSELECT_A::AREFA),
            4 => Some(REFSELSELECT_A::AREFB),
            _ => None,
        }
    }
    #[doc = "1.0V voltage reference"]
    #[inline(always)]
    pub fn is_int1v(&self) -> bool {
        *self == REFSELSELECT_A::INT1V
    }
    #[doc = "1/1.48 VDDANA"]
    #[inline(always)]
    pub fn is_intvcc0(&self) -> bool {
        *self == REFSELSELECT_A::INTVCC0
    }
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    #[inline(always)]
    pub fn is_intvcc1(&self) -> bool {
        *self == REFSELSELECT_A::INTVCC1
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn is_arefa(&self) -> bool {
        *self == REFSELSELECT_A::AREFA
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == REFSELSELECT_A::AREFB
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, REFSELSELECT_A>;
impl<'a, REG, const O: u8> REFSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.0V voltage reference"]
    #[inline(always)]
    pub fn int1v(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::INT1V)
    }
    #[doc = "1/1.48 VDDANA"]
    #[inline(always)]
    pub fn intvcc0(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::INTVCC0)
    }
    #[doc = "1/2 VDDANA (only for VDDANA > 2.0V)"]
    #[inline(always)]
    pub fn intvcc1(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::INTVCC1)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefa(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::AREFA)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut crate::W<REG> {
        self.variant(REFSELSELECT_A::AREFB)
    }
}
#[doc = "Field `REFCOMP` reader - Reference Buffer Offset Compensation Enable"]
pub type REFCOMP_R = crate::BitReader;
#[doc = "Field `REFCOMP` writer - Reference Buffer Offset Compensation Enable"]
pub type REFCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&self) -> REFCOMP_R {
        REFCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<REFCTRL_SPEC, 0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refcomp(&mut self) -> REFCOMP_W<REFCTRL_SPEC, 7> {
        REFCOMP_W::new(self)
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
#[doc = "Reference Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REFCTRL_SPEC;
impl crate::RegisterSpec for REFCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`refctrl::R`](R) reader structure"]
impl crate::Readable for REFCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`refctrl::W`](W) writer structure"]
impl crate::Writable for REFCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFCTRL to value 0"]
impl crate::Resettable for REFCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
