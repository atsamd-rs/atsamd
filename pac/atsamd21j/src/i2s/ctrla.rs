#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable"]
pub type CKEN0_R = crate::BitReader<bool>;
#[doc = "Field `CKEN0` writer - Clock Unit 0 Enable"]
pub type CKEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable"]
pub type CKEN1_R = crate::BitReader<bool>;
#[doc = "Field `CKEN1` writer - Clock Unit 1 Enable"]
pub type CKEN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `SEREN0` reader - Serializer 0 Enable"]
pub type SEREN0_R = crate::BitReader<bool>;
#[doc = "Field `SEREN0` writer - Serializer 0 Enable"]
pub type SEREN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `SEREN1` reader - Serializer 1 Enable"]
pub type SEREN1_R = crate::BitReader<bool>;
#[doc = "Field `SEREN1` writer - Serializer 1 Enable"]
pub type SEREN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable"]
    #[inline(always)]
    pub fn cken0(&self) -> CKEN0_R {
        CKEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serializer 0 Enable"]
    #[inline(always)]
    pub fn seren0(&self) -> SEREN0_R {
        SEREN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serializer 1 Enable"]
    #[inline(always)]
    pub fn seren1(&self) -> SEREN1_R {
        SEREN1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken0(&mut self) -> CKEN0_W<2> {
        CKEN0_W::new(self)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken1(&mut self) -> CKEN1_W<3> {
        CKEN1_W::new(self)
    }
    #[doc = "Bit 4 - Serializer 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seren0(&mut self) -> SEREN0_W<4> {
        SEREN0_W::new(self)
    }
    #[doc = "Bit 5 - Serializer 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seren1(&mut self) -> SEREN1_W<5> {
        SEREN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
