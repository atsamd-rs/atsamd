#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LENGTH_SPEC>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LENGTH_SPEC>;
#[doc = "Field `LEN` reader - Data Length"]
pub type LEN_R = crate::FieldReader;
#[doc = "Field `LEN` writer - Data Length"]
pub type LEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LENEN` reader - Data Length Enable"]
pub type LENEN_R = crate::BitReader;
#[doc = "Field `LENEN` writer - Data Length Enable"]
pub type LENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<LENGTH_SPEC, 0> {
        LEN_W::new(self)
    }
    #[doc = "Bit 8 - Data Length Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lenen(&mut self) -> LENEN_W<LENGTH_SPEC, 8> {
        LENEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPIM Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
