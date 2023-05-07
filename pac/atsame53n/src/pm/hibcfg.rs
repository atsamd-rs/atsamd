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
#[doc = "Field `RAMCFG` reader - Ram Configuration"]
pub type RAMCFG_R = crate::FieldReader<u8, RAMCFGSELECT_A>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMCFGSELECT_A {
    #[doc = "0: All the system RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 32Kbytes of the system RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the system RAM is turned OFF"]
    OFF = 2,
}
impl From<RAMCFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMCFGSELECT_A) -> Self {
        variant as _
    }
}
impl RAMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMCFGSELECT_A> {
        match self.bits {
            0 => Some(RAMCFGSELECT_A::RET),
            1 => Some(RAMCFGSELECT_A::PARTIAL),
            2 => Some(RAMCFGSELECT_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == RAMCFGSELECT_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == RAMCFGSELECT_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMCFGSELECT_A::OFF
    }
}
#[doc = "Field `RAMCFG` writer - Ram Configuration"]
pub type RAMCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, HIBCFG_SPEC, u8, RAMCFGSELECT_A, 2, O>;
impl<'a, const O: u8> RAMCFG_W<'a, O> {
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(RAMCFGSELECT_A::RET)
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(RAMCFGSELECT_A::PARTIAL)
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RAMCFGSELECT_A::OFF)
    }
}
#[doc = "Field `BRAMCFG` reader - Backup Ram Configuration"]
pub type BRAMCFG_R = crate::FieldReader<u8, BRAMCFGSELECT_A>;
#[doc = "Backup Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRAMCFGSELECT_A {
    #[doc = "0: All the backup RAM is retained"]
    RET = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    PARTIAL = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    OFF = 2,
}
impl From<BRAMCFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BRAMCFGSELECT_A) -> Self {
        variant as _
    }
}
impl BRAMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRAMCFGSELECT_A> {
        match self.bits {
            0 => Some(BRAMCFGSELECT_A::RET),
            1 => Some(BRAMCFGSELECT_A::PARTIAL),
            2 => Some(BRAMCFGSELECT_A::OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET`"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == BRAMCFGSELECT_A::RET
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == BRAMCFGSELECT_A::PARTIAL
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BRAMCFGSELECT_A::OFF
    }
}
#[doc = "Field `BRAMCFG` writer - Backup Ram Configuration"]
pub type BRAMCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, HIBCFG_SPEC, u8, BRAMCFGSELECT_A, 2, O>;
impl<'a, const O: u8> BRAMCFG_W<'a, O> {
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut W {
        self.variant(BRAMCFGSELECT_A::RET)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(BRAMCFGSELECT_A::PARTIAL)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BRAMCFGSELECT_A::OFF)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RAMCFG_R {
        RAMCFG_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfg(&mut self) -> RAMCFG_W<0> {
        RAMCFG_W::new(self)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bramcfg(&mut self) -> BRAMCFG_W<2> {
        BRAMCFG_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIBCFG to value 0"]
impl crate::Resettable for HIBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
