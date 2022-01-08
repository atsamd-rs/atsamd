#[doc = "Register `PCFG` reader"]
pub struct R(crate::R<PCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG` writer"]
pub struct W(crate::W<PCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG_SPEC>;
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
impl From<crate::W<PCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub struct PTOKEN_R(crate::FieldReader<u8, u8>);
impl PTOKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTOKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTOKEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub struct PTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Field `BK` reader - Pipe Bank"]
pub struct BK_R(crate::FieldReader<bool, bool>);
impl BK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK` writer - Pipe Bank"]
pub struct BK_W<'a> {
    w: &'a mut W,
}
impl<'a> BK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub struct PTYPE_R(crate::FieldReader<u8, u8>);
impl PTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u8 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&self) -> BK_R {
        BK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W {
        PTOKEN_W { w: self }
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline(always)]
    pub fn bk(&mut self) -> BK_W {
        BK_W { w: self }
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST_PIPE End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](index.html) module"]
pub struct PCFG_SPEC;
impl crate::RegisterSpec for PCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pcfg::R](R) reader structure"]
impl crate::Readable for PCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg::W](W) writer structure"]
impl crate::Writable for PCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG to value 0"]
impl crate::Resettable for PCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
