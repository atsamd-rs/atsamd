#[doc = "Register `SHPR1` reader"]
pub struct R(crate::R<SHPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHPR1` writer"]
pub struct W(crate::W<SHPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHPR1_SPEC>;
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
impl From<crate::W<SHPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_4` reader - Priority of system handler 4, MemManage"]
pub type PRI_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_4` writer - Priority of system handler 4, MemManage"]
pub type PRI_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_5` reader - Priority of system handler 5, BusFault"]
pub type PRI_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_5` writer - Priority of system handler 5, BusFault"]
pub type PRI_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRI_6` reader - Priority of system handler 6, UsageFault"]
pub type PRI_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRI_6` writer - Priority of system handler 6, UsageFault"]
pub type PRI_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHPR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    #[must_use]
    pub fn pri_4(&mut self) -> PRI_4_W<0> {
        PRI_4_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_5(&mut self) -> PRI_5_W<8> {
        PRI_5_W::new(self)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    #[must_use]
    pub fn pri_6(&mut self) -> PRI_6_W<16> {
        PRI_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpr1](index.html) module"]
pub struct SHPR1_SPEC;
impl crate::RegisterSpec for SHPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shpr1::R](R) reader structure"]
impl crate::Readable for SHPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shpr1::W](W) writer structure"]
impl crate::Writable for SHPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for SHPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
