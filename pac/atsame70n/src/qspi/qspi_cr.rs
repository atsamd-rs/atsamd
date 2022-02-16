#[doc = "Register `QSPI_CR` writer"]
pub struct W(crate::W<QSPI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_CR_SPEC>;
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
impl From<crate::W<QSPI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPIEN` writer - QSPI Enable"]
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `QSPIDIS` writer - QSPI Disable"]
pub struct QSPIDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SWRST` writer - QSPI Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub struct LASTXFER_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTXFER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
    #[doc = "Bit 1 - QSPI Disable"]
    #[inline(always)]
    pub fn qspidis(&mut self) -> QSPIDIS_W {
        QSPIDIS_W { w: self }
    }
    #[doc = "Bit 7 - QSPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LASTXFER_W {
        LASTXFER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_cr](index.html) module"]
pub struct QSPI_CR_SPEC;
impl crate::RegisterSpec for QSPI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qspi_cr::W](W) writer structure"]
impl crate::Writable for QSPI_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_CR to value 0"]
impl crate::Resettable for QSPI_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
