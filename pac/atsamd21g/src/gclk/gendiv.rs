#[doc = "Register `GENDIV` reader"]
pub type R = crate::R<GENDIV_SPEC>;
#[doc = "Register `GENDIV` writer"]
pub type W = crate::W<GENDIV_SPEC>;
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<GENDIV_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<GENDIV_SPEC, 8> {
        DIV_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Generic Clock Generator Division\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gendiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gendiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GENDIV_SPEC;
impl crate::RegisterSpec for GENDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gendiv::R`](R) reader structure"]
impl crate::Readable for GENDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gendiv::W`](W) writer structure"]
impl crate::Writable for GENDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GENDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
