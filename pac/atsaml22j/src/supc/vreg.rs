#[doc = "Register `VREG` reader"]
pub struct R(crate::R<VREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREG` writer"]
pub struct W(crate::W<VREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREG_SPEC>;
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
impl From<crate::W<VREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREG_SPEC, bool, O>;
#[doc = "Field `SEL` reader - Voltage Regulator Selection in active mode"]
pub type SEL_R = crate::FieldReader<u8, SELSELECT_A>;
#[doc = "Voltage Regulator Selection in active mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELSELECT_A {
    #[doc = "0: LDO selection"]
    LDO = 0,
    #[doc = "1: Buck selection"]
    BUCK = 1,
    #[doc = "2: Switched Cap selection"]
    SCVREG = 2,
}
impl From<SELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELSELECT_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELSELECT_A> {
        match self.bits {
            0 => Some(SELSELECT_A::LDO),
            1 => Some(SELSELECT_A::BUCK),
            2 => Some(SELSELECT_A::SCVREG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        *self == SELSELECT_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        *self == SELSELECT_A::BUCK
    }
    #[doc = "Checks if the value of the field is `SCVREG`"]
    #[inline(always)]
    pub fn is_scvreg(&self) -> bool {
        *self == SELSELECT_A::SCVREG
    }
}
#[doc = "Field `SEL` writer - Voltage Regulator Selection in active mode"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREG_SPEC, u8, SELSELECT_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(SELSELECT_A::LDO)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(SELSELECT_A::BUCK)
    }
    #[doc = "Switched Cap selection"]
    #[inline(always)]
    pub fn scvreg(self) -> &'a mut W {
        self.variant(SELSELECT_A::SCVREG)
    }
}
#[doc = "Field `STDBYPL0` reader - Standby in PL0"]
pub type STDBYPL0_R = crate::BitReader<bool>;
#[doc = "Field `STDBYPL0` writer - Standby in PL0"]
pub type STDBYPL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREG_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREG_SPEC, bool, O>;
#[doc = "Field `LPEFF` reader - Low Power efficiency"]
pub type LPEFF_R = crate::BitReader<bool>;
#[doc = "Field `LPEFF` writer - Low Power efficiency"]
pub type LPEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREG_SPEC, bool, O>;
#[doc = "Field `VSVSTEP` reader - Voltage Scaling Voltage Step"]
pub type VSVSTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSVSTEP` writer - Voltage Scaling Voltage Step"]
pub type VSVSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREG_SPEC, u8, u8, 4, O>;
#[doc = "Field `VSPER` reader - Voltage Scaling Period"]
pub type VSPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSPER` writer - Voltage Scaling Period"]
pub type VSPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Voltage Regulator Selection in active mode"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Standby in PL0"]
    #[inline(always)]
    pub fn stdbypl0(&self) -> STDBYPL0_R {
        STDBYPL0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Power efficiency"]
    #[inline(always)]
    pub fn lpeff(&self) -> LPEFF_R {
        LPEFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Scaling Voltage Step"]
    #[inline(always)]
    pub fn vsvstep(&self) -> VSVSTEP_R {
        VSVSTEP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VSPER_R {
        VSPER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Voltage Regulator Selection in active mode"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<2> {
        SEL_W::new(self)
    }
    #[doc = "Bit 5 - Standby in PL0"]
    #[inline(always)]
    #[must_use]
    pub fn stdbypl0(&mut self) -> STDBYPL0_W<5> {
        STDBYPL0_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Low Power efficiency"]
    #[inline(always)]
    #[must_use]
    pub fn lpeff(&mut self) -> LPEFF_W<8> {
        LPEFF_W::new(self)
    }
    #[doc = "Bits 16:19 - Voltage Scaling Voltage Step"]
    #[inline(always)]
    #[must_use]
    pub fn vsvstep(&mut self) -> VSVSTEP_W<16> {
        VSVSTEP_W::new(self)
    }
    #[doc = "Bits 24:31 - Voltage Scaling Period"]
    #[inline(always)]
    #[must_use]
    pub fn vsper(&mut self) -> VSPER_W<24> {
        VSPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREG Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg](index.html) module"]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreg::R](R) reader structure"]
impl crate::Readable for VREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreg::W](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREG to value 0"]
impl crate::Resettable for VREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
