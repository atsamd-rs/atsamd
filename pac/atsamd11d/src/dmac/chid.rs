#[doc = "Register `CHID` reader"]
pub type R = crate::R<CHID_SPEC>;
#[doc = "Register `CHID` writer"]
pub type W = crate::W<CHID_SPEC>;
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<CHID_SPEC, 0> {
        ID_W::new(self)
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
#[doc = "Channel ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHID_SPEC;
impl crate::RegisterSpec for CHID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chid::R`](R) reader structure"]
impl crate::Readable for CHID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chid::W`](W) writer structure"]
impl crate::Writable for CHID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHID to value 0"]
impl crate::Resettable for CHID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
