#[doc = "Register `TMR` reader"]
pub struct R(crate::R<TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR` writer"]
pub struct W(crate::W<TMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_SPEC>;
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
impl From<crate::W<TMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: No data transfer or Non DMA data transfer"]
    DISABLE = 0,
    #[doc = "1: DMA data transfer"]
    ENABLE = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLE,
            true => DMAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMAEN_A::ENABLE
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No data transfer or Non DMA data transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLE)
    }
    #[doc = "DMA data transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCEN` reader - Block Count Enable"]
pub struct BCEN_R(crate::FieldReader<bool, BCEN_A>);
impl BCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::DISABLE,
            true => BCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BCEN_A::ENABLE
    }
}
impl core::ops::Deref for BCEN_R {
    type Target = crate::FieldReader<bool, BCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCEN` writer - Block Count Enable"]
pub struct BCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BCEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BCEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Auto Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACMDEN_A {
    #[doc = "0: Auto Command Disabled"]
    DISABLED = 0,
    #[doc = "1: Auto CMD12 Enable"]
    CMD12 = 1,
    #[doc = "2: Auto CMD23 Enable"]
    CMD23 = 2,
}
impl From<ACMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACMDEN` reader - Auto Command Enable"]
pub struct ACMDEN_R(crate::FieldReader<u8, ACMDEN_A>);
impl ACMDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACMDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACMDEN_A> {
        match self.bits {
            0 => Some(ACMDEN_A::DISABLED),
            1 => Some(ACMDEN_A::CMD12),
            2 => Some(ACMDEN_A::CMD23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `CMD12`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        **self == ACMDEN_A::CMD12
    }
    #[doc = "Checks if the value of the field is `CMD23`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        **self == ACMDEN_A::CMD23
    }
}
impl core::ops::Deref for ACMDEN_R {
    type Target = crate::FieldReader<u8, ACMDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMDEN` writer - Auto Command Enable"]
pub struct ACMDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMDEN_A::DISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut W {
        self.variant(ACMDEN_A::CMD12)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut W {
        self.variant(ACMDEN_A::CMD23)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "Data Transfer Direction Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSEL_A {
    #[doc = "0: Write (Host to Card)"]
    WRITE = 0,
    #[doc = "1: Read (Card to Host)"]
    READ = 1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDSEL` reader - Data Transfer Direction Selection"]
pub struct DTDSEL_R(crate::FieldReader<bool, DTDSEL_A>);
impl DTDSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::WRITE,
            true => DTDSEL_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == DTDSEL_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == DTDSEL_A::READ
    }
}
impl core::ops::Deref for DTDSEL_R {
    type Target = crate::FieldReader<bool, DTDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTDSEL` writer - Data Transfer Direction Selection"]
pub struct DTDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(DTDSEL_A::WRITE)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(DTDSEL_A::READ)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Multi/Single Block Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSEL_A {
    #[doc = "0: Single Block"]
    SINGLE = 0,
    #[doc = "1: Multiple Block"]
    MULTIPLE = 1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBSEL` reader - Multi/Single Block Selection"]
pub struct MSBSEL_R(crate::FieldReader<bool, MSBSEL_A>);
impl MSBSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::SINGLE,
            true => MSBSEL_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == MSBSEL_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        **self == MSBSEL_A::MULTIPLE
    }
}
impl core::ops::Deref for MSBSEL_R {
    type Target = crate::FieldReader<bool, MSBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSBSEL` writer - Multi/Single Block Selection"]
pub struct MSBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MSBSEL_A::SINGLE)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(MSBSEL_A::MULTIPLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&self) -> ACMDEN_R {
        ACMDEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&mut self) -> BCEN_W {
        BCEN_W { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline(always)]
    pub fn acmden(&mut self) -> ACMDEN_W {
        ACMDEN_W { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Selection"]
    #[inline(always)]
    pub fn dtdsel(&mut self) -> DTDSEL_W {
        DTDSEL_W { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Selection"]
    #[inline(always)]
    pub fn msbsel(&mut self) -> MSBSEL_W {
        MSBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](index.html) module"]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tmr::R](R) reader structure"]
impl crate::Readable for TMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr::W](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
