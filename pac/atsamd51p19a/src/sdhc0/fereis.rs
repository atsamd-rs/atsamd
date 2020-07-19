#[doc = "Writer for register FEREIS"]
pub type W = crate::W<u16, super::FEREIS>;
#[doc = "Register FEREIS `reset()`'s with value 0"]
impl crate::ResetValue for super::FEREIS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTEO_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDTEO_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDTEO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CMDTEO`"]
pub struct CMDTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTEO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDTEO_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDTEO_AW::YES)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDCRC_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDCRC_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDCRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CMDCRC`"]
pub struct CMDCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDCRC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDCRC_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDCRC_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEND_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDEND_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CMDEND`"]
pub struct CMDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDEND_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDEND_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDIDX_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CMDIDX_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDIDX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CMDIDX`"]
pub struct CMDIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDIDX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDIDX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CMDIDX_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CMDIDX_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATTEO_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATTEO_AW> for bool {
    #[inline(always)]
    fn from(variant: DATTEO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DATTEO`"]
pub struct DATTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTEO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATTEO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(DATTEO_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATTEO_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATCRC_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATCRC_AW> for bool {
    #[inline(always)]
    fn from(variant: DATCRC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DATCRC`"]
pub struct DATCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATCRC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(DATCRC_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATCRC_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATEND_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<DATEND_AW> for bool {
    #[inline(always)]
    fn from(variant: DATEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DATEND`"]
pub struct DATEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DATEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(DATEND_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(DATEND_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURLIM_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<CURLIM_AW> for bool {
    #[inline(always)]
    fn from(variant: CURLIM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CURLIM`"]
pub struct CURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CURLIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CURLIM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CURLIM_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CURLIM_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ACMD_AW> for bool {
    #[inline(always)]
    fn from(variant: ACMD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ACMD`"]
pub struct ACMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACMD_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ACMD_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Force Event for ADMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMA_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<ADMA_AW> for bool {
    #[inline(always)]
    fn from(variant: ADMA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADMA`"]
pub struct ADMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADMA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ADMA_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(ADMA_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Force Event for Boot Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTAE_AW {
    #[doc = "0: No Interrupt"]
    NO = 0,
    #[doc = "1: Interrupt is generated"]
    YES = 1,
}
impl From<BOOTAE_AW> for bool {
    #[inline(always)]
    fn from(variant: BOOTAE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BOOTAE`"]
pub struct BOOTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOTAE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(BOOTAE_AW::NO)
    }
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(BOOTAE_AW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdteo(&mut self) -> CMDTEO_W {
        CMDTEO_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrc(&mut self) -> CMDCRC_W {
        CMDCRC_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdend(&mut self) -> CMDEND_W {
        CMDEND_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W {
        CMDIDX_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn datteo(&mut self) -> DATTEO_W {
        DATTEO_W { w: self }
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrc(&mut self) -> DATCRC_W {
        DATCRC_W { w: self }
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datend(&mut self) -> DATEND_W {
        DATEND_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlim(&mut self) -> CURLIM_W {
        CURLIM_W { w: self }
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn acmd(&mut self) -> ACMD_W {
        ACMD_W { w: self }
    }
    #[doc = "Bit 9 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn adma(&mut self) -> ADMA_W {
        ADMA_W { w: self }
    }
    #[doc = "Bit 12 - Force Event for Boot Acknowledge Error"]
    #[inline(always)]
    pub fn bootae(&mut self) -> BOOTAE_W {
        BOOTAE_W { w: self }
    }
}
