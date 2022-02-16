#[doc = "Register `MCAN_RWD` reader"]
pub struct R(crate::R<MCAN_RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_RWD` writer"]
pub struct W(crate::W<MCAN_RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_RWD_SPEC>;
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
impl From<crate::W<MCAN_RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_RWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDC` reader - Watchdog Configuration (read/write)"]
pub struct WDC_R(crate::FieldReader<u8, u8>);
impl WDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDC` writer - Watchdog Configuration (read/write)"]
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `WDV` reader - Watchdog Value (read-only)"]
pub struct WDV_R(crate::FieldReader<u8, u8>);
impl WDV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDV` writer - Watchdog Value (read-only)"]
pub struct WDV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Watchdog Configuration (read/write)"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog Value (read-only)"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Configuration (read/write)"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
    #[doc = "Bits 8:15 - Watchdog Value (read-only)"]
    #[inline(always)]
    pub fn wdv(&mut self) -> WDV_W {
        WDV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rwd](index.html) module"]
pub struct MCAN_RWD_SPEC;
impl crate::RegisterSpec for MCAN_RWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rwd::R](R) reader structure"]
impl crate::Readable for MCAN_RWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_rwd::W](W) writer structure"]
impl crate::Writable for MCAN_RWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_RWD to value 0"]
impl crate::Resettable for MCAN_RWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
