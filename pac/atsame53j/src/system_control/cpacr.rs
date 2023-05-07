#[doc = "Register `CPACR` reader"]
pub struct R(crate::R<CPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPACR` writer"]
pub struct W(crate::W<CPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPACR_SPEC>;
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
impl From<crate::W<CPACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10"]
pub type CP10_R = crate::FieldReader<u8, CP10SELECT_A>;
#[doc = "Access privileges for coprocessor 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP10SELECT_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP10SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10SELECT_A) -> Self {
        variant as _
    }
}
impl CP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP10SELECT_A> {
        match self.bits {
            0 => Some(CP10SELECT_A::DENIED),
            1 => Some(CP10SELECT_A::PRIV),
            3 => Some(CP10SELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP10SELECT_A::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == CP10SELECT_A::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP10SELECT_A::FULL
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10"]
pub type CP10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP10SELECT_A, 2, O>;
impl<'a, const O: u8> CP10_W<'a, O> {
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP10SELECT_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP10SELECT_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(CP10SELECT_A::FULL)
    }
}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11"]
pub type CP11_R = crate::FieldReader<u8, CP11SELECT_A>;
#[doc = "Access privileges for coprocessor 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP11SELECT_A {
    #[doc = "0: Access denied"]
    DENIED = 0,
    #[doc = "1: Privileged access only"]
    PRIV = 1,
    #[doc = "3: Full access"]
    FULL = 3,
}
impl From<CP11SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11SELECT_A) -> Self {
        variant as _
    }
}
impl CP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP11SELECT_A> {
        match self.bits {
            0 => Some(CP11SELECT_A::DENIED),
            1 => Some(CP11SELECT_A::PRIV),
            3 => Some(CP11SELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DENIED`"]
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        *self == CP11SELECT_A::DENIED
    }
    #[doc = "Checks if the value of the field is `PRIV`"]
    #[inline(always)]
    pub fn is_priv(&self) -> bool {
        *self == CP11SELECT_A::PRIV
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CP11SELECT_A::FULL
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11"]
pub type CP11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP11SELECT_A, 2, O>;
impl<'a, const O: u8> CP11_W<'a, O> {
    #[doc = "Access denied"]
    #[inline(always)]
    pub fn denied(self) -> &'a mut W {
        self.variant(CP11SELECT_A::DENIED)
    }
    #[doc = "Privileged access only"]
    #[inline(always)]
    pub fn priv_(self) -> &'a mut W {
        self.variant(CP11SELECT_A::PRIV)
    }
    #[doc = "Full access"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(CP11SELECT_A::FULL)
    }
}
impl R {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<20> {
        CP10_W::new(self)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<22> {
        CP11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](index.html) module"]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpacr::R](R) reader structure"]
impl crate::Readable for CPACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpacr::W](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
