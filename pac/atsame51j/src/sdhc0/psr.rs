#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDINHC_A {
    #[doc = "0: Can issue command using only CMD line"]
    CAN = 0,
    #[doc = "1: Cannot issue command"]
    CANNOT = 1,
}
impl From<CMDINHC_A> for bool {
    #[inline(always)]
    fn from(variant: CMDINHC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDINHC`"]
pub type CMDINHC_R = crate::R<bool, CMDINHC_A>;
impl CMDINHC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDINHC_A {
        match self.bits {
            false => CMDINHC_A::CAN,
            true => CMDINHC_A::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == CMDINHC_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHC_A::CANNOT
    }
}
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDINHD_A {
    #[doc = "0: Can issue command which uses the DAT line"]
    CAN = 0,
    #[doc = "1: Cannot issue command which uses the DAT line"]
    CANNOT = 1,
}
impl From<CMDINHD_A> for bool {
    #[inline(always)]
    fn from(variant: CMDINHD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMDINHD`"]
pub type CMDINHD_R = crate::R<bool, CMDINHD_A>;
impl CMDINHD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDINHD_A {
        match self.bits {
            false => CMDINHD_A::CAN,
            true => CMDINHD_A::CANNOT,
        }
    }
    #[doc = "Checks if the value of the field is `CAN`"]
    #[inline(always)]
    pub fn is_can(&self) -> bool {
        *self == CMDINHD_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        *self == CMDINHD_A::CANNOT
    }
}
#[doc = "DAT Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLACT_A {
    #[doc = "0: DAT Line Inactive"]
    INACTIVE = 0,
    #[doc = "1: DAT Line Active"]
    ACTIVE = 1,
}
impl From<DLACT_A> for bool {
    #[inline(always)]
    fn from(variant: DLACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLACT`"]
pub type DLACT_R = crate::R<bool, DLACT_A>;
impl DLACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLACT_A {
        match self.bits {
            false => DLACT_A::INACTIVE,
            true => DLACT_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DLACT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLACT_A::ACTIVE
    }
}
#[doc = "Re-Tuning Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTREQ_A {
    #[doc = "0: Fixed or well-tuned sampling clock"]
    OK = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    REQUIRED = 1,
}
impl From<RTREQ_A> for bool {
    #[inline(always)]
    fn from(variant: RTREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTREQ`"]
pub type RTREQ_R = crate::R<bool, RTREQ_A>;
impl RTREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTREQ_A {
        match self.bits {
            false => RTREQ_A::OK,
            true => RTREQ_A::REQUIRED,
        }
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == RTREQ_A::OK
    }
    #[doc = "Checks if the value of the field is `REQUIRED`"]
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        *self == RTREQ_A::REQUIRED
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTACT_A {
    #[doc = "0: No valid data"]
    NO = 0,
    #[doc = "1: Transferring data"]
    YES = 1,
}
impl From<WTACT_A> for bool {
    #[inline(always)]
    fn from(variant: WTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTACT`"]
pub type WTACT_R = crate::R<bool, WTACT_A>;
impl WTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTACT_A {
        match self.bits {
            false => WTACT_A::NO,
            true => WTACT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WTACT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == WTACT_A::YES
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTACT_A {
    #[doc = "0: No valid data"]
    NO = 0,
    #[doc = "1: Transferring data"]
    YES = 1,
}
impl From<RTACT_A> for bool {
    #[inline(always)]
    fn from(variant: RTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTACT`"]
pub type RTACT_R = crate::R<bool, RTACT_A>;
impl RTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTACT_A {
        match self.bits {
            false => RTACT_A::NO,
            true => RTACT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RTACT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == RTACT_A::YES
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFWREN_A {
    #[doc = "0: Write disable"]
    DISABLE = 0,
    #[doc = "1: Write enable"]
    ENABLE = 1,
}
impl From<BUFWREN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFWREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFWREN`"]
pub type BUFWREN_R = crate::R<bool, BUFWREN_A>;
impl BUFWREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFWREN_A {
        match self.bits {
            false => BUFWREN_A::DISABLE,
            true => BUFWREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFWREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFWREN_A::ENABLE
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFRDEN_A {
    #[doc = "0: Read disable"]
    DISABLE = 0,
    #[doc = "1: Read enable"]
    ENABLE = 1,
}
impl From<BUFRDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFRDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFRDEN`"]
pub type BUFRDEN_R = crate::R<bool, BUFRDEN_A>;
impl BUFRDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFRDEN_A {
        match self.bits {
            false => BUFRDEN_A::DISABLE,
            true => BUFRDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUFRDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUFRDEN_A::ENABLE
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDINS_A {
    #[doc = "0: Reset or Debouncing or No Card"]
    NO = 0,
    #[doc = "1: Card inserted"]
    YES = 1,
}
impl From<CARDINS_A> for bool {
    #[inline(always)]
    fn from(variant: CARDINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARDINS`"]
pub type CARDINS_R = crate::R<bool, CARDINS_A>;
impl CARDINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDINS_A {
        match self.bits {
            false => CARDINS_A::NO,
            true => CARDINS_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDINS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDINS_A::YES
    }
}
#[doc = "Card State Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDSS_A {
    #[doc = "0: Reset or Debouncing"]
    NO = 0,
    #[doc = "1: No Card or Insered"]
    YES = 1,
}
impl From<CARDSS_A> for bool {
    #[inline(always)]
    fn from(variant: CARDSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARDSS`"]
pub type CARDSS_R = crate::R<bool, CARDSS_A>;
impl CARDSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDSS_A {
        match self.bits {
            false => CARDSS_A::NO,
            true => CARDSS_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDSS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDSS_A::YES
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARDDPL_A {
    #[doc = "0: No card present (SDCD#=1)"]
    NO = 0,
    #[doc = "1: Card present (SDCD#=0)"]
    YES = 1,
}
impl From<CARDDPL_A> for bool {
    #[inline(always)]
    fn from(variant: CARDDPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARDDPL`"]
pub type CARDDPL_R = crate::R<bool, CARDDPL_A>;
impl CARDDPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARDDPL_A {
        match self.bits {
            false => CARDDPL_A::NO,
            true => CARDDPL_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CARDDPL_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CARDDPL_A::YES
    }
}
#[doc = "Write Protect Pin Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPPL_A {
    #[doc = "0: Write protected (SDWP#=0)"]
    PROTECTED = 0,
    #[doc = "1: Write enabled (SDWP#=1)"]
    ENABLED = 1,
}
impl From<WRPPL_A> for bool {
    #[inline(always)]
    fn from(variant: WRPPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRPPL`"]
pub type WRPPL_R = crate::R<bool, WRPPL_A>;
impl WRPPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPPL_A {
        match self.bits {
            false => WRPPL_A::PROTECTED,
            true => WRPPL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == WRPPL_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRPPL_A::ENABLED
    }
}
#[doc = "Reader of field `DATLL`"]
pub type DATLL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMDLL`"]
pub type CMDLL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhc(&self) -> CMDINHC_R {
        CMDINHC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhd(&self) -> CMDINHD_R {
        CMDINHD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn dlact(&self) -> DLACT_R {
        DLACT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn rtreq(&self) -> RTREQ_R {
        RTREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wtact(&self) -> WTACT_R {
        WTACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rtact(&self) -> RTACT_R {
        RTACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufwren(&self) -> BUFWREN_R {
        BUFWREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BUFRDEN_R {
        BUFRDEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline(always)]
    pub fn cardss(&self) -> CARDSS_R {
        CARDSS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddpl(&self) -> CARDDPL_R {
        CARDDPL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write Protect Pin Level"]
    #[inline(always)]
    pub fn wrppl(&self) -> WRPPL_R {
        WRPPL_R::new(((self.bits >> 19) & 0x01) != 0)
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
        CMDLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
