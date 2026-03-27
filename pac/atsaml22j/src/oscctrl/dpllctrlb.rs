#[doc = "Register `DPLLCTRLB` reader"]
pub struct R(crate::R<DPLLCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLCTRLB` writer"]
pub struct W(crate::W<DPLLCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLCTRLB_SPEC>;
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
impl From<crate::W<DPLLCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub type FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub type FILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLCTRLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPEN` reader - Low-Power Enable"]
pub type LPEN_R = crate::BitReader<bool>;
#[doc = "Field `LPEN` writer - Low-Power Enable"]
pub type LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPLLCTRLB_SPEC, bool, O>;
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub type WUF_R = crate::BitReader<bool>;
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub type WUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPLLCTRLB_SPEC, bool, O>;
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub type REFCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub type REFCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLCTRLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `LTIME` reader - Lock Time"]
pub type LTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LTIME` writer - Lock Time"]
pub type LTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLCTRLB_SPEC, u8, u8, 3, O>;
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub type LBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub type LBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPLLCTRLB_SPEC, bool, O>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPLLCTRLB_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<0> {
        FILTER_W::new(self)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LPEN_W<2> {
        LPEN_W::new(self)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<3> {
        WUF_W::new(self)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> REFCLK_W<4> {
        REFCLK_W::new(self)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    #[must_use]
    pub fn ltime(&mut self) -> LTIME_W<8> {
        LTIME_W::new(self)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lbypass(&mut self) -> LBYPASS_W<12> {
        LBYPASS_W::new(self)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Core Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](index.html) module"]
pub struct DPLLCTRLB_SPEC;
impl crate::RegisterSpec for DPLLCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllctrlb::R](R) reader structure"]
impl crate::Readable for DPLLCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](W) writer structure"]
impl crate::Writable for DPLLCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0"]
impl crate::Resettable for DPLLCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
