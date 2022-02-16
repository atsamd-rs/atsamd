#[doc = "Register `SUPC_SMMR` reader"]
pub struct R(crate::R<SUPC_SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUPC_SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUPC_SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUPC_SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUPC_SMMR` writer"]
pub struct W(crate::W<SUPC_SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUPC_SMMR_SPEC>;
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
impl From<crate::W<SUPC_SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUPC_SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub struct SMTH_R(crate::FieldReader<u8, u8>);
impl SMTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub struct SMTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMSMPL_A {
    #[doc = "0: Supply Monitor disabled"]
    SMD = 0,
    #[doc = "1: Continuous Supply Monitor"]
    CSM = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK = 4,
}
impl From<SMSMPL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSMPL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub struct SMSMPL_R(crate::FieldReader<u8, SMSMPL_A>);
impl SMSMPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMSMPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMSMPL_A> {
        match self.bits {
            0 => Some(SMSMPL_A::SMD),
            1 => Some(SMSMPL_A::CSM),
            2 => Some(SMSMPL_A::_32SLCK),
            3 => Some(SMSMPL_A::_256SLCK),
            4 => Some(SMSMPL_A::_2048SLCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        **self == SMSMPL_A::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        **self == SMSMPL_A::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        **self == SMSMPL_A::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        **self == SMSMPL_A::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        **self == SMSMPL_A::_2048SLCK
    }
}
impl core::ops::Deref for SMSMPL_R {
    type Target = crate::FieldReader<u8, SMSMPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub struct SMSMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSMPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSMPL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPL_A::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPL_A::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_2048SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTEN_A {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub struct SMRSTEN_R(crate::FieldReader<bool, SMRSTEN_A>);
impl SMRSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMRSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTEN_A {
        match self.bits {
            false => SMRSTEN_A::NOT_ENABLE,
            true => SMRSTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == SMRSTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SMRSTEN_A::ENABLE
    }
}
impl core::ops::Deref for SMRSTEN_R {
    type Target = crate::FieldReader<bool, SMRSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub struct SMRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMRSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::NOT_ENABLE)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::ENABLE)
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
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIEN_A {
    #[doc = "0: The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub struct SMIEN_R(crate::FieldReader<bool, SMIEN_A>);
impl SMIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIEN_A {
        match self.bits {
            false => SMIEN_A::NOT_ENABLE,
            true => SMIEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == SMIEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SMIEN_A::ENABLE
    }
}
impl core::ops::Deref for SMIEN_R {
    type Target = crate::FieldReader<bool, SMIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub struct SMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIEN_A::NOT_ENABLE)
    }
    #[doc = "The SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&mut self) -> SMTH_W {
        SMTH_W { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&mut self) -> SMSMPL_W {
        SMSMPL_W { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&mut self) -> SMRSTEN_W {
        SMRSTEN_W { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&mut self) -> SMIEN_W {
        SMIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_smmr](index.html) module"]
pub struct SUPC_SMMR_SPEC;
impl crate::RegisterSpec for SUPC_SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [supc_smmr::R](R) reader structure"]
impl crate::Readable for SUPC_SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [supc_smmr::W](W) writer structure"]
impl crate::Writable for SUPC_SMMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUPC_SMMR to value 0"]
impl crate::Resettable for SUPC_SMMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
