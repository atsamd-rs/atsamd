#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN0` reader - Clock Unit 0 Enable"]
pub type CKEN0_R = crate::BitReader;
#[doc = "Field `CKEN0` writer - Clock Unit 0 Enable"]
pub type CKEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN1` reader - Clock Unit 1 Enable"]
pub type CKEN1_R = crate::BitReader;
#[doc = "Field `CKEN1` writer - Clock Unit 1 Enable"]
pub type CKEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEREN0` reader - Serializer 0 Enable"]
pub type SEREN0_R = crate::BitReader;
#[doc = "Field `SEREN0` writer - Serializer 0 Enable"]
pub type SEREN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEREN1` reader - Serializer 1 Enable"]
pub type SEREN1_R = crate::BitReader;
#[doc = "Field `SEREN1` writer - Serializer 1 Enable"]
pub type SEREN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn swrst(&mut self) -> SWRST_W<CTRLA_SPEC, 0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC, 1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Clock Unit 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken0(&mut self) -> CKEN0_W<CTRLA_SPEC, 2> {
        CKEN0_W::new(self)
    }
    #[doc = "Bit 3 - Clock Unit 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken1(&mut self) -> CKEN1_W<CTRLA_SPEC, 3> {
        CKEN1_W::new(self)
    }
    #[doc = "Bit 4 - Serializer 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seren0(&mut self) -> SEREN0_W<CTRLA_SPEC, 4> {
        SEREN0_W::new(self)
    }
    #[doc = "Bit 5 - Serializer 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seren1(&mut self) -> SEREN1_W<CTRLA_SPEC, 5> {
        SEREN1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
