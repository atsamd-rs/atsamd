#[doc = "Register `QSPI_MR` reader"]
pub struct R(crate::R<QSPI_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_MR` writer"]
pub struct W(crate::W<QSPI_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_MR_SPEC>;
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
impl From<crate::W<QSPI_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMM_A {
    #[doc = "0: The QSPI is in SPI mode."]
    SPI = 0,
    #[doc = "1: The QSPI is in Serial Memory mode."]
    MEMORY = 1,
}
impl From<SMM_A> for bool {
    #[inline(always)]
    fn from(variant: SMM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMM` reader - Serial Memory Mode"]
pub struct SMM_R(crate::FieldReader<bool, SMM_A>);
impl SMM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMM_A {
        match self.bits {
            false => SMM_A::SPI,
            true => SMM_A::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == SMM_A::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        **self == SMM_A::MEMORY
    }
}
impl core::ops::Deref for SMM_R {
    type Target = crate::FieldReader<bool, SMM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMM` writer - Serial Memory Mode"]
pub struct SMM_W<'a> {
    w: &'a mut W,
}
impl<'a> SMM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(SMM_A::SPI)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(SMM_A::MEMORY)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLB_A {
    #[doc = "0: Local loopback path disabled."]
    DISABLED = 0,
    #[doc = "1: Local loopback path enabled."]
    ENABLED = 1,
}
impl From<LLB_A> for bool {
    #[inline(always)]
    fn from(variant: LLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub struct LLB_R(crate::FieldReader<bool, LLB_A>);
impl LLB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LLB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLB_A {
        match self.bits {
            false => LLB_A::DISABLED,
            true => LLB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LLB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LLB_A::ENABLED
    }
}
impl core::ops::Deref for LLB_R {
    type Target = crate::FieldReader<bool, LLB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub struct LLB_W<'a> {
    w: &'a mut W,
}
impl<'a> LLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LLB_A::DISABLED)
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LLB_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Wait Data Read Before Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRBT_A {
    #[doc = "0: No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    DISABLED = 0,
    #[doc = "1: In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    ENABLED = 1,
}
impl From<WDRBT_A> for bool {
    #[inline(always)]
    fn from(variant: WDRBT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub struct WDRBT_R(crate::FieldReader<bool, WDRBT_A>);
impl WDRBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDRBT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRBT_A {
        match self.bits {
            false => WDRBT_A::DISABLED,
            true => WDRBT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WDRBT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WDRBT_A::ENABLED
    }
}
impl core::ops::Deref for WDRBT_R {
    type Target = crate::FieldReader<bool, WDRBT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub struct WDRBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDRBT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDRBT_A::DISABLED)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDRBT_A::ENABLED)
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
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSMODE_A {
    #[doc = "0: The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NOT_RELOADED = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    LASTXFER = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY = 2,
}
impl From<CSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub struct CSMODE_R(crate::FieldReader<u8, CSMODE_A>);
impl CSMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSMODE_A> {
        match self.bits {
            0 => Some(CSMODE_A::NOT_RELOADED),
            1 => Some(CSMODE_A::LASTXFER),
            2 => Some(CSMODE_A::SYSTEMATICALLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RELOADED`"]
    #[inline(always)]
    pub fn is_not_reloaded(&self) -> bool {
        **self == CSMODE_A::NOT_RELOADED
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        **self == CSMODE_A::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        **self == CSMODE_A::SYSTEMATICALLY
    }
}
impl core::ops::Deref for CSMODE_R {
    type Target = crate::FieldReader<u8, CSMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub struct CSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn not_reloaded(self) -> &'a mut W {
        self.variant(CSMODE_A::NOT_RELOADED)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODE_A::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODE_A::SYSTEMATICALLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Number Of Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBBITS_A {
    #[doc = "0: 8 bits for transfer"]
    _8_BIT = 0,
    #[doc = "8: 16 bits for transfer"]
    _16_BIT = 8,
}
impl From<NBBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NBBITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBBITS` reader - Number Of Bits Per Transfer"]
pub struct NBBITS_R(crate::FieldReader<u8, NBBITS_A>);
impl NBBITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBBITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBBITS_A> {
        match self.bits {
            0 => Some(NBBITS_A::_8_BIT),
            8 => Some(NBBITS_A::_16_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        **self == NBBITS_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        **self == NBBITS_A::_16_BIT
    }
}
impl core::ops::Deref for NBBITS_R {
    type Target = crate::FieldReader<u8, NBBITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBBITS` writer - Number Of Bits Per Transfer"]
pub struct NBBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBBITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(NBBITS_A::_8_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(NBBITS_A::_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub struct DLYBCT_R(crate::FieldReader<u8, u8>);
impl DLYBCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub struct DLYBCT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DLYCS` reader - Minimum Inactive QCS Delay"]
pub struct DLYCS_R(crate::FieldReader<u8, u8>);
impl DLYCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYCS` writer - Minimum Inactive QCS Delay"]
pub struct DLYCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&self) -> SMM_R {
        SMM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&self) -> NBBITS_R {
        NBBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&mut self) -> SMM_W {
        SMM_W { w: self }
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W {
        LLB_W { w: self }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W {
        WDRBT_W { w: self }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&mut self) -> CSMODE_W {
        CSMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&mut self) -> NBBITS_W {
        NBBITS_W { w: self }
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W {
        DLYBCT_W { w: self }
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&mut self) -> DLYCS_W {
        DLYCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_mr](index.html) module"]
pub struct QSPI_MR_SPEC;
impl crate::RegisterSpec for QSPI_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_mr::R](R) reader structure"]
impl crate::Readable for QSPI_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_mr::W](W) writer structure"]
impl crate::Writable for QSPI_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_MR to value 0"]
impl crate::Resettable for QSPI_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
