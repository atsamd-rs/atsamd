#[doc = "Register `PMC_PCR` reader"]
pub struct R(crate::R<PMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PCR` writer"]
pub struct W(crate::W<PMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCR_SPEC>;
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
impl From<crate::W<PMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Peripheral ID"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - Peripheral ID"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Generic Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GCLKCSS_A {
    #[doc = "0: Slow clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLL Clock is selected"]
    UPLL_CLK = 3,
    #[doc = "4: Master Clock is selected"]
    MCK_CLK = 4,
}
impl From<GCLKCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: GCLKCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GCLKCSS` reader - Generic Clock Source Selection"]
pub struct GCLKCSS_R(crate::FieldReader<u8, GCLKCSS_A>);
impl GCLKCSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GCLKCSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GCLKCSS_A> {
        match self.bits {
            0 => Some(GCLKCSS_A::SLOW_CLK),
            1 => Some(GCLKCSS_A::MAIN_CLK),
            2 => Some(GCLKCSS_A::PLLA_CLK),
            3 => Some(GCLKCSS_A::UPLL_CLK),
            4 => Some(GCLKCSS_A::MCK_CLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        **self == GCLKCSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        **self == GCLKCSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        **self == GCLKCSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        **self == GCLKCSS_A::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK_CLK`"]
    #[inline(always)]
    pub fn is_mck_clk(&self) -> bool {
        **self == GCLKCSS_A::MCK_CLK
    }
}
impl core::ops::Deref for GCLKCSS_R {
    type Target = crate::FieldReader<u8, GCLKCSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLKCSS` writer - Generic Clock Source Selection"]
pub struct GCLKCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCLKCSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::SLOW_CLK)
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::MCK_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<bool, bool>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
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
#[doc = "Field `GCLKDIV` reader - Generic Clock Division Ratio"]
pub struct GCLKDIV_R(crate::FieldReader<u8, u8>);
impl GCLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLKDIV` writer - Generic Clock Division Ratio"]
pub struct GCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `GCLKEN` reader - Generic Clock Enable"]
pub struct GCLKEN_R(crate::FieldReader<bool, bool>);
impl GCLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLKEN` writer - Generic Clock Enable"]
pub struct GCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&self) -> GCLKCSS_R {
        GCLKCSS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&self) -> GCLKDIV_R {
        GCLKDIV_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&self) -> GCLKEN_R {
        GCLKEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&mut self) -> GCLKCSS_W {
        GCLKCSS_W { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&mut self) -> GCLKDIV_W {
        GCLKDIV_W { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&mut self) -> GCLKEN_W {
        GCLKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcr](index.html) module"]
pub struct PMC_PCR_SPEC;
impl crate::RegisterSpec for PMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pcr::R](R) reader structure"]
impl crate::Readable for PMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcr::W](W) writer structure"]
impl crate::Writable for PMC_PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_PCR to value 0"]
impl crate::Resettable for PMC_PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
