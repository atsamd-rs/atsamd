#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT0` reader - External Interrupt 0"]
pub type EXTINT0_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT0` writer - External Interrupt 0"]
pub type EXTINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT1` reader - External Interrupt 1"]
pub type EXTINT1_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT1` writer - External Interrupt 1"]
pub type EXTINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT2` reader - External Interrupt 2"]
pub type EXTINT2_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT2` writer - External Interrupt 2"]
pub type EXTINT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT3` reader - External Interrupt 3"]
pub type EXTINT3_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT3` writer - External Interrupt 3"]
pub type EXTINT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT4` reader - External Interrupt 4"]
pub type EXTINT4_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT4` writer - External Interrupt 4"]
pub type EXTINT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT5` reader - External Interrupt 5"]
pub type EXTINT5_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT5` writer - External Interrupt 5"]
pub type EXTINT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT6` reader - External Interrupt 6"]
pub type EXTINT6_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT6` writer - External Interrupt 6"]
pub type EXTINT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EXTINT7` reader - External Interrupt 7"]
pub type EXTINT7_R = crate::BitReader<bool>;
#[doc = "Field `EXTINT7` writer - External Interrupt 7"]
pub type EXTINT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> EXTINT0_W<0> {
        EXTINT0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> EXTINT1_W<1> {
        EXTINT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> EXTINT2_W<2> {
        EXTINT2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> EXTINT3_W<3> {
        EXTINT3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> EXTINT4_W<4> {
        EXTINT4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> EXTINT5_W<5> {
        EXTINT5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> EXTINT6_W<6> {
        EXTINT6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> EXTINT7_W<7> {
        EXTINT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
