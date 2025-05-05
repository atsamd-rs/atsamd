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
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
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
    #[doc = "6: OSC48M oscillator output"]
    OSC48M = 6,
    #[doc = "7: DPLL96M output"]
    DPLL96M = 7,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Source Select"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            0 => SRC_A::XOSC,
            1 => SRC_A::GCLKIN,
            2 => SRC_A::GCLKGEN1,
            3 => SRC_A::OSCULP32K,
            4 => SRC_A::OSC32K,
            5 => SRC_A::XOSC32K,
            6 => SRC_A::OSC48M,
            7 => SRC_A::DPLL96M,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        **self == SRC_A::XOSC
    }
    #[doc = "Checks if the value of the field is `GCLKIN`"]
    #[inline(always)]
    pub fn is_gclkin(&self) -> bool {
        **self == SRC_A::GCLKIN
    }
    #[doc = "Checks if the value of the field is `GCLKGEN1`"]
    #[inline(always)]
    pub fn is_gclkgen1(&self) -> bool {
        **self == SRC_A::GCLKGEN1
    }
    #[doc = "Checks if the value of the field is `OSCULP32K`"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        **self == SRC_A::OSCULP32K
    }
    #[doc = "Checks if the value of the field is `OSC32K`"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        **self == SRC_A::OSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        **self == SRC_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `OSC48M`"]
    #[inline(always)]
    pub fn is_osc48m(&self) -> bool {
        **self == SRC_A::OSC48M
    }
    #[doc = "Checks if the value of the field is `DPLL96M`"]
    #[inline(always)]
    pub fn is_dpll96m(&self) -> bool {
        **self == SRC_A::DPLL96M
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Source Select"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "XOSC oscillator output"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SRC_A::XOSC)
    }
    #[doc = "Generator input pad"]
    #[inline(always)]
    pub fn gclkin(self) -> &'a mut W {
        self.variant(SRC_A::GCLKIN)
    }
    #[doc = "Generic clock generator 1 output"]
    #[inline(always)]
    pub fn gclkgen1(self) -> &'a mut W {
        self.variant(SRC_A::GCLKGEN1)
    }
    #[doc = "OSCULP32K oscillator output"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut W {
        self.variant(SRC_A::OSCULP32K)
    }
    #[doc = "OSC32K oscillator output"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut W {
        self.variant(SRC_A::OSC32K)
    }
    #[doc = "XOSC32K oscillator output"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(SRC_A::XOSC32K)
    }
    #[doc = "OSC48M oscillator output"]
    #[inline(always)]
    pub fn osc48m(self) -> &'a mut W {
        self.variant(SRC_A::OSC48M)
    }
    #[doc = "DPLL96M output"]
    #[inline(always)]
    pub fn dpll96m(self) -> &'a mut W {
        self.variant(SRC_A::DPLL96M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `GENEN` reader - Generic Clock Generator Enable"]
pub struct GENEN_R(crate::FieldReader<bool, bool>);
impl GENEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GENEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENEN` writer - Generic Clock Generator Enable"]
pub struct GENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GENEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `IDC` reader - Improve Duty Cycle"]
pub struct IDC_R(crate::FieldReader<bool, bool>);
impl IDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDC` writer - Improve Duty Cycle"]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OOV` reader - Output Off Value"]
pub struct OOV_R(crate::FieldReader<bool, bool>);
impl OOV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OOV` writer - Output Off Value"]
pub struct OOV_W<'a> {
    w: &'a mut W,
}
impl<'a> OOV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OE` reader - Output Enable"]
pub struct OE_R(crate::FieldReader<bool, bool>);
impl OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE` writer - Output Enable"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Divide Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSEL_A {
    #[doc = "0: Divide input directly by divider factor"]
    DIV1 = 0,
    #[doc = "1: Divide input by 2^(divider factor+ 1)"]
    DIV2 = 1,
}
impl From<DIVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DIVSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVSEL` reader - Divide Selection"]
pub struct DIVSEL_R(crate::FieldReader<bool, DIVSEL_A>);
impl DIVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVSEL_A {
        match self.bits {
            false => DIVSEL_A::DIV1,
            true => DIVSEL_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == DIVSEL_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == DIVSEL_A::DIV2
    }
}
impl core::ops::Deref for DIVSEL_R {
    type Target = crate::FieldReader<bool, DIVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVSEL` writer - Divide Selection"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divide input directly by divider factor"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV1)
    }
    #[doc = "Divide input by 2^(divider factor+ 1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DIV` reader - Division Factor"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Division Factor"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Source Select"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&self) -> GENEN_R {
        GENEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&self) -> OOV_R {
        OOV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Select"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 8 - Generic Clock Generator Enable"]
    #[inline(always)]
    pub fn genen(&mut self) -> GENEN_W {
        GENEN_W { w: self }
    }
    #[doc = "Bit 9 - Improve Duty Cycle"]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bit 10 - Output Off Value"]
    #[inline(always)]
    pub fn oov(&mut self) -> OOV_W {
        OOV_W { w: self }
    }
    #[doc = "Bit 11 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 12 - Divide Selection"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bit 13 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
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
}
#[doc = "`reset()` method sets GENCTRL[%s]
to value 0"]
impl crate::Resettable for GENCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
