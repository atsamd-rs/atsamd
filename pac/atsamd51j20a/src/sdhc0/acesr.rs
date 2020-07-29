#[doc = "Reader of register ACESR"]
pub type R = crate::R<u16, super::ACESR>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD12NE_A {
    #[doc = "0: Executed"]
    EXEC = 0,
    #[doc = "1: Not executed"]
    NOT_EXEC = 1,
}
impl From<ACMD12NE_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD12NE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMD12NE`"]
pub type ACMD12NE_R = crate::R<bool, ACMD12NE_A>;
impl ACMD12NE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD12NE_A {
        match self.bits {
            false => ACMD12NE_A::EXEC,
            true => ACMD12NE_A::NOT_EXEC,
        }
    }
    #[doc = "Checks if the value of the field is `EXEC`"]
    #[inline(always)]
    pub fn is_exec(&self) -> bool {
        *self == ACMD12NE_A::EXEC
    }
    #[doc = "Checks if the value of the field is `NOT_EXEC`"]
    #[inline(always)]
    pub fn is_not_exec(&self) -> bool {
        *self == ACMD12NE_A::NOT_EXEC
    }
}
#[doc = "Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDTEO_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: Timeout"]
    YES = 1,
}
impl From<ACMDTEO_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDTEO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMDTEO`"]
pub type ACMDTEO_R = crate::R<bool, ACMDTEO_A>;
impl ACMDTEO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDTEO_A {
        match self.bits {
            false => ACMDTEO_A::NO,
            true => ACMDTEO_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDTEO_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDTEO_A::YES
    }
}
#[doc = "Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDCRC_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: CRC Error Generated"]
    YES = 1,
}
impl From<ACMDCRC_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDCRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMDCRC`"]
pub type ACMDCRC_R = crate::R<bool, ACMDCRC_A>;
impl ACMDCRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDCRC_A {
        match self.bits {
            false => ACMDCRC_A::NO,
            true => ACMDCRC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDCRC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDCRC_A::YES
    }
}
#[doc = "Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDEND_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: End Bit Error Generated"]
    YES = 1,
}
impl From<ACMDEND_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMDEND`"]
pub type ACMDEND_R = crate::R<bool, ACMDEND_A>;
impl ACMDEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDEND_A {
        match self.bits {
            false => ACMDEND_A::NO,
            true => ACMDEND_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDEND_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDEND_A::YES
    }
}
#[doc = "Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMDIDX_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<ACMDIDX_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDIDX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMDIDX`"]
pub type ACMDIDX_R = crate::R<bool, ACMDIDX_A>;
impl ACMDIDX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDIDX_A {
        match self.bits {
            false => ACMDIDX_A::NO,
            true => ACMDIDX_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDIDX_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDIDX_A::YES
    }
}
#[doc = "Command not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDNI_A {
    #[doc = "0: No error"]
    OK = 0,
    #[doc = "1: Not Issued"]
    NOT_ISSUED = 1,
}
impl From<CMDNI_A> for bool {
    #[inline(always)]
    fn from(variant: CMDNI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDNI`"]
pub type CMDNI_R = crate::R<bool, CMDNI_A>;
impl CMDNI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDNI_A {
        match self.bits {
            false => CMDNI_A::OK,
            true => CMDNI_A::NOT_ISSUED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == CMDNI_A::OK
    }
    #[doc = "Checks if the value of the field is `NOT_ISSUED`"]
    #[inline(always)]
    pub fn is_not_issued(&self) -> bool {
        *self == CMDNI_A::NOT_ISSUED
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12ne(&self) -> ACMD12NE_R {
        ACMD12NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmdteo(&self) -> ACMDTEO_R {
        ACMDTEO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmdcrc(&self) -> ACMDCRC_R {
        ACMDCRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmdend(&self) -> ACMDEND_R {
        ACMDEND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmdidx(&self) -> ACMDIDX_R {
        ACMDIDX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmdni(&self) -> CMDNI_R {
        CMDNI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
