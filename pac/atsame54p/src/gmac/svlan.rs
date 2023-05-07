#[doc = "Register `SVLAN` reader"]
pub struct R(crate::R<SVLAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVLAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVLAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVLAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVLAN` writer"]
pub struct W(crate::W<SVLAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVLAN_SPEC>;
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
impl From<crate::W<SVLAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVLAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLAN_TYPE` reader - User Defined VLAN_TYPE Field"]
pub type VLAN_TYPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLAN_TYPE` writer - User Defined VLAN_TYPE Field"]
pub type VLAN_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SVLAN_SPEC, u16, u16, 16, O>;
#[doc = "Field `ESVLAN` reader - Enable Stacked VLAN Processing Mode"]
pub type ESVLAN_R = crate::BitReader<bool>;
#[doc = "Field `ESVLAN` writer - Enable Stacked VLAN Processing Mode"]
pub type ESVLAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVLAN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&self) -> VLAN_TYPE_R {
        VLAN_TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&self) -> ESVLAN_R {
        ESVLAN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_type(&mut self) -> VLAN_TYPE_W<0> {
        VLAN_TYPE_W::new(self)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn esvlan(&mut self) -> ESVLAN_W<31> {
        ESVLAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svlan](index.html) module"]
pub struct SVLAN_SPEC;
impl crate::RegisterSpec for SVLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svlan::R](R) reader structure"]
impl crate::Readable for SVLAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svlan::W](W) writer structure"]
impl crate::Writable for SVLAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVLAN to value 0"]
impl crate::Resettable for SVLAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
