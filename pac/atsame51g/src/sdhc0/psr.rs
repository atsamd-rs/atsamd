#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDINHC` reader - Command Inhibit (CMD)"]
pub type CMDINHC_R = crate::BitReader<CMDINHCSELECT_A>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDINHCSELECT_A {
    #[doc = "0: Can issue command using only CMD line"]
    CAN = 0,
    #[doc = "1: Cannot issue command"]
    CANNOT = 1,
}
impl From<CMDINHCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDINHCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDINHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDINHCSELECT_A {
        match self.bits {
            false => CMDINHCSELECT_A::CAN,
            true => CMDINHCSELECT_A::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == CMDINHCSELECT_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHCSELECT_A::CANNOT
    }
}
#[doc = "Field `CMDINHD` reader - Command Inhibit (DAT)"]
pub type CMDINHD_R = crate::BitReader<CMDINHDSELECT_A>;
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDINHDSELECT_A {
    #[doc = "0: Can issue command which uses the DAT line"]
    CAN = 0,
    #[doc = "1: Cannot issue command which uses the DAT line"]
    CANNOT = 1,
}
impl From<CMDINHDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDINHDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDINHD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDINHDSELECT_A {
        match self.bits {
            false => CMDINHDSELECT_A::CAN,
            true => CMDINHDSELECT_A::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == CMDINHDSELECT_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHDSELECT_A::CANNOT
    }
}
#[doc = "Field `DLACT` reader - DAT Line Active"]
pub type DLACT_R = crate::BitReader<DLACTSELECT_A>;
#[doc = "DAT Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLACTSELECT_A {
    #[doc = "0: DAT Line Inactive"]
    INACTIVE = 0,
    #[doc = "1: DAT Line Active"]
    ACTIVE = 1,
}
impl From<DLACTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DLACTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DLACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLACTSELECT_A {
        match self.bits {
            false => DLACTSELECT_A::INACTIVE,
            true => DLACTSELECT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DLACTSELECT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLACTSELECT_A::ACTIVE
    }
}
#[doc = "Field `RTREQ` reader - Re-Tuning Request"]
pub type RTREQ_R = crate::BitReader<RTREQSELECT_A>;
#[doc = "Re-Tuning Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTREQSELECT_A {
    #[doc = "0: Fixed or well-tuned sampling clock"]
    OK = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    REQUIRED = 1,
}
impl From<RTREQSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RTREQSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTREQSELECT_A {
        match self.bits {
            false => RTREQSELECT_A::OK,
            true => RTREQSELECT_A::REQUIRED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == RTREQSELECT_A::OK
    }
    #[doc = "Checks if the value of the field is `REQUIRED`"]
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        *self == RTREQSELECT_A::REQUIRED
    }
}
#[doc = "Field `WTACT` reader - Write Transfer Active"]
pub type WTACT_R = crate::BitReader<WTACTSELECT_A>;
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WTACTSELECT_A {
    #[doc = "0: No valid data"]
    NO = 0,
    #[doc = "1: Transferring data"]
    YES = 1,
}
impl From<WTACTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WTACTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTACTSELECT_A {
        match self.bits {
            false => WTACTSELECT_A::NO,
            true => WTACTSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WTACTSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == WTACTSELECT_A::YES
    }
}
#[doc = "Field `RTACT` reader - Read Transfer Active"]
pub type RTACT_R = crate::BitReader<RTACTSELECT_A>;
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTACTSELECT_A {
    #[doc = "0: No valid data"]
    NO = 0,
    #[doc = "1: Transferring data"]
    YES = 1,
}
impl From<RTACTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RTACTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTACTSELECT_A {
        match self.bits {
            false => RTACTSELECT_A::NO,
            true => RTACTSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RTACTSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == RTACTSELECT_A::YES
    }
}
#[doc = "Field `BUFWREN` reader - Buffer Write Enable"]
pub type BUFWREN_R = crate::BitReader<BUFWRENSELECT_A>;
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFWRENSELECT_A {
    #[doc = "0: Write disable"]
    DISABLE = 0,
    #[doc = "1: Write enable"]
    ENABLE = 1,
}
impl From<BUFWRENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BUFWRENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFWRENSELECT_A {
        match self.bits {
            false => BUFWRENSELECT_A::DISABLE,
            true => BUFWRENSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFWRENSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFWRENSELECT_A::ENABLE
    }
}
#[doc = "Field `BUFRDEN` reader - Buffer Read Enable"]
pub type BUFRDEN_R = crate::BitReader<BUFRDENSELECT_A>;
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUFRDENSELECT_A {
    #[doc = "0: Read disable"]
    DISABLE = 0,
    #[doc = "1: Read enable"]
    ENABLE = 1,
}
impl From<BUFRDENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BUFRDENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUFRDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFRDENSELECT_A {
        match self.bits {
            false => BUFRDENSELECT_A::DISABLE,
            true => BUFRDENSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFRDENSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFRDENSELECT_A::ENABLE
    }
}
#[doc = "Field `CARDINS` reader - Card Inserted"]
pub type CARDINS_R = crate::BitReader<CARDINSSELECT_A>;
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARDINSSELECT_A {
    #[doc = "0: Reset or Debouncing or No Card"]
    NO = 0,
    #[doc = "1: Card inserted"]
    YES = 1,
}
impl From<CARDINSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARDINSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARDINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDINSSELECT_A {
        match self.bits {
            false => CARDINSSELECT_A::NO,
            true => CARDINSSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDINSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDINSSELECT_A::YES
    }
}
#[doc = "Field `CARDSS` reader - Card State Stable"]
pub type CARDSS_R = crate::BitReader<CARDSSSELECT_A>;
#[doc = "Card State Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARDSSSELECT_A {
    #[doc = "0: Reset or Debouncing"]
    NO = 0,
    #[doc = "1: No Card or Insered"]
    YES = 1,
}
impl From<CARDSSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARDSSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARDSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDSSSELECT_A {
        match self.bits {
            false => CARDSSSELECT_A::NO,
            true => CARDSSSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDSSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDSSSELECT_A::YES
    }
}
#[doc = "Field `CARDDPL` reader - Card Detect Pin Level"]
pub type CARDDPL_R = crate::BitReader<CARDDPLSELECT_A>;
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARDDPLSELECT_A {
    #[doc = "0: No card present (SDCD#=1)"]
    NO = 0,
    #[doc = "1: Card present (SDCD#=0)"]
    YES = 1,
}
impl From<CARDDPLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDPLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CARDDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDDPLSELECT_A {
        match self.bits {
            false => CARDDPLSELECT_A::NO,
            true => CARDDPLSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDDPLSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDDPLSELECT_A::YES
    }
}
#[doc = "Field `WRPPL` reader - Write Protect Pin Level"]
pub type WRPPL_R = crate::BitReader<WRPPLSELECT_A>;
#[doc = "Write Protect Pin Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPPLSELECT_A {
    #[doc = "0: Write protected (SDWP#=0)"]
    PROTECTED = 0,
    #[doc = "1: Write enabled (SDWP#=1)"]
    ENABLED = 1,
}
impl From<WRPPLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WRPPLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPPLSELECT_A {
        match self.bits {
            false => WRPPLSELECT_A::PROTECTED,
            true => WRPPLSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == WRPPLSELECT_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRPPLSELECT_A::ENABLED
    }
}
#[doc = "Field `DATLL` reader - DAT\\[3:0\\]
Line Level"]
pub type DATLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDLL` reader - CMD Line Level"]
pub type CMDLL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhc(&self) -> CMDINHC_R {
        CMDINHC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhd(&self) -> CMDINHD_R {
        CMDINHD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn dlact(&self) -> DLACT_R {
        DLACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn rtreq(&self) -> RTREQ_R {
        RTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wtact(&self) -> WTACT_R {
        WTACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rtact(&self) -> RTACT_R {
        RTACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufwren(&self) -> BUFWREN_R {
        BUFWREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BUFRDEN_R {
        BUFRDEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline(always)]
    pub fn cardss(&self) -> CARDSS_R {
        CARDSS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddpl(&self) -> CARDDPL_R {
        CARDDPL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Pin Level"]
    #[inline(always)]
    pub fn wrppl(&self) -> WRPPL_R {
        WRPPL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
Line Level"]
    #[inline(always)]
    pub fn datll(&self) -> DATLL_R {
        DATLL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Level"]
    #[inline(always)]
    pub fn cmdll(&self) -> CMDLL_R {
        CMDLL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Present State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0x00f8_0000"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f8_0000;
}
