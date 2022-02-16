#[doc = "Register `GMAC_NCFGR` reader"]
pub struct R(crate::R<GMAC_NCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_NCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_NCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_NCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_NCFGR` writer"]
pub struct W(crate::W<GMAC_NCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_NCFGR_SPEC>;
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
impl From<crate::W<GMAC_NCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_NCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPD` reader - Speed"]
pub struct SPD_R(crate::FieldReader<bool, bool>);
impl SPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPD` writer - Speed"]
pub struct SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD_W<'a> {
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
#[doc = "Field `FD` reader - Full Duplex"]
pub struct FD_R(crate::FieldReader<bool, bool>);
impl FD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FD` writer - Full Duplex"]
pub struct FD_W<'a> {
    w: &'a mut W,
}
impl<'a> FD_W<'a> {
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
#[doc = "Field `DNVLAN` reader - Discard Non-VLAN FRAMES"]
pub struct DNVLAN_R(crate::FieldReader<bool, bool>);
impl DNVLAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNVLAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNVLAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNVLAN` writer - Discard Non-VLAN FRAMES"]
pub struct DNVLAN_W<'a> {
    w: &'a mut W,
}
impl<'a> DNVLAN_W<'a> {
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
#[doc = "Field `JFRAME` reader - Jumbo Frame Size"]
pub struct JFRAME_R(crate::FieldReader<bool, bool>);
impl JFRAME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JFRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JFRAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JFRAME` writer - Jumbo Frame Size"]
pub struct JFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> JFRAME_W<'a> {
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
#[doc = "Field `CAF` reader - Copy All Frames"]
pub struct CAF_R(crate::FieldReader<bool, bool>);
impl CAF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAF` writer - Copy All Frames"]
pub struct CAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAF_W<'a> {
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
#[doc = "Field `NBC` reader - No Broadcast"]
pub struct NBC_R(crate::FieldReader<bool, bool>);
impl NBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBC` writer - No Broadcast"]
pub struct NBC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBC_W<'a> {
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
#[doc = "Field `MTIHEN` reader - Multicast Hash Enable"]
pub struct MTIHEN_R(crate::FieldReader<bool, bool>);
impl MTIHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTIHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTIHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTIHEN` writer - Multicast Hash Enable"]
pub struct MTIHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MTIHEN_W<'a> {
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
#[doc = "Field `UNIHEN` reader - Unicast Hash Enable"]
pub struct UNIHEN_R(crate::FieldReader<bool, bool>);
impl UNIHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNIHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNIHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNIHEN` writer - Unicast Hash Enable"]
pub struct UNIHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNIHEN_W<'a> {
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
#[doc = "Field `MAXFS` reader - 1536 Maximum Frame Size"]
pub struct MAXFS_R(crate::FieldReader<bool, bool>);
impl MAXFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAXFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXFS` writer - 1536 Maximum Frame Size"]
pub struct MAXFS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFS_W<'a> {
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
#[doc = "Field `RTY` reader - Retry Test"]
pub struct RTY_R(crate::FieldReader<bool, bool>);
impl RTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTY` writer - Retry Test"]
pub struct RTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTY_W<'a> {
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
#[doc = "Field `PEN` reader - Pause Enable"]
pub struct PEN_R(crate::FieldReader<bool, bool>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Pause Enable"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
#[doc = "Field `RXBUFO` reader - Receive Buffer Offset"]
pub struct RXBUFO_R(crate::FieldReader<u8, u8>);
impl RXBUFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXBUFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFO` writer - Receive Buffer Offset"]
pub struct RXBUFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `LFERD` reader - Length Field Error Frame Discard"]
pub struct LFERD_R(crate::FieldReader<bool, bool>);
impl LFERD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LFERD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFERD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFERD` writer - Length Field Error Frame Discard"]
pub struct LFERD_W<'a> {
    w: &'a mut W,
}
impl<'a> LFERD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RFCS` reader - Remove FCS"]
pub struct RFCS_R(crate::FieldReader<bool, bool>);
impl RFCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCS` writer - Remove FCS"]
pub struct RFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "MDC CLock Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: MCK divided by 8 (MCK up to 20 MHz)"]
    MCK_8 = 0,
    #[doc = "1: MCK divided by 16 (MCK up to 40 MHz)"]
    MCK_16 = 1,
    #[doc = "2: MCK divided by 32 (MCK up to 80 MHz)"]
    MCK_32 = 2,
    #[doc = "3: MCK divided by 48 (MCK up to 120 MHz)"]
    MCK_48 = 3,
    #[doc = "4: MCK divided by 64 (MCK up to 160 MHz)"]
    MCK_64 = 4,
    #[doc = "5: MCK divided by 96 (MCK up to 240 MHz)"]
    MCK_96 = 5,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK` reader - MDC CLock Division"]
pub struct CLK_R(crate::FieldReader<u8, CLK_A>);
impl CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_A> {
        match self.bits {
            0 => Some(CLK_A::MCK_8),
            1 => Some(CLK_A::MCK_16),
            2 => Some(CLK_A::MCK_32),
            3 => Some(CLK_A::MCK_48),
            4 => Some(CLK_A::MCK_64),
            5 => Some(CLK_A::MCK_96),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK_8`"]
    #[inline(always)]
    pub fn is_mck_8(&self) -> bool {
        **self == CLK_A::MCK_8
    }
    #[doc = "Checks if the value of the field is `MCK_16`"]
    #[inline(always)]
    pub fn is_mck_16(&self) -> bool {
        **self == CLK_A::MCK_16
    }
    #[doc = "Checks if the value of the field is `MCK_32`"]
    #[inline(always)]
    pub fn is_mck_32(&self) -> bool {
        **self == CLK_A::MCK_32
    }
    #[doc = "Checks if the value of the field is `MCK_48`"]
    #[inline(always)]
    pub fn is_mck_48(&self) -> bool {
        **self == CLK_A::MCK_48
    }
    #[doc = "Checks if the value of the field is `MCK_64`"]
    #[inline(always)]
    pub fn is_mck_64(&self) -> bool {
        **self == CLK_A::MCK_64
    }
    #[doc = "Checks if the value of the field is `MCK_96`"]
    #[inline(always)]
    pub fn is_mck_96(&self) -> bool {
        **self == CLK_A::MCK_96
    }
}
impl core::ops::Deref for CLK_R {
    type Target = crate::FieldReader<u8, CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK` writer - MDC CLock Division"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)"]
    #[inline(always)]
    pub fn mck_8(self) -> &'a mut W {
        self.variant(CLK_A::MCK_8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)"]
    #[inline(always)]
    pub fn mck_16(self) -> &'a mut W {
        self.variant(CLK_A::MCK_16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)"]
    #[inline(always)]
    pub fn mck_32(self) -> &'a mut W {
        self.variant(CLK_A::MCK_32)
    }
    #[doc = "MCK divided by 48 (MCK up to 120 MHz)"]
    #[inline(always)]
    pub fn mck_48(self) -> &'a mut W {
        self.variant(CLK_A::MCK_48)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)"]
    #[inline(always)]
    pub fn mck_64(self) -> &'a mut W {
        self.variant(CLK_A::MCK_64)
    }
    #[doc = "MCK divided by 96 (MCK up to 240 MHz)"]
    #[inline(always)]
    pub fn mck_96(self) -> &'a mut W {
        self.variant(CLK_A::MCK_96)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub struct DBW_R(crate::FieldReader<u8, u8>);
impl DBW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub struct DBW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `DCPF` reader - Disable Copy of Pause Frames"]
pub struct DCPF_R(crate::FieldReader<bool, bool>);
impl DCPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCPF` writer - Disable Copy of Pause Frames"]
pub struct DCPF_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RXCOEN` reader - Receive Checksum Offload Enable"]
pub struct RXCOEN_R(crate::FieldReader<bool, bool>);
impl RXCOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXCOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCOEN` writer - Receive Checksum Offload Enable"]
pub struct RXCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EFRHD` reader - Enable Frames Received in Half Duplex"]
pub struct EFRHD_R(crate::FieldReader<bool, bool>);
impl EFRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EFRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFRHD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFRHD` writer - Enable Frames Received in Half Duplex"]
pub struct EFRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFRHD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub struct IRXFCS_R(crate::FieldReader<bool, bool>);
impl IRXFCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRXFCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRXFCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub struct IRXFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRXFCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `IPGSEN` reader - IP Stretch Enable"]
pub struct IPGSEN_R(crate::FieldReader<bool, bool>);
impl IPGSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPGSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPGSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPGSEN` writer - IP Stretch Enable"]
pub struct IPGSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `RXBP` reader - Receive Bad Preamble"]
pub struct RXBP_R(crate::FieldReader<bool, bool>);
impl RXBP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBP` writer - Receive Bad Preamble"]
pub struct RXBP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `IRXER` reader - Ignore IPG GRXER"]
pub struct IRXER_R(crate::FieldReader<bool, bool>);
impl IRXER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRXER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRXER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRXER` writer - Ignore IPG GRXER"]
pub struct IRXER_W<'a> {
    w: &'a mut W,
}
impl<'a> IRXER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&self) -> DNVLAN_R {
        DNVLAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&self) -> MTIHEN_R {
        MTIHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&self) -> UNIHEN_R {
        UNIHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&self) -> MAXFS_R {
        MAXFS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&self) -> RXBUFO_R {
        RXBUFO_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&self) -> LFERD_R {
        LFERD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&self) -> RFCS_R {
        RFCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&self) -> DCPF_R {
        DCPF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&self) -> RXCOEN_R {
        RXCOEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&self) -> IPGSEN_R {
        IPGSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&self) -> RXBP_R {
        RXBP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&self) -> IRXER_R {
        IRXER_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W {
        SPD_W { w: self }
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W {
        FD_W { w: self }
    }
    #[doc = "Bit 2 - Discard Non-VLAN FRAMES"]
    #[inline(always)]
    pub fn dnvlan(&mut self) -> DNVLAN_W {
        DNVLAN_W { w: self }
    }
    #[doc = "Bit 3 - Jumbo Frame Size"]
    #[inline(always)]
    pub fn jframe(&mut self) -> JFRAME_W {
        JFRAME_W { w: self }
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W {
        CAF_W { w: self }
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W {
        NBC_W { w: self }
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mtihen(&mut self) -> MTIHEN_W {
        MTIHEN_W { w: self }
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn unihen(&mut self) -> UNIHEN_W {
        UNIHEN_W { w: self }
    }
    #[doc = "Bit 8 - 1536 Maximum Frame Size"]
    #[inline(always)]
    pub fn maxfs(&mut self) -> MAXFS_W {
        MAXFS_W { w: self }
    }
    #[doc = "Bit 12 - Retry Test"]
    #[inline(always)]
    pub fn rty(&mut self) -> RTY_W {
        RTY_W { w: self }
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rxbufo(&mut self) -> RXBUFO_W {
        RXBUFO_W { w: self }
    }
    #[doc = "Bit 16 - Length Field Error Frame Discard"]
    #[inline(always)]
    pub fn lferd(&mut self) -> LFERD_W {
        LFERD_W { w: self }
    }
    #[doc = "Bit 17 - Remove FCS"]
    #[inline(always)]
    pub fn rfcs(&mut self) -> RFCS_W {
        RFCS_W { w: self }
    }
    #[doc = "Bits 18:20 - MDC CLock Division"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bits 21:22 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W {
        DBW_W { w: self }
    }
    #[doc = "Bit 23 - Disable Copy of Pause Frames"]
    #[inline(always)]
    pub fn dcpf(&mut self) -> DCPF_W {
        DCPF_W { w: self }
    }
    #[doc = "Bit 24 - Receive Checksum Offload Enable"]
    #[inline(always)]
    pub fn rxcoen(&mut self) -> RXCOEN_W {
        RXCOEN_W { w: self }
    }
    #[doc = "Bit 25 - Enable Frames Received in Half Duplex"]
    #[inline(always)]
    pub fn efrhd(&mut self) -> EFRHD_W {
        EFRHD_W { w: self }
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&mut self) -> IRXFCS_W {
        IRXFCS_W { w: self }
    }
    #[doc = "Bit 28 - IP Stretch Enable"]
    #[inline(always)]
    pub fn ipgsen(&mut self) -> IPGSEN_W {
        IPGSEN_W { w: self }
    }
    #[doc = "Bit 29 - Receive Bad Preamble"]
    #[inline(always)]
    pub fn rxbp(&mut self) -> RXBP_W {
        RXBP_W { w: self }
    }
    #[doc = "Bit 30 - Ignore IPG GRXER"]
    #[inline(always)]
    pub fn irxer(&mut self) -> IRXER_W {
        IRXER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_ncfgr](index.html) module"]
pub struct GMAC_NCFGR_SPEC;
impl crate::RegisterSpec for GMAC_NCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_ncfgr::R](R) reader structure"]
impl crate::Readable for GMAC_NCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_ncfgr::W](W) writer structure"]
impl crate::Writable for GMAC_NCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_NCFGR to value 0"]
impl crate::Resettable for GMAC_NCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
