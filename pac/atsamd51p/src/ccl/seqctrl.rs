#[doc = "Register `SEQCTRL[%s]` reader"]
pub type R = crate::R<SEQCTRL_SPEC>;
#[doc = "Register `SEQCTRL[%s]` writer"]
pub type W = crate::W<SEQCTRL_SPEC>;
#[doc = "Field `SEQSEL` reader - Sequential Selection"]
pub type SEQSEL_R = crate::FieldReader<SEQSELSELECT_A>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEQSELSELECT_A {
    #[doc = "0: Sequential logic is disabled"]
    DISABLE = 0,
    #[doc = "1: D flip flop"]
    DFF = 1,
    #[doc = "2: JK flip flop"]
    JK = 2,
    #[doc = "3: D latch"]
    LATCH = 3,
    #[doc = "4: RS latch"]
    RS = 4,
}
impl From<SEQSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEQSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEQSELSELECT_A {
    type Ux = u8;
}
impl SEQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEQSELSELECT_A> {
        match self.bits {
            0 => Some(SEQSELSELECT_A::DISABLE),
            1 => Some(SEQSELSELECT_A::DFF),
            2 => Some(SEQSELSELECT_A::JK),
            3 => Some(SEQSELSELECT_A::LATCH),
            4 => Some(SEQSELSELECT_A::RS),
            _ => None,
        }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEQSELSELECT_A::DISABLE
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == SEQSELSELECT_A::DFF
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == SEQSELSELECT_A::JK
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == SEQSELSELECT_A::LATCH
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == SEQSELSELECT_A::RS
    }
}
#[doc = "Field `SEQSEL` writer - Sequential Selection"]
pub type SEQSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, SEQSELSELECT_A>;
impl<'a, REG, const O: u8> SEQSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSELSELECT_A::DISABLE)
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSELSELECT_A::DFF)
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSELSELECT_A::JK)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSELSELECT_A::LATCH)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(SEQSELSELECT_A::RS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SEQSEL_R {
        SEQSEL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel(&mut self) -> SEQSEL_W<SEQCTRL_SPEC, 0> {
        SEQSEL_W::new(self)
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
#[doc = "SEQ Control x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL[%s]
to value 0"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
