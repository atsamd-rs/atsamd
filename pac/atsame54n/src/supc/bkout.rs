#[doc = "Register `BKOUT` reader"]
pub struct R(crate::R<BKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKOUT` writer"]
pub struct W(crate::W<BKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKOUT_SPEC>;
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
impl From<crate::W<BKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENOUT0` reader - Enable OUT0"]
pub type ENOUT0_R = crate::BitReader<bool>;
#[doc = "Field `ENOUT0` writer - Enable OUT0"]
pub type ENOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `ENOUT1` reader - Enable OUT1"]
pub type ENOUT1_R = crate::BitReader<bool>;
#[doc = "Field `ENOUT1` writer - Enable OUT1"]
pub type ENOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `CLROUT0` reader - Clear OUT0"]
pub type CLROUT0_R = crate::BitReader<bool>;
#[doc = "Field `CLROUT0` writer - Clear OUT0"]
pub type CLROUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `CLROUT1` reader - Clear OUT1"]
pub type CLROUT1_R = crate::BitReader<bool>;
#[doc = "Field `CLROUT1` writer - Clear OUT1"]
pub type CLROUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `SETOUT0` reader - Set OUT0"]
pub type SETOUT0_R = crate::BitReader<bool>;
#[doc = "Field `SETOUT0` writer - Set OUT0"]
pub type SETOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `SETOUT1` reader - Set OUT1"]
pub type SETOUT1_R = crate::BitReader<bool>;
#[doc = "Field `SETOUT1` writer - Set OUT1"]
pub type SETOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `RTCTGLOUT0` reader - RTC Toggle OUT0"]
pub type RTCTGLOUT0_R = crate::BitReader<bool>;
#[doc = "Field `RTCTGLOUT0` writer - RTC Toggle OUT0"]
pub type RTCTGLOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
#[doc = "Field `RTCTGLOUT1` reader - RTC Toggle OUT1"]
pub type RTCTGLOUT1_R = crate::BitReader<bool>;
#[doc = "Field `RTCTGLOUT1` writer - RTC Toggle OUT1"]
pub type RTCTGLOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BKOUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    pub fn enout0(&self) -> ENOUT0_R {
        ENOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    pub fn enout1(&self) -> ENOUT1_R {
        ENOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    pub fn clrout0(&self) -> CLROUT0_R {
        CLROUT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    pub fn clrout1(&self) -> CLROUT1_R {
        CLROUT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    pub fn setout0(&self) -> SETOUT0_R {
        SETOUT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    pub fn setout1(&self) -> SETOUT1_R {
        SETOUT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    pub fn rtctglout0(&self) -> RTCTGLOUT0_R {
        RTCTGLOUT0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    pub fn rtctglout1(&self) -> RTCTGLOUT1_R {
        RTCTGLOUT1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn enout0(&mut self) -> ENOUT0_W<0> {
        ENOUT0_W::new(self)
    }
    #[doc = "Bit 1 - Enable OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn enout1(&mut self) -> ENOUT1_W<1> {
        ENOUT1_W::new(self)
    }
    #[doc = "Bit 8 - Clear OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn clrout0(&mut self) -> CLROUT0_W<8> {
        CLROUT0_W::new(self)
    }
    #[doc = "Bit 9 - Clear OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn clrout1(&mut self) -> CLROUT1_W<9> {
        CLROUT1_W::new(self)
    }
    #[doc = "Bit 16 - Set OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn setout0(&mut self) -> SETOUT0_W<16> {
        SETOUT0_W::new(self)
    }
    #[doc = "Bit 17 - Set OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn setout1(&mut self) -> SETOUT1_W<17> {
        SETOUT1_W::new(self)
    }
    #[doc = "Bit 24 - RTC Toggle OUT0"]
    #[inline(always)]
    #[must_use]
    pub fn rtctglout0(&mut self) -> RTCTGLOUT0_W<24> {
        RTCTGLOUT0_W::new(self)
    }
    #[doc = "Bit 25 - RTC Toggle OUT1"]
    #[inline(always)]
    #[must_use]
    pub fn rtctglout1(&mut self) -> RTCTGLOUT1_W<25> {
        RTCTGLOUT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkout](index.html) module"]
pub struct BKOUT_SPEC;
impl crate::RegisterSpec for BKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkout::R](R) reader structure"]
impl crate::Readable for BKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkout::W](W) writer structure"]
impl crate::Writable for BKOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKOUT to value 0"]
impl crate::Resettable for BKOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
