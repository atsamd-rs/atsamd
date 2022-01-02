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
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Voltage Regulator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: LDO selection"]
    LDO = 0,
    #[doc = "1: Buck selection"]
    BUCK = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Voltage Regulator Selection"]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::LDO,
            true => SEL_A::BUCK,
        }
    }
    #[doc = "Checks if the value of the field is `LDO`"]
    #[inline(always)]
    pub fn is_ldo(&self) -> bool {
        **self == SEL_A::LDO
    }
    #[doc = "Checks if the value of the field is `BUCK`"]
    #[inline(always)]
    pub fn is_buck(&self) -> bool {
        **self == SEL_A::BUCK
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Voltage Regulator Selection"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LDO selection"]
    #[inline(always)]
    pub fn ldo(self) -> &'a mut W {
        self.variant(SEL_A::LDO)
    }
    #[doc = "Buck selection"]
    #[inline(always)]
    pub fn buck(self) -> &'a mut W {
        self.variant(SEL_A::BUCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub struct RUNBKUP_R(crate::FieldReader<bool, bool>);
impl RUNBKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNBKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNBKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub struct RUNBKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNBKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `VSEN` reader - Voltage Scaling Enable"]
pub struct VSEN_R(crate::FieldReader<bool, bool>);
impl VSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSEN` writer - Voltage Scaling Enable"]
pub struct VSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `VSPER` reader - Voltage Scaling Period"]
pub struct VSPER_R(crate::FieldReader<u8, u8>);
impl VSPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VSPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSPER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSPER` writer - Voltage Scaling Period"]
pub struct VSPER_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RUNBKUP_R {
        RUNBKUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VSEN_R {
        VSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&self) -> VSPER_R {
        VSPER_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Voltage Regulator Selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&mut self) -> RUNBKUP_W {
        RUNBKUP_W { w: self }
    }
    #[doc = "Bit 16 - Voltage Scaling Enable"]
    #[inline(always)]
    pub fn vsen(&mut self) -> VSEN_W {
        VSEN_W { w: self }
    }
    #[doc = "Bits 24:26 - Voltage Scaling Period"]
    #[inline(always)]
    pub fn vsper(&mut self) -> VSPER_W {
        VSPER_W { w: self }
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
}
#[doc = "`reset()` method sets VREG to value 0x02"]
impl crate::Resettable for VREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
