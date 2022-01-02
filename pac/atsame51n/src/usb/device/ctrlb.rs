#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DETACH` reader - Detach"]
pub struct DETACH_R(crate::FieldReader<bool, bool>);
impl DETACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETACH` writer - Detach"]
pub struct DETACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DETACH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `UPRSM` reader - Upstream Resume"]
pub struct UPRSM_R(crate::FieldReader<bool, bool>);
impl UPRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSM` writer - Upstream Resume"]
pub struct UPRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: FS : Full Speed"]
    FS = 0,
    #[doc = "1: LS : Low Speed"]
    LS = 1,
    #[doc = "2: HS : High Speed capable"]
    HS = 2,
    #[doc = "3: HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    HSTM = 3,
}
impl From<SPDCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPDCONF` reader - Speed Configuration"]
pub struct SPDCONF_R(crate::FieldReader<u8, SPDCONF_A>);
impl SPDCONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPDCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDCONF_A {
        match self.bits {
            0 => SPDCONF_A::FS,
            1 => SPDCONF_A::LS,
            2 => SPDCONF_A::HS,
            3 => SPDCONF_A::HSTM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        **self == SPDCONF_A::FS
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        **self == SPDCONF_A::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        **self == SPDCONF_A::HS
    }
    #[doc = "Checks if the value of the field is `HSTM`"]
    #[inline(always)]
    pub fn is_hstm(&self) -> bool {
        **self == SPDCONF_A::HSTM
    }
}
impl core::ops::Deref for SPDCONF_R {
    type Target = crate::FieldReader<u8, SPDCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDCONF` writer - Speed Configuration"]
pub struct SPDCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDCONF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FS : Full Speed"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(SPDCONF_A::FS)
    }
    #[doc = "LS : Low Speed"]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(SPDCONF_A::LS)
    }
    #[doc = "HS : High Speed capable"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut W {
        self.variant(SPDCONF_A::HS)
    }
    #[doc = "HSTM: High Speed Test Mode (force high-speed mode for test mode)"]
    #[inline(always)]
    pub fn hstm(self) -> &'a mut W {
        self.variant(SPDCONF_A::HSTM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `NREPLY` reader - No Reply"]
pub struct NREPLY_R(crate::FieldReader<bool, bool>);
impl NREPLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NREPLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NREPLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NREPLY` writer - No Reply"]
pub struct NREPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TSTJ` reader - Test mode J"]
pub struct TSTJ_R(crate::FieldReader<bool, bool>);
impl TSTJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTJ` writer - Test mode J"]
pub struct TSTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TSTK` reader - Test mode K"]
pub struct TSTK_R(crate::FieldReader<bool, bool>);
impl TSTK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTK` writer - Test mode K"]
pub struct TSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub struct TSTPCKT_R(crate::FieldReader<bool, bool>);
impl TSTPCKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTPCKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTPCKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub struct TSTPCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTPCKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OPMODE2` reader - Specific Operational Mode"]
pub struct OPMODE2_R(crate::FieldReader<bool, bool>);
impl OPMODE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPMODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPMODE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE2` writer - Specific Operational Mode"]
pub struct OPMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `GNAK` reader - Global NAK"]
pub struct GNAK_R(crate::FieldReader<bool, bool>);
impl GNAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GNAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GNAK` writer - Global NAK"]
pub struct GNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> GNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Link Power Management Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMHDSK_A {
    #[doc = "0: No handshake. LPM is not supported"]
    NO = 0,
    #[doc = "1: ACK"]
    ACK = 1,
    #[doc = "2: NYET"]
    NYET = 2,
    #[doc = "3: STALL"]
    STALL = 3,
}
impl From<LPMHDSK_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMHDSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMHDSK` reader - Link Power Management Handshake"]
pub struct LPMHDSK_R(crate::FieldReader<u8, LPMHDSK_A>);
impl LPMHDSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPMHDSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMHDSK_A {
        match self.bits {
            0 => LPMHDSK_A::NO,
            1 => LPMHDSK_A::ACK,
            2 => LPMHDSK_A::NYET,
            3 => LPMHDSK_A::STALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == LPMHDSK_A::NO
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == LPMHDSK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NYET`"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        **self == LPMHDSK_A::NYET
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == LPMHDSK_A::STALL
    }
}
impl core::ops::Deref for LPMHDSK_R {
    type Target = crate::FieldReader<u8, LPMHDSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMHDSK` writer - Link Power Management Handshake"]
pub struct LPMHDSK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMHDSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMHDSK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No handshake. LPM is not supported"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(LPMHDSK_A::NO)
    }
    #[doc = "ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMHDSK_A::ACK)
    }
    #[doc = "NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMHDSK_A::NYET)
    }
    #[doc = "STALL"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(LPMHDSK_A::STALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u16 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NREPLY_R {
        NREPLY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&self) -> GNAK_R {
        GNAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&self) -> LPMHDSK_R {
        LPMHDSK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Detach"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W {
        DETACH_W { w: self }
    }
    #[doc = "Bit 1 - Upstream Resume"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UPRSM_W {
        UPRSM_W { w: self }
    }
    #[doc = "Bits 2:3 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Bit 4 - No Reply"]
    #[inline(always)]
    pub fn nreply(&mut self) -> NREPLY_W {
        NREPLY_W { w: self }
    }
    #[doc = "Bit 5 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W {
        TSTJ_W { w: self }
    }
    #[doc = "Bit 6 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W {
        TSTK_W { w: self }
    }
    #[doc = "Bit 7 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&mut self) -> TSTPCKT_W {
        TSTPCKT_W { w: self }
    }
    #[doc = "Bit 8 - Specific Operational Mode"]
    #[inline(always)]
    pub fn opmode2(&mut self) -> OPMODE2_W {
        OPMODE2_W { w: self }
    }
    #[doc = "Bit 9 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&mut self) -> GNAK_W {
        GNAK_W { w: self }
    }
    #[doc = "Bits 10:11 - Link Power Management Handshake"]
    #[inline(always)]
    pub fn lpmhdsk(&mut self) -> LPMHDSK_W {
        LPMHDSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0x01"]
impl crate::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
