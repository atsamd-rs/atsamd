#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub struct OVF_R(crate::FieldReader<bool, bool>);
impl OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
#[doc = "Field `TRG` reader - Retrigger"]
pub struct TRG_R(crate::FieldReader<bool, bool>);
impl TRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRG` writer - Retrigger"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
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
#[doc = "Field `CNT` reader - Counter"]
pub struct CNT_R(crate::FieldReader<bool, bool>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Counter"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
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
#[doc = "Field `ERR` reader - Error"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - Error"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Field `DFS` reader - Non-Recoverable Debug Fault"]
pub struct DFS_R(crate::FieldReader<bool, bool>);
impl DFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS` writer - Non-Recoverable Debug Fault"]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
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
#[doc = "Field `FAULTA` reader - Recoverable Fault A"]
pub struct FAULTA_R(crate::FieldReader<bool, bool>);
impl FAULTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTA` writer - Recoverable Fault A"]
pub struct FAULTA_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTA_W<'a> {
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
#[doc = "Field `FAULTB` reader - Recoverable Fault B"]
pub struct FAULTB_R(crate::FieldReader<bool, bool>);
impl FAULTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTB` writer - Recoverable Fault B"]
pub struct FAULTB_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTB_W<'a> {
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
#[doc = "Field `FAULT0` reader - Non-Recoverable Fault 0"]
pub struct FAULT0_R(crate::FieldReader<bool, bool>);
impl FAULT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0` writer - Non-Recoverable Fault 0"]
pub struct FAULT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FAULT1` reader - Non-Recoverable Fault 1"]
pub struct FAULT1_R(crate::FieldReader<bool, bool>);
impl FAULT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1` writer - Non-Recoverable Fault 1"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MC0` reader - Match or Capture 0"]
pub struct MC0_R(crate::FieldReader<bool, bool>);
impl MC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC0` writer - Match or Capture 0"]
pub struct MC0_W<'a> {
    w: &'a mut W,
}
impl<'a> MC0_W<'a> {
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
#[doc = "Field `MC1` reader - Match or Capture 1"]
pub struct MC1_R(crate::FieldReader<bool, bool>);
impl MC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC1` writer - Match or Capture 1"]
pub struct MC1_W<'a> {
    w: &'a mut W,
}
impl<'a> MC1_W<'a> {
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
#[doc = "Field `MC2` reader - Match or Capture 2"]
pub struct MC2_R(crate::FieldReader<bool, bool>);
impl MC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC2` writer - Match or Capture 2"]
pub struct MC2_W<'a> {
    w: &'a mut W,
}
impl<'a> MC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MC3` reader - Match or Capture 3"]
pub struct MC3_R(crate::FieldReader<bool, bool>);
impl MC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC3` writer - Match or Capture 3"]
pub struct MC3_W<'a> {
    w: &'a mut W,
}
impl<'a> MC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    pub fn faulta(&self) -> FAULTA_R {
        FAULTA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    pub fn faultb(&self) -> FAULTB_R {
        FAULTB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    pub fn fault0(&self) -> FAULT0_R {
        FAULT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    pub fn mc2(&self) -> MC2_R {
        MC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    pub fn mc3(&self) -> MC3_R {
        MC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 1 - Retrigger"]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
    #[doc = "Bit 2 - Counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Bit 3 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 11 - Non-Recoverable Debug Fault"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bit 12 - Recoverable Fault A"]
    #[inline(always)]
    pub fn faulta(&mut self) -> FAULTA_W {
        FAULTA_W { w: self }
    }
    #[doc = "Bit 13 - Recoverable Fault B"]
    #[inline(always)]
    pub fn faultb(&mut self) -> FAULTB_W {
        FAULTB_W { w: self }
    }
    #[doc = "Bit 14 - Non-Recoverable Fault 0"]
    #[inline(always)]
    pub fn fault0(&mut self) -> FAULT0_W {
        FAULT0_W { w: self }
    }
    #[doc = "Bit 15 - Non-Recoverable Fault 1"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 16 - Match or Capture 0"]
    #[inline(always)]
    pub fn mc0(&mut self) -> MC0_W {
        MC0_W { w: self }
    }
    #[doc = "Bit 17 - Match or Capture 1"]
    #[inline(always)]
    pub fn mc1(&mut self) -> MC1_W {
        MC1_W { w: self }
    }
    #[doc = "Bit 18 - Match or Capture 2"]
    #[inline(always)]
    pub fn mc2(&mut self) -> MC2_W {
        MC2_W { w: self }
    }
    #[doc = "Bit 19 - Match or Capture 3"]
    #[inline(always)]
    pub fn mc3(&mut self) -> MC3_W {
        MC3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
