#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICDIS` reader - Instruction Cache Disable"]
pub struct ICDIS_R(crate::FieldReader<bool, bool>);
impl ICDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICDIS` writer - Instruction Cache Disable"]
pub struct ICDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICDIS_W<'a> {
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
#[doc = "Field `DCDIS` reader - Data Cache Disable"]
pub struct DCDIS_R(crate::FieldReader<bool, bool>);
impl DCDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDIS` writer - Data Cache Disable"]
pub struct DCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDIS_W<'a> {
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
#[doc = "Cache size configured by software\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSIZESW_A {
    #[doc = "0: The Cache Size is configured to 1KB"]
    CONF_CSIZE_1KB = 0,
    #[doc = "1: The Cache Size is configured to 2KB"]
    CONF_CSIZE_2KB = 1,
    #[doc = "2: The Cache Size is configured to 4KB"]
    CONF_CSIZE_4KB = 2,
    #[doc = "3: The Cache Size is configured to 8KB"]
    CONF_CSIZE_8KB = 3,
    #[doc = "4: The Cache Size is configured to 16KB"]
    CONF_CSIZE_16KB = 4,
    #[doc = "5: The Cache Size is configured to 32KB"]
    CONF_CSIZE_32KB = 5,
    #[doc = "6: The Cache Size is configured to 64KB"]
    CONF_CSIZE_64KB = 6,
}
impl From<CSIZESW_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSIZESW` reader - Cache size configured by software"]
pub struct CSIZESW_R(crate::FieldReader<u8, CSIZESW_A>);
impl CSIZESW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSIZESW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZESW_A> {
        match self.bits {
            0 => Some(CSIZESW_A::CONF_CSIZE_1KB),
            1 => Some(CSIZESW_A::CONF_CSIZE_2KB),
            2 => Some(CSIZESW_A::CONF_CSIZE_4KB),
            3 => Some(CSIZESW_A::CONF_CSIZE_8KB),
            4 => Some(CSIZESW_A::CONF_CSIZE_16KB),
            5 => Some(CSIZESW_A::CONF_CSIZE_32KB),
            6 => Some(CSIZESW_A::CONF_CSIZE_64KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_conf_csize_1kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_conf_csize_2kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_conf_csize_4kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_conf_csize_8kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_8KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_16KB`"]
    #[inline(always)]
    pub fn is_conf_csize_16kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_16KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_32KB`"]
    #[inline(always)]
    pub fn is_conf_csize_32kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_32KB
    }
    #[doc = "Checks if the value of the field is `CONF_CSIZE_64KB`"]
    #[inline(always)]
    pub fn is_conf_csize_64kb(&self) -> bool {
        **self == CSIZESW_A::CONF_CSIZE_64KB
    }
}
impl core::ops::Deref for CSIZESW_R {
    type Target = crate::FieldReader<u8, CSIZESW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIZESW` writer - Cache size configured by software"]
pub struct CSIZESW_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIZESW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIZESW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The Cache Size is configured to 1KB"]
    #[inline(always)]
    pub fn conf_csize_1kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_1KB)
    }
    #[doc = "The Cache Size is configured to 2KB"]
    #[inline(always)]
    pub fn conf_csize_2kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_2KB)
    }
    #[doc = "The Cache Size is configured to 4KB"]
    #[inline(always)]
    pub fn conf_csize_4kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_4KB)
    }
    #[doc = "The Cache Size is configured to 8KB"]
    #[inline(always)]
    pub fn conf_csize_8kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_8KB)
    }
    #[doc = "The Cache Size is configured to 16KB"]
    #[inline(always)]
    pub fn conf_csize_16kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_16KB)
    }
    #[doc = "The Cache Size is configured to 32KB"]
    #[inline(always)]
    pub fn conf_csize_32kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_32KB)
    }
    #[doc = "The Cache Size is configured to 64KB"]
    #[inline(always)]
    pub fn conf_csize_64kb(self) -> &'a mut W {
        self.variant(CSIZESW_A::CONF_CSIZE_64KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&self) -> ICDIS_R {
        ICDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&self) -> DCDIS_R {
        DCDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&self) -> CSIZESW_R {
        CSIZESW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Instruction Cache Disable"]
    #[inline(always)]
    pub fn icdis(&mut self) -> ICDIS_W {
        ICDIS_W { w: self }
    }
    #[doc = "Bit 2 - Data Cache Disable"]
    #[inline(always)]
    pub fn dcdis(&mut self) -> DCDIS_W {
        DCDIS_W { w: self }
    }
    #[doc = "Bits 4:6 - Cache size configured by software"]
    #[inline(always)]
    pub fn csizesw(&mut self) -> CSIZESW_W {
        CSIZESW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x20"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
