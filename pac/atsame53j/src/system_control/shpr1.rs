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
pub struct PRI_4_R(crate::FieldReader<u8, u8>);
impl PRI_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_4` writer - Priority of system handler 4, MemManage"]
pub struct PRI_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PRI_5` reader - Priority of system handler 5, BusFault"]
pub struct PRI_5_R(crate::FieldReader<u8, u8>);
impl PRI_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_5` writer - Priority of system handler 5, BusFault"]
pub struct PRI_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PRI_6` reader - Priority of system handler 6, UsageFault"]
pub struct PRI_6_R(crate::FieldReader<u8, u8>);
impl PRI_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_6` writer - Priority of system handler 6, UsageFault"]
pub struct PRI_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
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
    pub fn pri_4(&mut self) -> PRI_4_W {
        PRI_4_W { w: self }
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&mut self) -> PRI_5_W {
        PRI_5_W { w: self }
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&mut self) -> PRI_6_W {
        PRI_6_W { w: self }
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
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for SHPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
