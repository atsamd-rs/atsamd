#[doc = "Register `HIBCFG` reader"]
pub struct R(crate::R<HIBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIBCFG` writer"]
pub struct W(crate::W<HIBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIBCFG_SPEC>;
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
impl From<crate::W<HIBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIBCFG_SPEC>) -> Self {
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
#[doc = "Backup Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRAMCFG_A {
    #[doc = "0: All the backup RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    OFF = 2,
}
impl From<BRAMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: BRAMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BRAMCFG` reader - Backup Ram Configuration"]
pub struct BRAMCFG_R(crate::FieldReader<u8, BRAMCFG_A>);
impl BRAMCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BRAMCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRAMCFG_A> {
        match self.bits {
            0 => Some(BRAMCFG_A::RET),
            1 => Some(BRAMCFG_A::PARTIAL),
            2 => Some(BRAMCFG_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        **self == BRAMCFG_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        **self == BRAMCFG_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == BRAMCFG_A::OFF
    }
}
impl core::ops::Deref for BRAMCFG_R {
    type Target = crate::FieldReader<u8, BRAMCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRAMCFG` writer - Backup Ram Configuration"]
pub struct BRAMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRAMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRAMCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(BRAMCFG_A::RET)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(BRAMCFG_A::PARTIAL)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BRAMCFG_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u8 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RAMCFG_W {
        RAMCFG_W { w: self }
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&mut self) -> BRAMCFG_W {
        BRAMCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hibcfg](index.html) module"]
pub struct HIBCFG_SPEC;
impl crate::RegisterSpec for HIBCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hibcfg::R](R) reader structure"]
impl crate::Readable for HIBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hibcfg::W](W) writer structure"]
impl crate::Writable for HIBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HIBCFG to value 0"]
impl crate::Resettable for HIBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
