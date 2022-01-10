#[doc = "Register `NISTR` reader"]
pub struct R(crate::R<NISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NISTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NISTR` writer"]
pub struct W(crate::W<NISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NISTR_SPEC>;
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
impl From<crate::W<NISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NISTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDC_A {
    #[doc = "0: No command complete"]
    NO = 0,
    #[doc = "1: Command complete"]
    YES = 1,
}
impl From<CMDC_A> for bool {
    #[inline(always)]
    fn from(variant: CMDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDC` reader - Command Complete"]
pub struct CMDC_R(crate::FieldReader<bool, CMDC_A>);
impl CMDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDC_A {
        match self.bits {
            false => CMDC_A::NO,
            true => CMDC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == CMDC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CMDC_A::YES
    }
}
impl core::ops::Deref for CMDC_R {
    type Target = crate::FieldReader<bool, CMDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDC` writer - Command Complete"]
pub struct CMDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No command complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDC_A::NO)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDC_A::YES)
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
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRFC_A {
    #[doc = "0: Not complete"]
    NO = 0,
    #[doc = "1: Command execution is completed"]
    YES = 1,
}
impl From<TRFC_A> for bool {
    #[inline(always)]
    fn from(variant: TRFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRFC` reader - Transfer Complete"]
pub struct TRFC_R(crate::FieldReader<bool, TRFC_A>);
impl TRFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRFC_A {
        match self.bits {
            false => TRFC_A::NO,
            true => TRFC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == TRFC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == TRFC_A::YES
    }
}
impl core::ops::Deref for TRFC_R {
    type Target = crate::FieldReader<bool, TRFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRFC` writer - Transfer Complete"]
pub struct TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not complete"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(TRFC_A::NO)
    }
    #[doc = "Command execution is completed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(TRFC_A::YES)
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
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKGE_A {
    #[doc = "0: No Block Gap Event"]
    NO = 0,
    #[doc = "1: Transaction stopped at block gap"]
    STOP = 1,
}
impl From<BLKGE_A> for bool {
    #[inline(always)]
    fn from(variant: BLKGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKGE` reader - Block Gap Event"]
pub struct BLKGE_R(crate::FieldReader<bool, BLKGE_A>);
impl BLKGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLKGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLKGE_A {
        match self.bits {
            false => BLKGE_A::NO,
            true => BLKGE_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BLKGE_A::NO
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == BLKGE_A::STOP
    }
}
impl core::ops::Deref for BLKGE_R {
    type Target = crate::FieldReader<bool, BLKGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKGE` writer - Block Gap Event"]
pub struct BLKGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BLKGE_A::NO)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(BLKGE_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAINT_A {
    #[doc = "0: No DMA Interrupt"]
    NO = 0,
    #[doc = "1: DMA Interrupt is generated"]
    YES = 1,
}
impl From<DMAINT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINT` reader - DMA Interrupt"]
pub struct DMAINT_R(crate::FieldReader<bool, DMAINT_A>);
impl DMAINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAINT_A {
        match self.bits {
            false => DMAINT_A::NO,
            true => DMAINT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == DMAINT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == DMAINT_A::YES
    }
}
impl core::ops::Deref for DMAINT_R {
    type Target = crate::FieldReader<bool, DMAINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAINT` writer - DMA Interrupt"]
pub struct DMAINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(DMAINT_A::NO)
    }
    #[doc = "DMA Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(DMAINT_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRRDY_A {
    #[doc = "0: Not ready to write buffer"]
    NO = 0,
    #[doc = "1: Ready to write buffer"]
    YES = 1,
}
impl From<BWRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BWRRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWRRDY` reader - Buffer Write Ready"]
pub struct BWRRDY_R(crate::FieldReader<bool, BWRRDY_A>);
impl BWRRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BWRRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRRDY_A {
        match self.bits {
            false => BWRRDY_A::NO,
            true => BWRRDY_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BWRRDY_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BWRRDY_A::YES
    }
}
impl core::ops::Deref for BWRRDY_R {
    type Target = crate::FieldReader<bool, BWRRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWRRDY` writer - Buffer Write Ready"]
pub struct BWRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BWRRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWRRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BWRRDY_A::NO)
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BWRRDY_A::YES)
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
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDRDY_A {
    #[doc = "0: Not ready to read buffer"]
    NO = 0,
    #[doc = "1: Ready to read buffer"]
    YES = 1,
}
impl From<BRDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BRDRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRDRDY` reader - Buffer Read Ready"]
pub struct BRDRDY_R(crate::FieldReader<bool, BRDRDY_A>);
impl BRDRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRDRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDRDY_A {
        match self.bits {
            false => BRDRDY_A::NO,
            true => BRDRDY_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BRDRDY_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == BRDRDY_A::YES
    }
}
impl core::ops::Deref for BRDRDY_R {
    type Target = crate::FieldReader<bool, BRDRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRDRDY` writer - Buffer Read Ready"]
pub struct BRDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRDRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BRDRDY_A::NO)
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BRDRDY_A::YES)
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
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINS_A {
    #[doc = "0: Card state stable or Debouncing"]
    NO = 0,
    #[doc = "1: Card inserted"]
    YES = 1,
}
impl From<CINS_A> for bool {
    #[inline(always)]
    fn from(variant: CINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINS` reader - Card Insertion"]
pub struct CINS_R(crate::FieldReader<bool, CINS_A>);
impl CINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CINS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINS_A {
        match self.bits {
            false => CINS_A::NO,
            true => CINS_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == CINS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CINS_A::YES
    }
}
impl core::ops::Deref for CINS_R {
    type Target = crate::FieldReader<bool, CINS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CINS` writer - Card Insertion"]
pub struct CINS_W<'a> {
    w: &'a mut W,
}
impl<'a> CINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CINS_A::NO)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CINS_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CREM_A {
    #[doc = "0: Card state stable or Debouncing"]
    NO = 0,
    #[doc = "1: Card Removed"]
    YES = 1,
}
impl From<CREM_A> for bool {
    #[inline(always)]
    fn from(variant: CREM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREM` reader - Card Removal"]
pub struct CREM_R(crate::FieldReader<bool, CREM_A>);
impl CREM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CREM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREM_A {
        match self.bits {
            false => CREM_A::NO,
            true => CREM_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == CREM_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CREM_A::YES
    }
}
impl core::ops::Deref for CREM_R {
    type Target = crate::FieldReader<bool, CREM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CREM` writer - Card Removal"]
pub struct CREM_W<'a> {
    w: &'a mut W,
}
impl<'a> CREM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CREM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card state stable or Debouncing"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CREM_A::NO)
    }
    #[doc = "Card Removed"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CREM_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINT_A {
    #[doc = "0: No Card Interrupt"]
    NO = 0,
    #[doc = "1: Generate Card Interrupt"]
    YES = 1,
}
impl From<CINT_A> for bool {
    #[inline(always)]
    fn from(variant: CINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CINT` reader - Card Interrupt"]
pub struct CINT_R(crate::FieldReader<bool, CINT_A>);
impl CINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINT_A {
        match self.bits {
            false => CINT_A::NO,
            true => CINT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == CINT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CINT_A::YES
    }
}
impl core::ops::Deref for CINT_R {
    type Target = crate::FieldReader<bool, CINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CINT` writer - Card Interrupt"]
pub struct CINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CINT_A::NO)
    }
    #[doc = "Generate Card Interrupt"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CINT_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_A {
    #[doc = "0: No Error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub struct ERRINT_R(crate::FieldReader<bool, ERRINT_A>);
impl ERRINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::NO,
            true => ERRINT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == ERRINT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == ERRINT_A::YES
    }
}
impl core::ops::Deref for ERRINT_R {
    type Target = crate::FieldReader<bool, ERRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub struct ERRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ERRINT_A::NO)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ERRINT_A::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdc(&self) -> CMDC_R {
        CMDC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkge(&self) -> BLKGE_R {
        BLKGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwrrdy(&self) -> BWRRDY_R {
        BWRRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brdrdy(&self) -> BRDRDY_R {
        BRDRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crem(&self) -> CREM_R {
        CREM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdc(&mut self) -> CMDC_W {
        CMDC_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W {
        TRFC_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkge(&mut self) -> BLKGE_W {
        BLKGE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&mut self) -> DMAINT_W {
        DMAINT_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwrrdy(&mut self) -> BWRRDY_W {
        BWRRDY_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brdrdy(&mut self) -> BRDRDY_W {
        BRDRDY_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&mut self) -> CINS_W {
        CINS_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crem(&mut self) -> CREM_W {
        CREM_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W { w: self }
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ERRINT_W {
        ERRINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nistr](index.html) module"]
pub struct NISTR_SPEC;
impl crate::RegisterSpec for NISTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nistr::R](R) reader structure"]
impl crate::Readable for NISTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nistr::W](W) writer structure"]
impl crate::Writable for NISTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NISTR to value 0"]
impl crate::Resettable for NISTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
