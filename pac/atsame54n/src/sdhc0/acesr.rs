#[doc = "Register `ACESR` reader"]
pub struct R(crate::R<ACESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACMD12NE` reader - Auto CMD12 Not Executed"]
pub type ACMD12NE_R = crate::BitReader<ACMD12NESELECT_A>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMD12NESELECT_A {
    #[doc = "0: Executed"]
    EXEC = 0,
    #[doc = "1: Not executed"]
    NOT_EXEC = 1,
}
impl From<ACMD12NESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD12NESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMD12NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD12NESELECT_A {
        match self.bits {
            false => ACMD12NESELECT_A::EXEC,
            true => ACMD12NESELECT_A::NOT_EXEC,
        }
    }
    #[doc = "Checks if the value of the field is `EXEC`"]
    #[inline(always)]
    pub fn is_exec(&self) -> bool {
        *self == ACMD12NESELECT_A::EXEC
    }
    #[doc = "Checks if the value of the field is `NOT_EXEC`"]
    #[inline(always)]
    pub fn is_not_exec(&self) -> bool {
        *self == ACMD12NESELECT_A::NOT_EXEC
    }
}
#[doc = "Field `ACMDTEO` reader - Auto CMD Timeout Error"]
pub type ACMDTEO_R = crate::BitReader<ACMDTEOSELECT_A>;
#[doc = "Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDTEOSELECT_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: Timeout"]
    YES = 1,
}
impl From<ACMDTEOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDTEOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMDTEO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDTEOSELECT_A {
        match self.bits {
            false => ACMDTEOSELECT_A::NO,
            true => ACMDTEOSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDTEOSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDTEOSELECT_A::YES
    }
}
#[doc = "Field `ACMDCRC` reader - Auto CMD CRC Error"]
pub type ACMDCRC_R = crate::BitReader<ACMDCRCSELECT_A>;
#[doc = "Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDCRCSELECT_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: CRC Error Generated"]
    YES = 1,
}
impl From<ACMDCRCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDCRCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMDCRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDCRCSELECT_A {
        match self.bits {
            false => ACMDCRCSELECT_A::NO,
            true => ACMDCRCSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDCRCSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDCRCSELECT_A::YES
    }
}
#[doc = "Field `ACMDEND` reader - Auto CMD End Bit Error"]
pub type ACMDEND_R = crate::BitReader<ACMDENDSELECT_A>;
#[doc = "Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDENDSELECT_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: End Bit Error Generated"]
    YES = 1,
}
impl From<ACMDENDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDENDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMDEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDENDSELECT_A {
        match self.bits {
            false => ACMDENDSELECT_A::NO,
            true => ACMDENDSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDENDSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDENDSELECT_A::YES
    }
}
#[doc = "Field `ACMDIDX` reader - Auto CMD Index Error"]
pub type ACMDIDX_R = crate::BitReader<ACMDIDXSELECT_A>;
#[doc = "Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMDIDXSELECT_A {
    #[doc = "0: No error"]
    NO = 0,
    #[doc = "1: Error"]
    YES = 1,
}
impl From<ACMDIDXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACMDIDXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMDIDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMDIDXSELECT_A {
        match self.bits {
            false => ACMDIDXSELECT_A::NO,
            true => ACMDIDXSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACMDIDXSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == ACMDIDXSELECT_A::YES
    }
}
#[doc = "Field `CMDNI` reader - Command not Issued By Auto CMD12 Error"]
pub type CMDNI_R = crate::BitReader<CMDNISELECT_A>;
#[doc = "Command not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDNISELECT_A {
    #[doc = "0: No error"]
    OK = 0,
    #[doc = "1: Not Issued"]
    NOT_ISSUED = 1,
}
impl From<CMDNISELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDNISELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDNI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDNISELECT_A {
        match self.bits {
            false => CMDNISELECT_A::OK,
            true => CMDNISELECT_A::NOT_ISSUED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == CMDNISELECT_A::OK
    }
    #[doc = "Checks if the value of the field is `NOT_ISSUED`"]
    #[inline(always)]
    pub fn is_not_issued(&self) -> bool {
        *self == CMDNISELECT_A::NOT_ISSUED
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12ne(&self) -> ACMD12NE_R {
        ACMD12NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmdteo(&self) -> ACMDTEO_R {
        ACMDTEO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmdcrc(&self) -> ACMDCRC_R {
        ACMDCRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmdend(&self) -> ACMDEND_R {
        ACMDEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmdidx(&self) -> ACMDIDX_R {
        ACMDIDX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmdni(&self) -> CMDNI_R {
        CMDNI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Auto CMD Error Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acesr](index.html) module"]
pub struct ACESR_SPEC;
impl crate::RegisterSpec for ACESR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [acesr::R](R) reader structure"]
impl crate::Readable for ACESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACESR to value 0"]
impl crate::Resettable for ACESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
