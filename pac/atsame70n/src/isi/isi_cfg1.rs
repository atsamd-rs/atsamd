#[doc = "Register `ISI_CFG1` reader"]
pub struct R(crate::R<ISI_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_CFG1` writer"]
pub struct W(crate::W<ISI_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_CFG1_SPEC>;
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
impl From<crate::W<ISI_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSYNC_POL` reader - Horizontal Synchronization Polarity"]
pub struct HSYNC_POL_R(crate::FieldReader<bool, bool>);
impl HSYNC_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSYNC_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSYNC_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSYNC_POL` writer - Horizontal Synchronization Polarity"]
pub struct HSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_POL_W<'a> {
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
#[doc = "Field `VSYNC_POL` reader - Vertical Synchronization Polarity"]
pub struct VSYNC_POL_R(crate::FieldReader<bool, bool>);
impl VSYNC_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_POL` writer - Vertical Synchronization Polarity"]
pub struct VSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_POL_W<'a> {
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
#[doc = "Field `PIXCLK_POL` reader - Pixel Clock Polarity"]
pub struct PIXCLK_POL_R(crate::FieldReader<bool, bool>);
impl PIXCLK_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIXCLK_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIXCLK_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIXCLK_POL` writer - Pixel Clock Polarity"]
pub struct PIXCLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXCLK_POL_W<'a> {
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
#[doc = "Field `GRAYLE` reader - Grayscale Little Endian"]
pub struct GRAYLE_R(crate::FieldReader<bool, bool>);
impl GRAYLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GRAYLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRAYLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GRAYLE` writer - Grayscale Little Endian"]
pub struct GRAYLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAYLE_W<'a> {
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
#[doc = "Field `EMB_SYNC` reader - Embedded Synchronization"]
pub struct EMB_SYNC_R(crate::FieldReader<bool, bool>);
impl EMB_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMB_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMB_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMB_SYNC` writer - Embedded Synchronization"]
pub struct EMB_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMB_SYNC_W<'a> {
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
#[doc = "Field `CRC_SYNC` reader - Embedded Synchronization Correction"]
pub struct CRC_SYNC_R(crate::FieldReader<bool, bool>);
impl CRC_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_SYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_SYNC` writer - Embedded Synchronization Correction"]
pub struct CRC_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_SYNC_W<'a> {
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
#[doc = "Field `FRATE` reader - Frame Rate \\[0..7\\]"]
pub struct FRATE_R(crate::FieldReader<u8, u8>);
impl FRATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRATE` writer - Frame Rate \\[0..7\\]"]
pub struct FRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DISCR` reader - Disable Codec Request"]
pub struct DISCR_R(crate::FieldReader<bool, bool>);
impl DISCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCR` writer - Disable Codec Request"]
pub struct DISCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCR_W<'a> {
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
#[doc = "Field `FULL` reader - Full Mode is Allowed"]
pub struct FULL_R(crate::FieldReader<bool, bool>);
impl FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL` writer - Full Mode is Allowed"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
#[doc = "Threshold Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THMASK_A {
    #[doc = "0: Only 4 beats AHB burst allowed"]
    BEATS_4 = 0,
    #[doc = "1: Only 4 and 8 beats AHB burst allowed"]
    BEATS_8 = 1,
    #[doc = "2: 4, 8 and 16 beats AHB burst allowed"]
    BEATS_16 = 2,
}
impl From<THMASK_A> for u8 {
    #[inline(always)]
    fn from(variant: THMASK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `THMASK` reader - Threshold Mask"]
pub struct THMASK_R(crate::FieldReader<u8, THMASK_A>);
impl THMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THMASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<THMASK_A> {
        match self.bits {
            0 => Some(THMASK_A::BEATS_4),
            1 => Some(THMASK_A::BEATS_8),
            2 => Some(THMASK_A::BEATS_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BEATS_4`"]
    #[inline(always)]
    pub fn is_beats_4(&self) -> bool {
        **self == THMASK_A::BEATS_4
    }
    #[doc = "Checks if the value of the field is `BEATS_8`"]
    #[inline(always)]
    pub fn is_beats_8(&self) -> bool {
        **self == THMASK_A::BEATS_8
    }
    #[doc = "Checks if the value of the field is `BEATS_16`"]
    #[inline(always)]
    pub fn is_beats_16(&self) -> bool {
        **self == THMASK_A::BEATS_16
    }
}
impl core::ops::Deref for THMASK_R {
    type Target = crate::FieldReader<u8, THMASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THMASK` writer - Threshold Mask"]
pub struct THMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> THMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THMASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_4(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_8(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_16(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `SLD` reader - Start of Line Delay"]
pub struct SLD_R(crate::FieldReader<u8, u8>);
impl SLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLD` writer - Start of Line Delay"]
pub struct SLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SFD` reader - Start of Frame Delay"]
pub struct SFD_R(crate::FieldReader<u8, u8>);
impl SFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFD` writer - Start of Frame Delay"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VSYNC_POL_R {
        VSYNC_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&self) -> PIXCLK_POL_R {
        PIXCLK_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&self) -> GRAYLE_R {
        GRAYLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&self) -> EMB_SYNC_R {
        EMB_SYNC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&self) -> CRC_SYNC_R {
        CRC_SYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&self) -> DISCR_R {
        DISCR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&self) -> THMASK_R {
        THMASK_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&self) -> SLD_R {
        SLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> HSYNC_POL_W {
        HSYNC_POL_W { w: self }
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&mut self) -> VSYNC_POL_W {
        VSYNC_POL_W { w: self }
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&mut self) -> PIXCLK_POL_W {
        PIXCLK_POL_W { w: self }
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&mut self) -> GRAYLE_W {
        GRAYLE_W { w: self }
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&mut self) -> EMB_SYNC_W {
        EMB_SYNC_W { w: self }
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&mut self) -> CRC_SYNC_W {
        CRC_SYNC_W { w: self }
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&mut self) -> FRATE_W {
        FRATE_W { w: self }
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&mut self) -> DISCR_W {
        DISCR_W { w: self }
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&mut self) -> THMASK_W {
        THMASK_W { w: self }
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&mut self) -> SLD_W {
        SLD_W { w: self }
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cfg1](index.html) module"]
pub struct ISI_CFG1_SPEC;
impl crate::RegisterSpec for ISI_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_cfg1::R](R) reader structure"]
impl crate::Readable for ISI_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_cfg1::W](W) writer structure"]
impl crate::Writable for ISI_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_CFG1 to value 0"]
impl crate::Resettable for ISI_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
