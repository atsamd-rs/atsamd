#[doc = "Register `ACC_MR` reader"]
pub struct R(crate::R<ACC_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACC_MR` writer"]
pub struct W(crate::W<ACC_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACC_MR_SPEC>;
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
impl From<crate::W<ACC_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACC_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selection for Minus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELMINUS_A {
    #[doc = "0: Select TS"]
    TS = 0,
    #[doc = "1: Select VREFP"]
    VREFP = 1,
    #[doc = "2: Select DAC0"]
    DAC0 = 2,
    #[doc = "3: Select DAC1"]
    DAC1 = 3,
    #[doc = "4: Select AFE0_AD0"]
    AFE0_AD0 = 4,
    #[doc = "5: Select AFE0_AD1"]
    AFE0_AD1 = 5,
    #[doc = "6: Select AFE0_AD2"]
    AFE0_AD2 = 6,
    #[doc = "7: Select AFE0_AD3"]
    AFE0_AD3 = 7,
}
impl From<SELMINUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMINUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELMINUS` reader - Selection for Minus Comparator Input"]
pub struct SELMINUS_R(crate::FieldReader<u8, SELMINUS_A>);
impl SELMINUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELMINUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMINUS_A {
        match self.bits {
            0 => SELMINUS_A::TS,
            1 => SELMINUS_A::VREFP,
            2 => SELMINUS_A::DAC0,
            3 => SELMINUS_A::DAC1,
            4 => SELMINUS_A::AFE0_AD0,
            5 => SELMINUS_A::AFE0_AD1,
            6 => SELMINUS_A::AFE0_AD2,
            7 => SELMINUS_A::AFE0_AD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == SELMINUS_A::TS
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        **self == SELMINUS_A::VREFP
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        **self == SELMINUS_A::DAC0
    }
    #[doc = "Checks if the value of the field is `DAC1`"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        **self == SELMINUS_A::DAC1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        **self == SELMINUS_A::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        **self == SELMINUS_A::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        **self == SELMINUS_A::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        **self == SELMINUS_A::AFE0_AD3
    }
}
impl core::ops::Deref for SELMINUS_R {
    type Target = crate::FieldReader<u8, SELMINUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELMINUS` writer - Selection for Minus Comparator Input"]
pub struct SELMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELMINUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMINUS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select TS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUS_A::TS)
    }
    #[doc = "Select VREFP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(SELMINUS_A::VREFP)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUS_A::DAC0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUS_A::DAC1)
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELMINUS_A::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELMINUS_A::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELMINUS_A::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELMINUS_A::AFE0_AD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Selection For Plus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELPLUS_A {
    #[doc = "0: Select AFE0_AD0"]
    AFE0_AD0 = 0,
    #[doc = "1: Select AFE0_AD1"]
    AFE0_AD1 = 1,
    #[doc = "2: Select AFE0_AD2"]
    AFE0_AD2 = 2,
    #[doc = "3: Select AFE0_AD3"]
    AFE0_AD3 = 3,
    #[doc = "4: Select AFE0_AD4"]
    AFE0_AD4 = 4,
    #[doc = "5: Select AFE0_AD5"]
    AFE0_AD5 = 5,
    #[doc = "6: Select AFE1_AD0"]
    AFE1_AD0 = 6,
    #[doc = "7: Select AFE1_AD1"]
    AFE1_AD1 = 7,
}
impl From<SELPLUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELPLUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELPLUS` reader - Selection For Plus Comparator Input"]
pub struct SELPLUS_R(crate::FieldReader<u8, SELPLUS_A>);
impl SELPLUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELPLUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPLUS_A {
        match self.bits {
            0 => SELPLUS_A::AFE0_AD0,
            1 => SELPLUS_A::AFE0_AD1,
            2 => SELPLUS_A::AFE0_AD2,
            3 => SELPLUS_A::AFE0_AD3,
            4 => SELPLUS_A::AFE0_AD4,
            5 => SELPLUS_A::AFE0_AD5,
            6 => SELPLUS_A::AFE1_AD0,
            7 => SELPLUS_A::AFE1_AD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD3
    }
    #[doc = "Checks if the value of the field is `AFE0_AD4`"]
    #[inline(always)]
    pub fn is_afe0_ad4(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD4
    }
    #[doc = "Checks if the value of the field is `AFE0_AD5`"]
    #[inline(always)]
    pub fn is_afe0_ad5(&self) -> bool {
        **self == SELPLUS_A::AFE0_AD5
    }
    #[doc = "Checks if the value of the field is `AFE1_AD0`"]
    #[inline(always)]
    pub fn is_afe1_ad0(&self) -> bool {
        **self == SELPLUS_A::AFE1_AD0
    }
    #[doc = "Checks if the value of the field is `AFE1_AD1`"]
    #[inline(always)]
    pub fn is_afe1_ad1(&self) -> bool {
        **self == SELPLUS_A::AFE1_AD1
    }
}
impl core::ops::Deref for SELPLUS_R {
    type Target = crate::FieldReader<u8, SELPLUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELPLUS` writer - Selection For Plus Comparator Input"]
pub struct SELPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELPLUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELPLUS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD3)
    }
    #[doc = "Select AFE0_AD4"]
    #[inline(always)]
    pub fn afe0_ad4(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD4)
    }
    #[doc = "Select AFE0_AD5"]
    #[inline(always)]
    pub fn afe0_ad5(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE0_AD5)
    }
    #[doc = "Select AFE1_AD0"]
    #[inline(always)]
    pub fn afe1_ad0(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE1_AD0)
    }
    #[doc = "Select AFE1_AD1"]
    #[inline(always)]
    pub fn afe1_ad1(self) -> &'a mut W {
        self.variant(SELPLUS_A::AFE1_AD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Analog Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACEN_A {
    #[doc = "0: Analog comparator disabled."]
    DIS = 0,
    #[doc = "1: Analog comparator enabled."]
    EN = 1,
}
impl From<ACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator Enable"]
pub struct ACEN_R(crate::FieldReader<bool, ACEN_A>);
impl ACEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEN_A {
        match self.bits {
            false => ACEN_A::DIS,
            true => ACEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ACEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ACEN_A::EN
    }
}
impl core::ops::Deref for ACEN_R {
    type Target = crate::FieldReader<bool, ACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator Enable"]
pub struct ACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog comparator disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACEN_A::DIS)
    }
    #[doc = "Analog comparator enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACEN_A::EN)
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
#[doc = "Edge Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGETYP_A {
    #[doc = "0: Only rising edge of comparator output"]
    RISING = 0,
    #[doc = "1: Falling edge of comparator output"]
    FALLING = 1,
    #[doc = "2: Any edge of comparator output"]
    ANY = 2,
}
impl From<EDGETYP_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGETYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDGETYP` reader - Edge Type"]
pub struct EDGETYP_R(crate::FieldReader<u8, EDGETYP_A>);
impl EDGETYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDGETYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EDGETYP_A> {
        match self.bits {
            0 => Some(EDGETYP_A::RISING),
            1 => Some(EDGETYP_A::FALLING),
            2 => Some(EDGETYP_A::ANY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == EDGETYP_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == EDGETYP_A::FALLING
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        **self == EDGETYP_A::ANY
    }
}
impl core::ops::Deref for EDGETYP_R {
    type Target = crate::FieldReader<u8, EDGETYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGETYP` writer - Edge Type"]
pub struct EDGETYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGETYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGETYP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYP_A::RISING)
    }
    #[doc = "Falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYP_A::FALLING)
    }
    #[doc = "Any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYP_A::ANY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Invert Comparator Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Analog comparator output is directly processed."]
    DIS = 0,
    #[doc = "1: Analog comparator output is inverted prior to being processed."]
    EN = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert Comparator Output"]
pub struct INV_R(crate::FieldReader<bool, INV_A>);
impl INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::DIS,
            true => INV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == INV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == INV_A::EN
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - Invert Comparator Output"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INV_A::DIS)
    }
    #[doc = "Analog comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INV_A::EN)
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
#[doc = "Selection Of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELFS_A {
    #[doc = "0: The CE flag is used to drive the FAULT output."]
    CE = 0,
    #[doc = "1: The output of the analog comparator flag is used to drive the FAULT output."]
    OUTPUT = 1,
}
impl From<SELFS_A> for bool {
    #[inline(always)]
    fn from(variant: SELFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELFS` reader - Selection Of Fault Source"]
pub struct SELFS_R(crate::FieldReader<bool, SELFS_A>);
impl SELFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SELFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELFS_A {
        match self.bits {
            false => SELFS_A::CE,
            true => SELFS_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CE`"]
    #[inline(always)]
    pub fn is_ce(&self) -> bool {
        **self == SELFS_A::CE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == SELFS_A::OUTPUT
    }
}
impl core::ops::Deref for SELFS_R {
    type Target = crate::FieldReader<bool, SELFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFS` writer - Selection Of Fault Source"]
pub struct SELFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CE flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn ce(self) -> &'a mut W {
        self.variant(SELFS_A::CE)
    }
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFS_A::OUTPUT)
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
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: The FAULT output is tied to 0."]
    DIS = 0,
    #[doc = "1: The FAULT output is driven by the signal defined by SELFS."]
    EN = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub struct FE_R(crate::FieldReader<bool, FE_A>);
impl FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::DIS,
            true => FE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FE_A::EN
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FE_A::DIS)
    }
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FE_A::EN)
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
impl R {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&self) -> SELMINUS_R {
        SELMINUS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&self) -> SELPLUS_R {
        SELPLUS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EDGETYP_R {
        EDGETYP_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SELFS_R {
        SELFS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&mut self) -> SELMINUS_W {
        SELMINUS_W { w: self }
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&mut self) -> SELPLUS_W {
        SELPLUS_W { w: self }
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&mut self) -> ACEN_W {
        ACEN_W { w: self }
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&mut self) -> EDGETYP_W {
        EDGETYP_W { w: self }
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&mut self) -> SELFS_W {
        SELFS_W { w: self }
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_mr](index.html) module"]
pub struct ACC_MR_SPEC;
impl crate::RegisterSpec for ACC_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc_mr::R](R) reader structure"]
impl crate::Readable for ACC_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acc_mr::W](W) writer structure"]
impl crate::Writable for ACC_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACC_MR to value 0"]
impl crate::Resettable for ACC_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
