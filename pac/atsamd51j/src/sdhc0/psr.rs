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
#[doc = "Field `CMDINHC` reader - Command Inhibit (CMD)"]
pub struct CMDINHC_R(crate::FieldReader<bool, CMDINHC_A>);
impl CMDINHC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDINHC_R(crate::FieldReader::new(bits))
    }
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
        **self == CMDINHC_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        **self == CMDINHC_A::CANNOT
    }
}
impl core::ops::Deref for CMDINHC_R {
    type Target = crate::FieldReader<bool, CMDINHC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CMDINHD` reader - Command Inhibit (DAT)"]
pub struct CMDINHD_R(crate::FieldReader<bool, CMDINHD_A>);
impl CMDINHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDINHD_R(crate::FieldReader::new(bits))
    }
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
        **self == CMDINHD_A::CAN
    }
    #[doc = "Checks if the value of the field is `CANNOT`"]
    #[inline(always)]
    pub fn is_cannot(&self) -> bool {
        **self == CMDINHD_A::CANNOT
    }
}
impl core::ops::Deref for CMDINHD_R {
    type Target = crate::FieldReader<bool, CMDINHD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DLACT` reader - DAT Line Active"]
pub struct DLACT_R(crate::FieldReader<bool, DLACT_A>);
impl DLACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLACT_R(crate::FieldReader::new(bits))
    }
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
        **self == DLACT_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DLACT_A::ACTIVE
    }
}
impl core::ops::Deref for DLACT_R {
    type Target = crate::FieldReader<bool, DLACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RTREQ` reader - Re-Tuning Request"]
pub struct RTREQ_R(crate::FieldReader<bool, RTREQ_A>);
impl RTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTREQ_R(crate::FieldReader::new(bits))
    }
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
        **self == RTREQ_A::OK
    }
    #[doc = "Checks if the value of the field is `REQUIRED`"]
    #[inline(always)]
    pub fn is_required(&self) -> bool {
        **self == RTREQ_A::REQUIRED
    }
}
impl core::ops::Deref for RTREQ_R {
    type Target = crate::FieldReader<bool, RTREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `WTACT` reader - Write Transfer Active"]
pub struct WTACT_R(crate::FieldReader<bool, WTACT_A>);
impl WTACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WTACT_R(crate::FieldReader::new(bits))
    }
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
        **self == WTACT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == WTACT_A::YES
    }
}
impl core::ops::Deref for WTACT_R {
    type Target = crate::FieldReader<bool, WTACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RTACT` reader - Read Transfer Active"]
pub struct RTACT_R(crate::FieldReader<bool, RTACT_A>);
impl RTACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTACT_R(crate::FieldReader::new(bits))
    }
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
        **self == RTACT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == RTACT_A::YES
    }
}
impl core::ops::Deref for RTACT_R {
    type Target = crate::FieldReader<bool, RTACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BUFWREN` reader - Buffer Write Enable"]
pub struct BUFWREN_R(crate::FieldReader<bool, BUFWREN_A>);
impl BUFWREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFWREN_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFWREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BUFWREN_A::ENABLE
    }
}
impl core::ops::Deref for BUFWREN_R {
    type Target = crate::FieldReader<bool, BUFWREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `BUFRDEN` reader - Buffer Read Enable"]
pub struct BUFRDEN_R(crate::FieldReader<bool, BUFRDEN_A>);
impl BUFRDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFRDEN_R(crate::FieldReader::new(bits))
    }
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
        **self == BUFRDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BUFRDEN_A::ENABLE
    }
}
impl core::ops::Deref for BUFRDEN_R {
    type Target = crate::FieldReader<bool, BUFRDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CARDINS` reader - Card Inserted"]
pub struct CARDINS_R(crate::FieldReader<bool, CARDINS_A>);
impl CARDINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDINS_R(crate::FieldReader::new(bits))
    }
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
        **self == CARDINS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CARDINS_A::YES
    }
}
impl core::ops::Deref for CARDINS_R {
    type Target = crate::FieldReader<bool, CARDINS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CARDSS` reader - Card State Stable"]
pub struct CARDSS_R(crate::FieldReader<bool, CARDSS_A>);
impl CARDSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDSS_R(crate::FieldReader::new(bits))
    }
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
        **self == CARDSS_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CARDSS_A::YES
    }
}
impl core::ops::Deref for CARDSS_R {
    type Target = crate::FieldReader<bool, CARDSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CARDDPL` reader - Card Detect Pin Level"]
pub struct CARDDPL_R(crate::FieldReader<bool, CARDDPL_A>);
impl CARDDPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDDPL_R(crate::FieldReader::new(bits))
    }
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
        **self == CARDDPL_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == CARDDPL_A::YES
    }
}
impl core::ops::Deref for CARDDPL_R {
    type Target = crate::FieldReader<bool, CARDDPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `WRPPL` reader - Write Protect Pin Level"]
pub struct WRPPL_R(crate::FieldReader<bool, WRPPL_A>);
impl WRPPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRPPL_R(crate::FieldReader::new(bits))
    }
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
        **self == WRPPL_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WRPPL_A::ENABLED
    }
}
impl core::ops::Deref for WRPPL_R {
    type Target = crate::FieldReader<bool, WRPPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLL` reader - DAT\\[3:0\\]
Line Level"]
pub struct DATLL_R(crate::FieldReader<u8, u8>);
impl DATLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDLL` reader - CMD Line Level"]
pub struct CMDLL_R(crate::FieldReader<bool, bool>);
impl CMDLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
