#[doc = "Register `GENCTRL[%s]` reader"]
pub struct R(crate::R<GENCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENCTRL[%s]` writer"]
pub struct W(crate::W<GENCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENCTRL_SPEC>;
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
impl From<crate::W<GENCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Source Select"]
pub type SRC_R = crate::FieldReader<u8, SRCSELECT_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCSELECT_A {
    #[doc = "0: XOSC0 oscillator output"]
    XOSC0 = 0,
    #[doc = "1: XOSC1 oscillator output"]
    XOSC1 = 1,
    #[doc = "2: Generator input pad"]
    GCLKIN = 2,
    #[doc = "3: Generic clock generator 1 output"]
    GCLKGEN1 = 3,
    #[doc = "4: OSCULP32K oscillator output"]
    OSCULP32K = 4,
    #[doc = "5: XOSC32K oscillator output"]
    XOSC32K = 5,
    #[doc = "6: DFLL output"]
    DFLL = 6,
    #[doc = "7: DPLL0 output"]
    DPLL0 = 7,
    #[doc = "8: DPLL1 output"]
    DPLL1 = 8,
}
impl From<SRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCSELECT_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCSELECT_A> {
        match self.bits {
            0 => Some(SRCSELECT_A::XOSC0),
            1 => Some(SRCSELECT_A::XOSC1),
            2 => Some(SRCSELECT_A::GCLKIN),
            3 => Some(SRCSELECT_A::GCLKGEN1),
            4 => Some(SRCSELECT_A::OSCULP32K),
            5 => Some(SRCSELECT_A::XOSC32K),
            6 => Some(SRCSELECT_A::DFLL),
            7 => Some(SRCSELECT_A::DPLL0),
            8 => Some(SRCSELECT_A::DPLL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC0`"]
    #[inline(always)]
    pub fn is_xosc0(&self) -> bool {
        *self == SRCSELECT_A::XOSC0
    }
    #[doc = "Checks if the value of the field is `XOSC1`"]
    #[inline(always)]
    pub fn is_xosc1(&self) -> bool {
        *self == SRCSELECT_A::XOSC1
    }
    #[doc = "Checks if the value of the field is `GCLKIN`"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == SRCSELECT_A::GCLKIN
    }
    #[doc = "Checks if the value of the field is `GCLKGEN1`"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRCSELECT_A::GCLKGEN1
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == SRCSELECT_A::OSCULP32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == SRCSELECT_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `DFLL`"]
    #[inline(always)]
    pub fn is_dfll(&self) -> bool {
        *self == SRCSELECT_A::DFLL
    }
    #[doc = "Checks if the value of the field is `DPLL0`"]
    #[inline(always)]
    pub fn is_dpll0(&self) -> bool {
        *self == SRCSELECT_A::DPLL0
    }
    #[doc = "Checks if the value of the field is `DPLL1`"]
    #[inline(always)]
    pub fn is_dpll1(&self) -> bool {
        *self == SRCSELECT_A::DPLL1
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GENCTRL_SPEC, u8, SRCSELECT_A, 4, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "XOSC0 oscillator output"]
    #[inline(always)]
    pub fn xosc0(self) -> &'a mut W {
        self.variant(SRCSELECT_A::XOSC0)
    }
    #[doc = "XOSC1 oscillator output"]
    #[inline(always)]
    pub fn xosc1(self) -> &'a mut W {
        self.variant(SRCSELECT_A::XOSC1)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut W {
        self.variant(SRCSELECT_A::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut W {
        self.variant(SRCSELECT_A::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(SRCSELECT_A::OSCULP32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(SRCSELECT_A::XOSC32K)
    }
    #[doc = "DFLL output"]
    #[inline(always)]
    pub fn dfll(self) -> &'a mut W {
        self.variant(SRCSELECT_A::DFLL)
    }
    #[doc = "DPLL0 output"]
    #[inline(always)]
    pub fn dpll0(self) -> &'a mut W {
        self.variant(SRCSELECT_A::DPLL0)
    }
    #[doc = "DPLL1 output"]
    #[inline(always)]
    pub fn dpll1(self) -> &'a mut W {
        self.variant(SRCSELECT_A::DPLL1)
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub type GENEN_R = crate::BitReader<bool>;
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub type GENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, bool, O>;
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub type IDC_R = crate::BitReader<bool>;
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub type IDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, bool, O>;
#[doc = "Field `OOV` reader - Output Off Value"]
pub type OOV_R = crate::BitReader<bool>;
#[doc = "Field `OOV` writer - Output Off Value"]
pub type OOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, bool, O>;
#[doc = "Field `OE` reader - Output Enable"]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - Output Enable"]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, bool, O>;
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub type DIVSEL_R = crate::BitReader<DIVSELSELECT_A>;
#[doc = "Divide Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVSELSELECT_A {
    #[doc = "0: Divide input directly by divider factor"]
    DIV1 = 0,
    #[doc = "1: Divide input by 2^(divider factor+ 1)"]
    DIV2 = 1,
}
impl From<DIVSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIVSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DIVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVSELSELECT_A {
        match self.bits {
            false => DIVSELSELECT_A::DIV1,
            true => DIVSELSELECT_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVSELSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVSELSELECT_A::DIV2
    }
}
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub type DIVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, DIVSELSELECT_A, O>;
impl<'a, const O: u8> DIVSEL_W<'a, O> {
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVSELSELECT_A::DIV1)
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVSELSELECT_A::DIV2)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GENCTRL_SPEC, bool, O>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GENCTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OOV_R {
        OOV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn genen(&mut self) -> GENEN_W<8> {
        GENEN_W::new(self)
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IDC_W<9> {
        IDC_W::new(self)
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    #[must_use]
    pub fn oov(&mut self) -> OOV_W<10> {
        OOV_W::new(self)
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<11> {
        OE_W::new(self)
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    #[must_use]
    pub fn divsel(&mut self) -> DIVSEL_W<12> {
        DIVSEL_W::new(self)
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<13> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Generator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [genctrl](index.html) module"]
pub struct GENCTRL_SPEC;
impl crate::RegisterSpec for GENCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [genctrl::R](R) reader structure"]
impl crate::Readable for GENCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [genctrl::W](W) writer structure"]
impl crate::Writable for GENCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENCTRL[%s]
to value 0"]
impl crate::Resettable for GENCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
