#[doc = "Register `TIDM[%s]` reader"]
pub type R = crate::R<TIDM_SPEC>;
#[doc = "Register `TIDM[%s]` writer"]
pub type W = crate::W<TIDM_SPEC>;
#[doc = "Field `TID` reader - Type ID Match 1"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 1"]
pub type TID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ENID` reader - Enable Copying of TID Matched Frames"]
pub type ENID_R = crate::BitReader;
#[doc = "Field `ENID` writer - Enable Copying of TID Matched Frames"]
pub type ENID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid(&self) -> ENID_R {
        ENID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<TIDM_SPEC, 0> {
        TID_W::new(self)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid(&mut self) -> ENID_W<TIDM_SPEC, 31> {
        ENID_W::new(self)
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
#[doc = "Type ID Match Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tidm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tidm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIDM_SPEC;
impl crate::RegisterSpec for TIDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tidm::R`](R) reader structure"]
impl crate::Readable for TIDM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tidm::W`](W) writer structure"]
impl crate::Writable for TIDM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIDM[%s]
to value 0"]
impl crate::Resettable for TIDM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
