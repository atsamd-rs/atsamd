#[doc = "Register `US_MAN` reader"]
pub struct R(crate::R<US_MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_MAN` writer"]
pub struct W(crate::W<US_MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_MAN_SPEC>;
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
impl From<crate::W<US_MAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub struct TX_PL_R(crate::FieldReader<u8, u8>);
impl TX_PL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub struct TX_PL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_PP_A {
    #[doc = "0: The preamble is composed of '1's"]
    ALL_ONE = 0,
    #[doc = "1: The preamble is composed of '0's"]
    ALL_ZERO = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZERO_ONE = 2,
    #[doc = "3: The preamble is composed of '10's"]
    ONE_ZERO = 3,
}
impl From<TX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub struct TX_PP_R(crate::FieldReader<u8, TX_PP_A>);
impl TX_PP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_PP_A {
        match self.bits {
            0 => TX_PP_A::ALL_ONE,
            1 => TX_PP_A::ALL_ZERO,
            2 => TX_PP_A::ZERO_ONE,
            3 => TX_PP_A::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        **self == TX_PP_A::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        **self == TX_PP_A::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        **self == TX_PP_A::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        **self == TX_PP_A::ONE_ZERO
    }
}
impl core::ops::Deref for TX_PP_R {
    type Target = crate::FieldReader<u8, TX_PP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub struct TX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(TX_PP_A::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(TX_PP_A::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(TX_PP_A::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(TX_PP_A::ONE_ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub struct TX_MPOL_R(crate::FieldReader<bool, bool>);
impl TX_MPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_MPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub struct TX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MPOL_W<'a> {
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
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub struct RX_PL_R(crate::FieldReader<u8, u8>);
impl RX_PL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_PL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub struct RX_PL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_PP_A {
    #[doc = "0: The preamble is composed of '1's"]
    ALL_ONE = 0,
    #[doc = "1: The preamble is composed of '0's"]
    ALL_ZERO = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZERO_ONE = 2,
    #[doc = "3: The preamble is composed of '10's"]
    ONE_ZERO = 3,
}
impl From<RX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub struct RX_PP_R(crate::FieldReader<u8, RX_PP_A>);
impl RX_PP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_PP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PP_A {
        match self.bits {
            0 => RX_PP_A::ALL_ONE,
            1 => RX_PP_A::ALL_ZERO,
            2 => RX_PP_A::ZERO_ONE,
            3 => RX_PP_A::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        **self == RX_PP_A::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        **self == RX_PP_A::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        **self == RX_PP_A::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        **self == RX_PP_A::ONE_ZERO
    }
}
impl core::ops::Deref for RX_PP_R {
    type Target = crate::FieldReader<u8, RX_PP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub struct RX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(RX_PP_A::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(RX_PP_A::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(RX_PP_A::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(RX_PP_A::ONE_ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub struct RX_MPOL_R(crate::FieldReader<bool, bool>);
impl RX_MPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub struct RX_MPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub struct ONE_R(crate::FieldReader<bool, bool>);
impl ONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DRIFT` reader - Drift Compensation"]
pub struct DRIFT_R(crate::FieldReader<bool, bool>);
impl DRIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIFT` writer - Drift Compensation"]
pub struct DRIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RXIDLEV` reader - Receiver Idle Value"]
pub struct RXIDLEV_R(crate::FieldReader<bool, bool>);
impl RXIDLEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIDLEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIDLEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIDLEV` writer - Receiver Idle Value"]
pub struct RXIDLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIDLEV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&self) -> RXIDLEV_R {
        RXIDLEV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&mut self) -> TX_PL_W {
        TX_PL_W { w: self }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&mut self) -> TX_PP_W {
        TX_PP_W { w: self }
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&mut self) -> TX_MPOL_W {
        TX_MPOL_W { w: self }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&mut self) -> RX_PL_W {
        RX_PL_W { w: self }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&mut self) -> RX_PP_W {
        RX_PP_W { w: self }
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&mut self) -> RX_MPOL_W {
        RX_MPOL_W { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&mut self) -> DRIFT_W {
        DRIFT_W { w: self }
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&mut self) -> RXIDLEV_W {
        RXIDLEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_man](index.html) module"]
pub struct US_MAN_SPEC;
impl crate::RegisterSpec for US_MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_man::R](R) reader structure"]
impl crate::Readable for US_MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_man::W](W) writer structure"]
impl crate::Writable for US_MAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_MAN to value 0"]
impl crate::Resettable for US_MAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
