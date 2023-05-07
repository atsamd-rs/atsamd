#[doc = "Register `BKUPCFG` reader"]
pub struct R(crate::R<BKUPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKUPCFG` writer"]
pub struct W(crate::W<BKUPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKUPCFG_SPEC>;
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
impl From<crate::W<BKUPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKUPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRAMCFG` reader - Ram Configuration"]
pub type BRAMCFG_R = crate::FieldReader<u8, BRAMCFGSELECT_A>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
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
#[doc = "Field `BRAMCFG` writer - Ram Configuration"]
pub type BRAMCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, BKUPCFG_SPEC, u8, BRAMCFGSELECT_A, 2, O>;
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
    pub fn bramcfg(&self) -> BRAMCFG_R {
        BRAMCFG_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bramcfg(&mut self) -> BRAMCFG_W<0> {
        BRAMCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupcfg](index.html) module"]
pub struct BKUPCFG_SPEC;
impl crate::RegisterSpec for BKUPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bkupcfg::R](R) reader structure"]
impl crate::Readable for BKUPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkupcfg::W](W) writer structure"]
impl crate::Writable for BKUPCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUPCFG to value 0"]
impl crate::Resettable for BKUPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
