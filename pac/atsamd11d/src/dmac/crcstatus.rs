#[doc = "Register `CRCSTATUS` reader"]
pub type R = crate::R<CRCSTATUS_SPEC>;
#[doc = "Register `CRCSTATUS` writer"]
pub type W = crate::W<CRCSTATUS_SPEC>;
#[doc = "Field `CRCBUSY` reader - CRC Module Busy"]
pub type CRCBUSY_R = crate::BitReader;
#[doc = "Field `CRCBUSY` writer - CRC Module Busy"]
pub type CRCBUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCZERO` reader - CRC Zero"]
pub type CRCZERO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    pub fn crcbusy(&self) -> CRCBUSY_R {
        CRCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Zero"]
    #[inline(always)]
    pub fn crczero(&self) -> CRCZERO_R {
        CRCZERO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Module Busy"]
    #[inline(always)]
    #[must_use]
    pub fn crcbusy(&mut self) -> CRCBUSY_W<CRCSTATUS_SPEC, 0> {
        CRCBUSY_W::new(self)
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
#[doc = "CRC Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCSTATUS_SPEC;
impl crate::RegisterSpec for CRCSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcstatus::R`](R) reader structure"]
impl crate::Readable for CRCSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcstatus::W`](W) writer structure"]
impl crate::Writable for CRCSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CRCSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
