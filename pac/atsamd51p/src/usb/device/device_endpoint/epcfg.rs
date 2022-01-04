#[doc = "Register `EPCFG` reader"]
pub struct R(crate::R<EPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCFG` writer"]
pub struct W(crate::W<EPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCFG_SPEC>;
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
impl From<crate::W<EPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPTYPE0` reader - End Point Type0"]
pub struct EPTYPE0_R(crate::FieldReader<u8, u8>);
impl EPTYPE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPTYPE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTYPE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYPE0` writer - End Point Type0"]
pub struct EPTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Field `EPTYPE1` reader - End Point Type1"]
pub struct EPTYPE1_R(crate::FieldReader<u8, u8>);
impl EPTYPE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EPTYPE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTYPE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYPE1` writer - End Point Type1"]
pub struct EPTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u8 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub struct NYETDIS_R(crate::FieldReader<bool, bool>);
impl NYETDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NYETDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYETDIS` writer - NYET Token Disable"]
pub struct NYETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - End Point Type0"]
    #[inline(always)]
    pub fn eptype0(&mut self) -> EPTYPE0_W {
        EPTYPE0_W { w: self }
    }
    #[doc = "Bits 4:6 - End Point Type1"]
    #[inline(always)]
    pub fn eptype1(&mut self) -> EPTYPE1_W {
        EPTYPE1_W { w: self }
    }
    #[doc = "Bit 7 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&mut self) -> NYETDIS_W {
        NYETDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE_ENDPOINT End Point Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcfg](index.html) module"]
pub struct EPCFG_SPEC;
impl crate::RegisterSpec for EPCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epcfg::R](R) reader structure"]
impl crate::Readable for EPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcfg::W](W) writer structure"]
impl crate::Writable for EPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPCFG to value 0"]
impl crate::Resettable for EPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
