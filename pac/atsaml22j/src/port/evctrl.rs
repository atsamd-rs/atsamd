#[doc = "Register `EVCTRL%s` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL%s` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID0` reader - Port Event Pin Identifier 0"]
pub type PID0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID0` writer - Port Event Pin Identifier 0"]
pub type PID0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `EVACT0` reader - Port Event Action 0"]
pub type EVACT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVACT0` writer - Port Event Action 0"]
pub type EVACT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORTEI0` reader - Port Event Enable Input 0"]
pub type PORTEI0_R = crate::BitReader<bool>;
#[doc = "Field `PORTEI0` writer - Port Event Enable Input 0"]
pub type PORTEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `PID1` reader - Port Event Pin Identifier 1"]
pub type PID1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID1` writer - Port Event Pin Identifier 1"]
pub type PID1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `EVACT1` reader - Port Event Action 1"]
pub type EVACT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVACT1` writer - Port Event Action 1"]
pub type EVACT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORTEI1` reader - Port Event Enable Input 1"]
pub type PORTEI1_R = crate::BitReader<bool>;
#[doc = "Field `PORTEI1` writer - Port Event Enable Input 1"]
pub type PORTEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `PID2` reader - Port Event Pin Identifier 2"]
pub type PID2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID2` writer - Port Event Pin Identifier 2"]
pub type PID2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `EVACT2` reader - Port Event Action 2"]
pub type EVACT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVACT2` writer - Port Event Action 2"]
pub type EVACT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORTEI2` reader - Port Event Enable Input 2"]
pub type PORTEI2_R = crate::BitReader<bool>;
#[doc = "Field `PORTEI2` writer - Port Event Enable Input 2"]
pub type PORTEI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
#[doc = "Field `PID3` reader - Port Event Pin Identifier 3"]
pub type PID3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID3` writer - Port Event Pin Identifier 3"]
pub type PID3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `EVACT3` reader - Port Event Action 3"]
pub type EVACT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVACT3` writer - Port Event Action 3"]
pub type EVACT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORTEI3` reader - Port Event Enable Input 3"]
pub type PORTEI3_R = crate::BitReader<bool>;
#[doc = "Field `PORTEI3` writer - Port Event Enable Input 3"]
pub type PORTEI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Port Event Pin Identifier 0"]
    #[inline(always)]
    pub fn pid0(&self) -> PID0_R {
        PID0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Port Event Action 0"]
    #[inline(always)]
    pub fn evact0(&self) -> EVACT0_R {
        EVACT0_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Port Event Enable Input 0"]
    #[inline(always)]
    pub fn portei0(&self) -> PORTEI0_R {
        PORTEI0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Port Event Pin Identifier 1"]
    #[inline(always)]
    pub fn pid1(&self) -> PID1_R {
        PID1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Port Event Action 1"]
    #[inline(always)]
    pub fn evact1(&self) -> EVACT1_R {
        EVACT1_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port Event Enable Input 1"]
    #[inline(always)]
    pub fn portei1(&self) -> PORTEI1_R {
        PORTEI1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Port Event Pin Identifier 2"]
    #[inline(always)]
    pub fn pid2(&self) -> PID2_R {
        PID2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Port Event Action 2"]
    #[inline(always)]
    pub fn evact2(&self) -> EVACT2_R {
        EVACT2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Port Event Enable Input 2"]
    #[inline(always)]
    pub fn portei2(&self) -> PORTEI2_R {
        PORTEI2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Port Event Pin Identifier 3"]
    #[inline(always)]
    pub fn pid3(&self) -> PID3_R {
        PID3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Port Event Action 3"]
    #[inline(always)]
    pub fn evact3(&self) -> EVACT3_R {
        EVACT3_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Port Event Enable Input 3"]
    #[inline(always)]
    pub fn portei3(&self) -> PORTEI3_R {
        PORTEI3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Port Event Pin Identifier 0"]
    #[inline(always)]
    #[must_use]
    pub fn pid0(&mut self) -> PID0_W<0> {
        PID0_W::new(self)
    }
    #[doc = "Bits 5:6 - Port Event Action 0"]
    #[inline(always)]
    #[must_use]
    pub fn evact0(&mut self) -> EVACT0_W<5> {
        EVACT0_W::new(self)
    }
    #[doc = "Bit 7 - Port Event Enable Input 0"]
    #[inline(always)]
    #[must_use]
    pub fn portei0(&mut self) -> PORTEI0_W<7> {
        PORTEI0_W::new(self)
    }
    #[doc = "Bits 8:12 - Port Event Pin Identifier 1"]
    #[inline(always)]
    #[must_use]
    pub fn pid1(&mut self) -> PID1_W<8> {
        PID1_W::new(self)
    }
    #[doc = "Bits 13:14 - Port Event Action 1"]
    #[inline(always)]
    #[must_use]
    pub fn evact1(&mut self) -> EVACT1_W<13> {
        EVACT1_W::new(self)
    }
    #[doc = "Bit 15 - Port Event Enable Input 1"]
    #[inline(always)]
    #[must_use]
    pub fn portei1(&mut self) -> PORTEI1_W<15> {
        PORTEI1_W::new(self)
    }
    #[doc = "Bits 16:20 - Port Event Pin Identifier 2"]
    #[inline(always)]
    #[must_use]
    pub fn pid2(&mut self) -> PID2_W<16> {
        PID2_W::new(self)
    }
    #[doc = "Bits 21:22 - Port Event Action 2"]
    #[inline(always)]
    #[must_use]
    pub fn evact2(&mut self) -> EVACT2_W<21> {
        EVACT2_W::new(self)
    }
    #[doc = "Bit 23 - Port Event Enable Input 2"]
    #[inline(always)]
    #[must_use]
    pub fn portei2(&mut self) -> PORTEI2_W<23> {
        PORTEI2_W::new(self)
    }
    #[doc = "Bits 24:28 - Port Event Pin Identifier 3"]
    #[inline(always)]
    #[must_use]
    pub fn pid3(&mut self) -> PID3_W<24> {
        PID3_W::new(self)
    }
    #[doc = "Bits 29:30 - Port Event Action 3"]
    #[inline(always)]
    #[must_use]
    pub fn evact3(&mut self) -> EVACT3_W<29> {
        EVACT3_W::new(self)
    }
    #[doc = "Bit 31 - Port Event Enable Input 3"]
    #[inline(always)]
    #[must_use]
    pub fn portei3(&mut self) -> PORTEI3_W<31> {
        PORTEI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL%s to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
