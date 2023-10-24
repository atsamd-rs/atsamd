#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<INTPEND_SPEC>;
#[doc = "Register `INTPEND` writer"]
pub type W = crate::W<INTPEND_SPEC>;
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TERR` reader - Transfer Error"]
pub type TERR_R = crate::BitReader;
#[doc = "Field `TERR` writer - Transfer Error"]
pub type TERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCMPL` reader - Transfer Complete"]
pub type TCMPL_R = crate::BitReader;
#[doc = "Field `TCMPL` writer - Transfer Complete"]
pub type TCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSP` reader - Channel Suspend"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Channel Suspend"]
pub type SUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERR` reader - Fetch Error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PEND` reader - Pending"]
pub type PEND_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    pub fn tcmpl(&self) -> TCMPL_R {
        TCMPL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fetch Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending"]
    #[inline(always)]
    pub fn pend(&self) -> PEND_R {
        PEND_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<INTPEND_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bit 8 - Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<INTPEND_SPEC, 8> {
        TERR_W::new(self)
    }
    #[doc = "Bit 9 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpl(&mut self) -> TCMPL_W<INTPEND_SPEC, 9> {
        TCMPL_W::new(self)
    }
    #[doc = "Bit 10 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<INTPEND_SPEC, 10> {
        SUSP_W::new(self)
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
#[doc = "Interrupt Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTPEND_SPEC;
impl crate::RegisterSpec for INTPEND_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for INTPEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intpend::W`](W) writer structure"]
impl crate::Writable for INTPEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for INTPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
