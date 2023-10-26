#[doc = "Register `GENCTRL` reader"]
pub type R = crate::R<GENCTRL_SPEC>;
#[doc = "Register `GENCTRL` writer"]
pub type W = crate::W<GENCTRL_SPEC>;
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SRC` reader - Source Select"]
pub type SRC_R = crate::FieldReader<SRCSELECT_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCSELECT_A {
    #[doc = "0: XOSC oscillator output"]
    XOSC = 0,
    #[doc = "1: Generator input pad"]
    GCLKIN = 1,
    #[doc = "2: Generic clock generator 1 output"]
    GCLKGEN1 = 2,
    #[doc = "3: OSCULP32K oscillator output"]
    OSCULP32K = 3,
    #[doc = "4: OSC32K oscillator output"]
    OSC32K = 4,
    #[doc = "5: XOSC32K oscillator output"]
    XOSC32K = 5,
    #[doc = "6: OSC8M oscillator output"]
    OSC8M = 6,
    #[doc = "7: DFLL48M output"]
    DFLL48M = 7,
    #[doc = "8: DPLL96M output"]
    DPLL96M = 8,
}
impl From<SRCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRCSELECT_A {
    type Ux = u8;
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRCSELECT_A> {
        match self.bits {
            0 => Some(SRCSELECT_A::XOSC),
            1 => Some(SRCSELECT_A::GCLKIN),
            2 => Some(SRCSELECT_A::GCLKGEN1),
            3 => Some(SRCSELECT_A::OSCULP32K),
            4 => Some(SRCSELECT_A::OSC32K),
            5 => Some(SRCSELECT_A::XOSC32K),
            6 => Some(SRCSELECT_A::OSC8M),
            7 => Some(SRCSELECT_A::DFLL48M),
            8 => Some(SRCSELECT_A::DPLL96M),
            _ => None,
        }
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == SRCSELECT_A::XOSC
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        *self == SRCSELECT_A::GCLKIN
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        *self == SRCSELECT_A::GCLKGEN1
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == SRCSELECT_A::OSCULP32K
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == SRCSELECT_A::OSC32K
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == SRCSELECT_A::XOSC32K
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn is_osc8m(&self) -> bool {
        *self == SRCSELECT_A::OSC8M
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn is_dfll48m(&self) -> bool {
        *self == SRCSELECT_A::DFLL48M
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn is_dpll96m(&self) -> bool {
        *self == SRCSELECT_A::DPLL96M
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub type SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, SRCSELECT_A>;
impl<'a, REG, const O: u8> SRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::XOSC)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::OSCULP32K)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::OSC32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::XOSC32K)
    }
    #[doc = "OSC8M oscillator output"]
    #[inline(always)]
    pub fn osc8m(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::OSC8M)
    }
    #[doc = "DFLL48M output"]
    #[inline(always)]
    pub fn dfll48m(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::DFLL48M)
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn dpll96m(self) -> &'a mut crate::W<REG> {
        self.variant(SRCSELECT_A::DPLL96M)
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub type GENEN_R = crate::BitReader;
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub type GENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub type IDC_R = crate::BitReader;
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub type IDC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOV` reader - Output Off Value"]
pub type OOV_R = crate::BitReader;
#[doc = "Field `OOV` writer - Output Off Value"]
pub type OOV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OE` reader - Output Enable"]
pub type OE_R = crate::BitReader;
#[doc = "Field `OE` writer - Output Enable"]
pub type OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub type DIVSEL_R = crate::BitReader;
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub type DIVSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OOV_R {
        OOV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<GENCTRL_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:12 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<GENCTRL_SPEC, 8> {
        SRC_W::new(self)
    }
    #[doc = "Bit 16 - Generic Clock Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn genen(&mut self) -> GENEN_W<GENCTRL_SPEC, 16> {
        GENEN_W::new(self)
    }
    #[doc = "Bit 17 - Improve Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn idc(&mut self) -> IDC_W<GENCTRL_SPEC, 17> {
        IDC_W::new(self)
    }
    #[doc = "Bit 18 - Output Off Value"]
    #[inline(always)]
    #[must_use]
    pub fn oov(&mut self) -> OOV_W<GENCTRL_SPEC, 18> {
        OOV_W::new(self)
    }
    #[doc = "Bit 19 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<GENCTRL_SPEC, 19> {
        OE_W::new(self)
    }
    #[doc = "Bit 20 - Divide Selection"]
    #[inline(always)]
    #[must_use]
    pub fn divsel(&mut self) -> DIVSEL_W<GENCTRL_SPEC, 20> {
        DIVSEL_W::new(self)
    }
    #[doc = "Bit 21 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<GENCTRL_SPEC, 21> {
        RUNSTDBY_W::new(self)
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
#[doc = "Generic Clock Generator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`genctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`genctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GENCTRL_SPEC;
impl crate::RegisterSpec for GENCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genctrl::R`](R) reader structure"]
impl crate::Readable for GENCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`genctrl::W`](W) writer structure"]
impl crate::Writable for GENCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENCTRL to value 0"]
impl crate::Resettable for GENCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
