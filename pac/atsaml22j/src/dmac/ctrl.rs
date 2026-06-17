#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `DMAENABLE` reader - DMA Enable"]
pub type DMAENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DMAENABLE` writer - DMA Enable"]
pub type DMAENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `CRCENABLE` reader - CRC Enable"]
pub type CRCENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CRCENABLE` writer - CRC Enable"]
pub type CRCENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `LVLEN0` reader - Priority Level 0 Enable"]
pub type LVLEN0_R = crate::BitReader<bool>;
#[doc = "Field `LVLEN0` writer - Priority Level 0 Enable"]
pub type LVLEN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `LVLEN1` reader - Priority Level 1 Enable"]
pub type LVLEN1_R = crate::BitReader<bool>;
#[doc = "Field `LVLEN1` writer - Priority Level 1 Enable"]
pub type LVLEN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `LVLEN2` reader - Priority Level 2 Enable"]
pub type LVLEN2_R = crate::BitReader<bool>;
#[doc = "Field `LVLEN2` writer - Priority Level 2 Enable"]
pub type LVLEN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
#[doc = "Field `LVLEN3` reader - Priority Level 3 Enable"]
pub type LVLEN3_R = crate::BitReader<bool>;
#[doc = "Field `LVLEN3` writer - Priority Level 3 Enable"]
pub type LVLEN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DMAENABLE_R {
        DMAENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    pub fn lvlen0(&self) -> LVLEN0_R {
        LVLEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    pub fn lvlen1(&self) -> LVLEN1_R {
        LVLEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    pub fn lvlen2(&self) -> LVLEN2_R {
        LVLEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    pub fn lvlen3(&self) -> LVLEN3_R {
        LVLEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaenable(&mut self) -> DMAENABLE_W<1> {
        DMAENABLE_W::new(self)
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcenable(&mut self) -> CRCENABLE_W<2> {
        CRCENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen0(&mut self) -> LVLEN0_W<8> {
        LVLEN0_W::new(self)
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen1(&mut self) -> LVLEN1_W<9> {
        LVLEN1_W::new(self)
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen2(&mut self) -> LVLEN2_W<10> {
        LVLEN2_W::new(self)
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen3(&mut self) -> LVLEN3_W<11> {
        LVLEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
