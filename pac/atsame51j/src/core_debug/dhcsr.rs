#[doc = "Register `DHCSR` reader"]
pub struct R(crate::R<DHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHCSR` writer"]
pub struct W(crate::W<DHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHCSR_SPEC>;
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
impl From<crate::W<DHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C_DEBUGEN` reader - "]
pub struct C_DEBUGEN_R(crate::FieldReader<bool, bool>);
impl C_DEBUGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_DEBUGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_DEBUGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_DEBUGEN` writer - "]
pub struct C_DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DEBUGEN_W<'a> {
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
#[doc = "Field `C_HALT` reader - "]
pub struct C_HALT_R(crate::FieldReader<bool, bool>);
impl C_HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_HALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_HALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_HALT` writer - "]
pub struct C_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> C_HALT_W<'a> {
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
#[doc = "Field `C_STEP` reader - "]
pub struct C_STEP_R(crate::FieldReader<bool, bool>);
impl C_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_STEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_STEP` writer - "]
pub struct C_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> C_STEP_W<'a> {
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
#[doc = "Field `C_MASKINTS` reader - "]
pub struct C_MASKINTS_R(crate::FieldReader<bool, bool>);
impl C_MASKINTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_MASKINTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_MASKINTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_MASKINTS` writer - "]
pub struct C_MASKINTS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_MASKINTS_W<'a> {
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
#[doc = "Field `C_SNAPSTALL` reader - "]
pub struct C_SNAPSTALL_R(crate::FieldReader<bool, bool>);
impl C_SNAPSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C_SNAPSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_SNAPSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_SNAPSTALL` writer - "]
pub struct C_SNAPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> C_SNAPSTALL_W<'a> {
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
#[doc = "Field `S_REGRDY` reader - "]
pub struct S_REGRDY_R(crate::FieldReader<bool, bool>);
impl S_REGRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_REGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_REGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_HALT` reader - "]
pub struct S_HALT_R(crate::FieldReader<bool, bool>);
impl S_HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_HALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_HALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_SLEEP` reader - "]
pub struct S_SLEEP_R(crate::FieldReader<bool, bool>);
impl S_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_LOCKUP` reader - "]
pub struct S_LOCKUP_R(crate::FieldReader<bool, bool>);
impl S_LOCKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_LOCKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_LOCKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_RETIRE_ST` reader - "]
pub struct S_RETIRE_ST_R(crate::FieldReader<bool, bool>);
impl S_RETIRE_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_RETIRE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_RETIRE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_RESET_ST` reader - "]
pub struct S_RESET_ST_R(crate::FieldReader<bool, bool>);
impl S_RESET_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGKEY` writer - "]
pub struct DBGKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W {
        C_SNAPSTALL_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dbgkey(&mut self) -> DBGKEY_W {
        DBGKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Halting Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](index.html) module"]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhcsr::R](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
