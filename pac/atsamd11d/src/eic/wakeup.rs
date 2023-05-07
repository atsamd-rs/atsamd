#[doc = "Register `WAKEUP` reader"]
pub struct R(crate::R<WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP` writer"]
pub struct W(crate::W<WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_SPEC>;
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
impl From<crate::W<WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUPEN0` reader - External Interrupt 0 Wake-up Enable"]
pub type WAKEUPEN0_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN0` writer - External Interrupt 0 Wake-up Enable"]
pub type WAKEUPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN1` reader - External Interrupt 1 Wake-up Enable"]
pub type WAKEUPEN1_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN1` writer - External Interrupt 1 Wake-up Enable"]
pub type WAKEUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN2` reader - External Interrupt 2 Wake-up Enable"]
pub type WAKEUPEN2_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN2` writer - External Interrupt 2 Wake-up Enable"]
pub type WAKEUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN3` reader - External Interrupt 3 Wake-up Enable"]
pub type WAKEUPEN3_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN3` writer - External Interrupt 3 Wake-up Enable"]
pub type WAKEUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN4` reader - External Interrupt 4 Wake-up Enable"]
pub type WAKEUPEN4_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN4` writer - External Interrupt 4 Wake-up Enable"]
pub type WAKEUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN5` reader - External Interrupt 5 Wake-up Enable"]
pub type WAKEUPEN5_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN5` writer - External Interrupt 5 Wake-up Enable"]
pub type WAKEUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN6` reader - External Interrupt 6 Wake-up Enable"]
pub type WAKEUPEN6_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN6` writer - External Interrupt 6 Wake-up Enable"]
pub type WAKEUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
#[doc = "Field `WAKEUPEN7` reader - External Interrupt 7 Wake-up Enable"]
pub type WAKEUPEN7_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUPEN7` writer - External Interrupt 7 Wake-up Enable"]
pub type WAKEUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEUP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> WAKEUPEN0_R {
        WAKEUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> WAKEUPEN1_R {
        WAKEUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> WAKEUPEN2_R {
        WAKEUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> WAKEUPEN3_R {
        WAKEUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> WAKEUPEN4_R {
        WAKEUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> WAKEUPEN5_R {
        WAKEUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> WAKEUPEN6_R {
        WAKEUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> WAKEUPEN7_R {
        WAKEUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen0(&mut self) -> WAKEUPEN0_W<0> {
        WAKEUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen1(&mut self) -> WAKEUPEN1_W<1> {
        WAKEUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen2(&mut self) -> WAKEUPEN2_W<2> {
        WAKEUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen3(&mut self) -> WAKEUPEN3_W<3> {
        WAKEUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen4(&mut self) -> WAKEUPEN4_W<4> {
        WAKEUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen5(&mut self) -> WAKEUPEN5_W<5> {
        WAKEUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen6(&mut self) -> WAKEUPEN6_W<6> {
        WAKEUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen7(&mut self) -> WAKEUPEN7_W<7> {
        WAKEUPEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-Up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup](index.html) module"]
pub struct WAKEUP_SPEC;
impl crate::RegisterSpec for WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup::R](R) reader structure"]
impl crate::Readable for WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup::W](W) writer structure"]
impl crate::Writable for WAKEUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WAKEUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
