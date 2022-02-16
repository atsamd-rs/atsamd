#[doc = "Register `USBHS_DEVEPTISR_ISO_MODE[%s]` reader"]
pub struct R(crate::R<USBHS_DEVEPTISR_ISO_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVEPTISR_ISO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVEPTISR_ISO_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVEPTISR_ISO_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub struct TXINI_R(crate::FieldReader<bool, bool>);
impl TXINI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub struct RXOUTI_R(crate::FieldReader<bool, bool>);
impl RXOUTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOUTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOUTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFI` reader - Underflow Interrupt"]
pub struct UNDERFI_R(crate::FieldReader<bool, bool>);
impl UNDERFI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBISOINERRI` reader - High Bandwidth Isochronous IN Underflow Error Interrupt"]
pub struct HBISOINERRI_R(crate::FieldReader<bool, bool>);
impl HBISOINERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HBISOINERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBISOINERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBISOFLUSHI` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub struct HBISOFLUSHI_R(crate::FieldReader<bool, bool>);
impl HBISOFLUSHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HBISOFLUSHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBISOFLUSHI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub struct OVERFI_R(crate::FieldReader<bool, bool>);
impl OVERFI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERRI` reader - CRC Error Interrupt"]
pub struct CRCERRI_R(crate::FieldReader<bool, bool>);
impl CRCERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHORTPACKET` reader - Short Packet Interrupt"]
pub struct SHORTPACKET_R(crate::FieldReader<bool, bool>);
impl SHORTPACKET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHORTPACKET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHORTPACKET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data Toggle Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTSEQ_A {
    #[doc = "0: Data0 toggle sequence"]
    DATA0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    DATA1 = 1,
    #[doc = "2: Reserved for high-bandwidth isochronous endpoint"]
    DATA2 = 2,
    #[doc = "3: Reserved for high-bandwidth isochronous endpoint"]
    MDATA = 3,
}
impl From<DTSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub struct DTSEQ_R(crate::FieldReader<u8, DTSEQ_A>);
impl DTSEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTSEQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTSEQ_A {
        match self.bits {
            0 => DTSEQ_A::DATA0,
            1 => DTSEQ_A::DATA1,
            2 => DTSEQ_A::DATA2,
            3 => DTSEQ_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        **self == DTSEQ_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        **self == DTSEQ_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        **self == DTSEQ_A::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        **self == DTSEQ_A::MDATA
    }
}
impl core::ops::Deref for DTSEQ_R {
    type Target = crate::FieldReader<u8, DTSEQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORTRANS` reader - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
pub struct ERRORTRANS_R(crate::FieldReader<bool, bool>);
impl ERRORTRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORTRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORTRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of Busy Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBUSYBK_A {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0_BUSY = 0,
    #[doc = "1: 1 busy bank"]
    _1_BUSY = 1,
    #[doc = "2: 2 busy banks"]
    _2_BUSY = 2,
    #[doc = "3: 3 busy banks"]
    _3_BUSY = 3,
}
impl From<NBUSYBK_A> for u8 {
    #[inline(always)]
    fn from(variant: NBUSYBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub struct NBUSYBK_R(crate::FieldReader<u8, NBUSYBK_A>);
impl NBUSYBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBUSYBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBUSYBK_A {
        match self.bits {
            0 => NBUSYBK_A::_0_BUSY,
            1 => NBUSYBK_A::_1_BUSY,
            2 => NBUSYBK_A::_2_BUSY,
            3 => NBUSYBK_A::_3_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        **self == NBUSYBK_A::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        **self == NBUSYBK_A::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        **self == NBUSYBK_A::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        **self == NBUSYBK_A::_3_BUSY
    }
}
impl core::ops::Deref for NBUSYBK_R {
    type Target = crate::FieldReader<u8, NBUSYBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CURRBK_A {
    #[doc = "0: Current bank is bank0"]
    BANK0 = 0,
    #[doc = "1: Current bank is bank1"]
    BANK1 = 1,
    #[doc = "2: Current bank is bank2"]
    BANK2 = 2,
}
impl From<CURRBK_A> for u8 {
    #[inline(always)]
    fn from(variant: CURRBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub struct CURRBK_R(crate::FieldReader<u8, CURRBK_A>);
impl CURRBK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CURRBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURRBK_A> {
        match self.bits {
            0 => Some(CURRBK_A::BANK0),
            1 => Some(CURRBK_A::BANK1),
            2 => Some(CURRBK_A::BANK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        **self == CURRBK_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        **self == CURRBK_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        **self == CURRBK_A::BANK2
    }
}
impl core::ops::Deref for CURRBK_R {
    type Target = crate::FieldReader<u8, CURRBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWALL` reader - Read/Write Allowed"]
pub struct RWALL_R(crate::FieldReader<bool, bool>);
impl RWALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub struct CFGOK_R(crate::FieldReader<bool, bool>);
impl CFGOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFGOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYCT` reader - Byte Count"]
pub struct BYCT_R(crate::FieldReader<u16, u16>);
impl BYCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BYCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYCT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfi(&self) -> UNDERFI_R {
        UNDERFI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerri(&self) -> HBISOINERRI_R {
        HBISOINERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushi(&self) -> HBISOFLUSHI_R {
        HBISOFLUSHI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerri(&self) -> CRCERRI_R {
        CRCERRI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> SHORTPACKET_R {
        SHORTPACKET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortrans(&self) -> ERRORTRANS_R {
        ERRORTRANS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr_iso_mode](index.html) module"]
pub struct USBHS_DEVEPTISR_ISO_MODE_SPEC;
impl crate::RegisterSpec for USBHS_DEVEPTISR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_deveptisr_iso_mode::R](R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR_ISO_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_DEVEPTISR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for USBHS_DEVEPTISR_ISO_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
