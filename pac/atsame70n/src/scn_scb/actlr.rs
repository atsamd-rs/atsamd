#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
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
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISFOLD` reader - Disables folding of IT instructions"]
pub struct DISFOLD_R(crate::FieldReader<bool, bool>);
impl DISFOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISFOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISFOLD` writer - Disables folding of IT instructions"]
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
#[doc = "Field `FPEXCODIS` reader - Disables FPU exception outputs"]
pub struct FPEXCODIS_R(crate::FieldReader<bool, bool>);
impl FPEXCODIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPEXCODIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPEXCODIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPEXCODIS` writer - Disables FPU exception outputs"]
pub struct FPEXCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEXCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DISRAMODE` reader - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub struct DISRAMODE_R(crate::FieldReader<bool, bool>);
impl DISRAMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISRAMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISRAMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISRAMODE` writer - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
pub struct DISRAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISRAMODE_W<'a> {
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
#[doc = "Field `DISITMATBFLUSH` reader - Disables ITM and DWT ATB flush"]
pub struct DISITMATBFLUSH_R(crate::FieldReader<bool, bool>);
impl DISITMATBFLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISITMATBFLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISITMATBFLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISITMATBFLUSH` writer - Disables ITM and DWT ATB flush"]
pub struct DISITMATBFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> DISITMATBFLUSH_W<'a> {
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
#[doc = "Field `DISBTACREAD` reader - "]
pub struct DISBTACREAD_R(crate::FieldReader<bool, bool>);
impl DISBTACREAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISBTACREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISBTACREAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISBTACREAD` writer - "]
pub struct DISBTACREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISBTACREAD_W<'a> {
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
#[doc = "Field `DISBTACALLOC` reader - "]
pub struct DISBTACALLOC_R(crate::FieldReader<bool, bool>);
impl DISBTACALLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISBTACALLOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISBTACALLOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISBTACALLOC` writer - "]
pub struct DISBTACALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DISBTACALLOC_W<'a> {
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
#[doc = "Field `DISCRITAXIRUR` reader - "]
pub struct DISCRITAXIRUR_R(crate::FieldReader<bool, bool>);
impl DISCRITAXIRUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCRITAXIRUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCRITAXIRUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCRITAXIRUR` writer - "]
pub struct DISCRITAXIRUR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRITAXIRUR_W<'a> {
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
#[doc = "Field `DISDI` reader - "]
pub struct DISDI_R(crate::FieldReader<u8, u8>);
impl DISDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DISDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISDI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISDI` writer - "]
pub struct DISDI_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `DISISSCH1` reader - "]
pub struct DISISSCH1_R(crate::FieldReader<u8, u8>);
impl DISISSCH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DISISSCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISISSCH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISISSCH1` writer - "]
pub struct DISISSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DISISSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `DISDYNADD` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub struct DISDYNADD_R(crate::FieldReader<bool, bool>);
impl DISDYNADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISDYNADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISDYNADD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISDYNADD` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub struct DISDYNADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDYNADD_W<'a> {
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
#[doc = "Field `DISCRITAXIRUW` reader - Disable critical AXI read-under-write"]
pub struct DISCRITAXIRUW_R(crate::FieldReader<bool, bool>);
impl DISCRITAXIRUW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCRITAXIRUW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCRITAXIRUW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCRITAXIRUW` writer - Disable critical AXI read-under-write"]
pub struct DISCRITAXIRUW_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRITAXIRUW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DISFPUISSOPT` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub struct DISFPUISSOPT_R(crate::FieldReader<bool, bool>);
impl DISFPUISSOPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISFPUISSOPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISFPUISSOPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISFPUISSOPT` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub struct DISFPUISSOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFPUISSOPT_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn disbtacread(&self) -> DISBTACREAD_R {
        DISBTACREAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn disbtacalloc(&self) -> DISBTACALLOC_R {
        DISBTACALLOC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn discritaxirur(&self) -> DISCRITAXIRUR_R {
        DISCRITAXIRUR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn disdi(&self) -> DISDI_R {
        DISDI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn disissch1(&self) -> DISISSCH1_R {
        DISISSCH1_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&self) -> DISDYNADD_R {
        DISDYNADD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&self) -> DISCRITAXIRUW_R {
        DISCRITAXIRUW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disfpuissopt(&self) -> DISFPUISSOPT_R {
        DISFPUISSOPT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disables folding of IT instructions"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    #[doc = "Bit 10 - Disables FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W {
        FPEXCODIS_W { w: self }
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions"]
    #[inline(always)]
    pub fn disramode(&mut self) -> DISRAMODE_W {
        DISRAMODE_W { w: self }
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush"]
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W {
        DISITMATBFLUSH_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn disbtacread(&mut self) -> DISBTACREAD_W {
        DISBTACREAD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn disbtacalloc(&mut self) -> DISBTACALLOC_W {
        DISBTACALLOC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn discritaxirur(&mut self) -> DISCRITAXIRUR_W {
        DISCRITAXIRUR_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn disdi(&mut self) -> DISDI_W {
        DISDI_W { w: self }
    }
    #[doc = "Bits 21:25"]
    #[inline(always)]
    pub fn disissch1(&mut self) -> DISISSCH1_W {
        DISISSCH1_W { w: self }
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&mut self) -> DISDYNADD_W {
        DISDYNADD_W { w: self }
    }
    #[doc = "Bit 27 - Disable critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&mut self) -> DISCRITAXIRUW_W {
        DISCRITAXIRUW_W { w: self }
    }
    #[doc = "Bit 28 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disfpuissopt(&mut self) -> DISFPUISSOPT_W {
        DISFPUISSOPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
