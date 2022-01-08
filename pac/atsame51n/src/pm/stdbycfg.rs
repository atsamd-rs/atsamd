#[doc = "Register `STDBYCFG` reader"]
pub struct R(crate::R<STDBYCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDBYCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDBYCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDBYCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDBYCFG` writer"]
pub struct W(crate::W<STDBYCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDBYCFG_SPEC>;
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
impl From<crate::W<STDBYCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDBYCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMCFG_A {
    #[doc = "0: All the system RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 32Kbytes of the system RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the system RAM is turned OFF"]
    OFF = 2,
}
impl From<RAMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAMCFG` reader - Ram Configuration"]
pub struct RAMCFG_R(crate::FieldReader<u8, RAMCFG_A>);
impl RAMCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAMCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMCFG_A> {
        match self.bits {
            0 => Some(RAMCFG_A::RET),
            1 => Some(RAMCFG_A::PARTIAL),
            2 => Some(RAMCFG_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        **self == RAMCFG_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        **self == RAMCFG_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == RAMCFG_A::OFF
    }
}
impl core::ops::Deref for RAMCFG_R {
    type Target = crate::FieldReader<u8, RAMCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMCFG` writer - Ram Configuration"]
pub struct RAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(RAMCFG_A::RET)
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(RAMCFG_A::PARTIAL)
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RAMCFG_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Fast Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FASTWKUP_A {
    #[doc = "0: Fast Wakeup is disabled"]
    NO = 0,
    #[doc = "1: Fast Wakeup is enabled on NVM"]
    NVM = 1,
    #[doc = "2: Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    MAINVREG = 2,
    #[doc = "3: Fast Wakeup is enabled on both NVM and MAINVREG"]
    BOTH = 3,
}
impl From<FASTWKUP_A> for u8 {
    #[inline(always)]
    fn from(variant: FASTWKUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FASTWKUP` reader - Fast Wakeup"]
pub struct FASTWKUP_R(crate::FieldReader<u8, FASTWKUP_A>);
impl FASTWKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FASTWKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTWKUP_A {
        match self.bits {
            0 => FASTWKUP_A::NO,
            1 => FASTWKUP_A::NVM,
            2 => FASTWKUP_A::MAINVREG,
            3 => FASTWKUP_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == FASTWKUP_A::NO
    }
    #[doc = "Checks if the value of the field is `NVM`"]
    #[inline(always)]
    pub fn is_nvm(&self) -> bool {
        **self == FASTWKUP_A::NVM
    }
    #[doc = "Checks if the value of the field is `MAINVREG`"]
    #[inline(always)]
    pub fn is_mainvreg(&self) -> bool {
        **self == FASTWKUP_A::MAINVREG
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == FASTWKUP_A::BOTH
    }
}
impl core::ops::Deref for FASTWKUP_R {
    type Target = crate::FieldReader<u8, FASTWKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wakeup"]
pub struct FASTWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTWKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FASTWKUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Fast Wakeup is disabled"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(FASTWKUP_A::NO)
    }
    #[doc = "Fast Wakeup is enabled on NVM"]
    #[inline(always)]
    pub fn nvm(self) -> &'a mut W {
        self.variant(FASTWKUP_A::NVM)
    }
    #[doc = "Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    #[inline(always)]
    pub fn mainvreg(self) -> &'a mut W {
        self.variant(FASTWKUP_A::MAINVREG)
    }
    #[doc = "Fast Wakeup is enabled on both NVM and MAINVREG"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(FASTWKUP_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RAMCFG_W {
        RAMCFG_W { w: self }
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W {
        FASTWKUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](index.html) module"]
pub struct STDBYCFG_SPEC;
impl crate::RegisterSpec for STDBYCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stdbycfg::R](R) reader structure"]
impl crate::Readable for STDBYCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](W) writer structure"]
impl crate::Writable for STDBYCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STDBYCFG to value 0"]
impl crate::Resettable for STDBYCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
