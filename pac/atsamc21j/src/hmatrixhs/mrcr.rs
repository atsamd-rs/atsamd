#[doc = "Register `MRCR` reader"]
pub struct R(crate::R<MRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRCR` writer"]
pub struct W(crate::W<MRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRCR_SPEC>;
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
impl From<crate::W<MRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Remap Command Bit for Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB0_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB0_A> for bool {
    #[inline(always)]
    fn from(variant: RCB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub struct RCB0_R(crate::FieldReader<bool, RCB0_A>);
impl RCB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB0_A {
        match self.bits {
            false => RCB0_A::DIS,
            true => RCB0_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB0_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB0_A::ENA
    }
}
impl core::ops::Deref for RCB0_R {
    type Target = crate::FieldReader<bool, RCB0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub struct RCB0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB0_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB0_A::ENA)
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
#[doc = "Remap Command Bit for Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB1_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB1_A> for bool {
    #[inline(always)]
    fn from(variant: RCB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub struct RCB1_R(crate::FieldReader<bool, RCB1_A>);
impl RCB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB1_A {
        match self.bits {
            false => RCB1_A::DIS,
            true => RCB1_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB1_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB1_A::ENA
    }
}
impl core::ops::Deref for RCB1_R {
    type Target = crate::FieldReader<bool, RCB1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub struct RCB1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB1_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB1_A::ENA)
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
#[doc = "Remap Command Bit for Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB2_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB2_A> for bool {
    #[inline(always)]
    fn from(variant: RCB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub struct RCB2_R(crate::FieldReader<bool, RCB2_A>);
impl RCB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB2_A {
        match self.bits {
            false => RCB2_A::DIS,
            true => RCB2_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB2_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB2_A::ENA
    }
}
impl core::ops::Deref for RCB2_R {
    type Target = crate::FieldReader<bool, RCB2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub struct RCB2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB2_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB2_A::ENA)
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
#[doc = "Remap Command Bit for Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB3_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB3_A> for bool {
    #[inline(always)]
    fn from(variant: RCB3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub struct RCB3_R(crate::FieldReader<bool, RCB3_A>);
impl RCB3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB3_A {
        match self.bits {
            false => RCB3_A::DIS,
            true => RCB3_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB3_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB3_A::ENA
    }
}
impl core::ops::Deref for RCB3_R {
    type Target = crate::FieldReader<bool, RCB3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub struct RCB3_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB3_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB3_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB4_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB4_A> for bool {
    #[inline(always)]
    fn from(variant: RCB4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub struct RCB4_R(crate::FieldReader<bool, RCB4_A>);
impl RCB4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB4_A {
        match self.bits {
            false => RCB4_A::DIS,
            true => RCB4_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB4_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB4_A::ENA
    }
}
impl core::ops::Deref for RCB4_R {
    type Target = crate::FieldReader<bool, RCB4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub struct RCB4_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB4_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB4_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB5_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB5_A> for bool {
    #[inline(always)]
    fn from(variant: RCB5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub struct RCB5_R(crate::FieldReader<bool, RCB5_A>);
impl RCB5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB5_A {
        match self.bits {
            false => RCB5_A::DIS,
            true => RCB5_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB5_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB5_A::ENA
    }
}
impl core::ops::Deref for RCB5_R {
    type Target = crate::FieldReader<bool, RCB5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub struct RCB5_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB5_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB5_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB6_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB6_A> for bool {
    #[inline(always)]
    fn from(variant: RCB6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB6` reader - Remap Command Bit for Master 6"]
pub struct RCB6_R(crate::FieldReader<bool, RCB6_A>);
impl RCB6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB6_A {
        match self.bits {
            false => RCB6_A::DIS,
            true => RCB6_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB6_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB6_A::ENA
    }
}
impl core::ops::Deref for RCB6_R {
    type Target = crate::FieldReader<bool, RCB6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB6` writer - Remap Command Bit for Master 6"]
pub struct RCB6_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB6_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB6_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB7_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB7_A> for bool {
    #[inline(always)]
    fn from(variant: RCB7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB7` reader - Remap Command Bit for Master 7"]
pub struct RCB7_R(crate::FieldReader<bool, RCB7_A>);
impl RCB7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB7_A {
        match self.bits {
            false => RCB7_A::DIS,
            true => RCB7_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB7_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB7_A::ENA
    }
}
impl core::ops::Deref for RCB7_R {
    type Target = crate::FieldReader<bool, RCB7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB7` writer - Remap Command Bit for Master 7"]
pub struct RCB7_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB7_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB7_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB8_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB8_A> for bool {
    #[inline(always)]
    fn from(variant: RCB8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB8` reader - Remap Command Bit for Master 8"]
pub struct RCB8_R(crate::FieldReader<bool, RCB8_A>);
impl RCB8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB8_A {
        match self.bits {
            false => RCB8_A::DIS,
            true => RCB8_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB8_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB8_A::ENA
    }
}
impl core::ops::Deref for RCB8_R {
    type Target = crate::FieldReader<bool, RCB8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB8` writer - Remap Command Bit for Master 8"]
pub struct RCB8_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB8_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB8_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB9_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB9_A> for bool {
    #[inline(always)]
    fn from(variant: RCB9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB9` reader - Remap Command Bit for Master 9"]
pub struct RCB9_R(crate::FieldReader<bool, RCB9_A>);
impl RCB9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB9_A {
        match self.bits {
            false => RCB9_A::DIS,
            true => RCB9_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB9_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB9_A::ENA
    }
}
impl core::ops::Deref for RCB9_R {
    type Target = crate::FieldReader<bool, RCB9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB9` writer - Remap Command Bit for Master 9"]
pub struct RCB9_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB9_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB9_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB10_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB10_A> for bool {
    #[inline(always)]
    fn from(variant: RCB10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB10` reader - Remap Command Bit for Master 10"]
pub struct RCB10_R(crate::FieldReader<bool, RCB10_A>);
impl RCB10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB10_A {
        match self.bits {
            false => RCB10_A::DIS,
            true => RCB10_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB10_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB10_A::ENA
    }
}
impl core::ops::Deref for RCB10_R {
    type Target = crate::FieldReader<bool, RCB10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB10` writer - Remap Command Bit for Master 10"]
pub struct RCB10_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB10_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB10_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB11_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB11_A> for bool {
    #[inline(always)]
    fn from(variant: RCB11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB11` reader - Remap Command Bit for Master 11"]
pub struct RCB11_R(crate::FieldReader<bool, RCB11_A>);
impl RCB11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB11_A {
        match self.bits {
            false => RCB11_A::DIS,
            true => RCB11_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB11_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB11_A::ENA
    }
}
impl core::ops::Deref for RCB11_R {
    type Target = crate::FieldReader<bool, RCB11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB11` writer - Remap Command Bit for Master 11"]
pub struct RCB11_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB11_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB11_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB12_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB12_A> for bool {
    #[inline(always)]
    fn from(variant: RCB12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB12` reader - Remap Command Bit for Master 12"]
pub struct RCB12_R(crate::FieldReader<bool, RCB12_A>);
impl RCB12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB12_A {
        match self.bits {
            false => RCB12_A::DIS,
            true => RCB12_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB12_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB12_A::ENA
    }
}
impl core::ops::Deref for RCB12_R {
    type Target = crate::FieldReader<bool, RCB12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB12` writer - Remap Command Bit for Master 12"]
pub struct RCB12_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB12_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB12_A::ENA)
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
#[doc = "Remap Command Bit for Master 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB13_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB13_A> for bool {
    #[inline(always)]
    fn from(variant: RCB13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB13` reader - Remap Command Bit for Master 13"]
pub struct RCB13_R(crate::FieldReader<bool, RCB13_A>);
impl RCB13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB13_A {
        match self.bits {
            false => RCB13_A::DIS,
            true => RCB13_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB13_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB13_A::ENA
    }
}
impl core::ops::Deref for RCB13_R {
    type Target = crate::FieldReader<bool, RCB13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB13` writer - Remap Command Bit for Master 13"]
pub struct RCB13_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB13_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB13_A::ENA)
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
#[doc = "Remap Command Bit for Master 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB14_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB14_A> for bool {
    #[inline(always)]
    fn from(variant: RCB14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB14` reader - Remap Command Bit for Master 14"]
pub struct RCB14_R(crate::FieldReader<bool, RCB14_A>);
impl RCB14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB14_A {
        match self.bits {
            false => RCB14_A::DIS,
            true => RCB14_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB14_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB14_A::ENA
    }
}
impl core::ops::Deref for RCB14_R {
    type Target = crate::FieldReader<bool, RCB14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB14` writer - Remap Command Bit for Master 14"]
pub struct RCB14_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB14_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB14_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Remap Command Bit for Master 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB15_A {
    #[doc = "0: Disable remapped address decoding for master"]
    DIS = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    ENA = 1,
}
impl From<RCB15_A> for bool {
    #[inline(always)]
    fn from(variant: RCB15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB15` reader - Remap Command Bit for Master 15"]
pub struct RCB15_R(crate::FieldReader<bool, RCB15_A>);
impl RCB15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCB15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB15_A {
        match self.bits {
            false => RCB15_A::DIS,
            true => RCB15_A::ENA,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RCB15_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENA`"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        **self == RCB15_A::ENA
    }
}
impl core::ops::Deref for RCB15_R {
    type Target = crate::FieldReader<bool, RCB15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB15` writer - Remap Command Bit for Master 15"]
pub struct RCB15_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCB15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RCB15_A::DIS)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut W {
        self.variant(RCB15_A::ENA)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> RCB0_R {
        RCB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> RCB1_R {
        RCB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> RCB2_R {
        RCB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> RCB3_R {
        RCB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> RCB4_R {
        RCB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> RCB5_R {
        RCB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> RCB6_R {
        RCB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&self) -> RCB7_R {
        RCB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> RCB8_R {
        RCB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> RCB9_R {
        RCB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> RCB10_R {
        RCB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> RCB11_R {
        RCB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> RCB12_R {
        RCB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Remap Command Bit for Master 13"]
    #[inline(always)]
    pub fn rcb13(&self) -> RCB13_R {
        RCB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Remap Command Bit for Master 14"]
    #[inline(always)]
    pub fn rcb14(&self) -> RCB14_R {
        RCB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Remap Command Bit for Master 15"]
    #[inline(always)]
    pub fn rcb15(&self) -> RCB15_R {
        RCB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&mut self) -> RCB0_W {
        RCB0_W { w: self }
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&mut self) -> RCB1_W {
        RCB1_W { w: self }
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&mut self) -> RCB2_W {
        RCB2_W { w: self }
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&mut self) -> RCB3_W {
        RCB3_W { w: self }
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&mut self) -> RCB4_W {
        RCB4_W { w: self }
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&mut self) -> RCB5_W {
        RCB5_W { w: self }
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&mut self) -> RCB6_W {
        RCB6_W { w: self }
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&mut self) -> RCB7_W {
        RCB7_W { w: self }
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&mut self) -> RCB8_W {
        RCB8_W { w: self }
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&mut self) -> RCB9_W {
        RCB9_W { w: self }
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&mut self) -> RCB10_W {
        RCB10_W { w: self }
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&mut self) -> RCB11_W {
        RCB11_W { w: self }
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&mut self) -> RCB12_W {
        RCB12_W { w: self }
    }
    #[doc = "Bit 13 - Remap Command Bit for Master 13"]
    #[inline(always)]
    pub fn rcb13(&mut self) -> RCB13_W {
        RCB13_W { w: self }
    }
    #[doc = "Bit 14 - Remap Command Bit for Master 14"]
    #[inline(always)]
    pub fn rcb14(&mut self) -> RCB14_W {
        RCB14_W { w: self }
    }
    #[doc = "Bit 15 - Remap Command Bit for Master 15"]
    #[inline(always)]
    pub fn rcb15(&mut self) -> RCB15_W {
        RCB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Remap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcr](index.html) module"]
pub struct MRCR_SPEC;
impl crate::RegisterSpec for MRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrcr::R](R) reader structure"]
impl crate::Readable for MRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrcr::W](W) writer structure"]
impl crate::Writable for MRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
