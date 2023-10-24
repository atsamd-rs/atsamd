#[doc = "Register `DADD` reader"]
pub type R = crate::R<DADD_SPEC>;
#[doc = "Register `DADD` writer"]
pub type W = crate::W<DADD_SPEC>;
#[doc = "Field `DADD` reader - Device Address"]
pub type DADD_R = crate::FieldReader;
#[doc = "Field `DADD` writer - Device Address"]
pub type DADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ADDEN` reader - Device Address Enable"]
pub type ADDEN_R = crate::BitReader;
#[doc = "Field `ADDEN` writer - Device Address Enable"]
pub type ADDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&self) -> DADD_R {
        DADD_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dadd(&mut self) -> DADD_W<DADD_SPEC, 0> {
        DADD_W::new(self)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> ADDEN_W<DADD_SPEC, 7> {
        ADDEN_W::new(self)
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
#[doc = "DEVICE Device Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADD_SPEC;
impl crate::RegisterSpec for DADD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dadd::R`](R) reader structure"]
impl crate::Readable for DADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dadd::W`](W) writer structure"]
impl crate::Writable for DADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADD to value 0"]
impl crate::Resettable for DADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
