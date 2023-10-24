#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<INTPEND_SPEC>;
#[doc = "Register `INTPEND` writer"]
pub type W = crate::W<INTPEND_SPEC>;
#[doc = "Field `ID` reader - Channel ID"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Channel ID"]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `OVR` reader - Channel Overrun"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - Channel Overrun"]
pub type OVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD` reader - Channel Event Detected"]
pub type EVD_R = crate::BitReader;
#[doc = "Field `EVD` writer - Channel Event Detected"]
pub type EVD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `READY` reader - Ready"]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - Ready"]
pub type READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Busy"]
pub type BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    pub fn evd(&self) -> EVD_R {
        EVD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<INTPEND_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = "Bit 8 - Channel Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<INTPEND_SPEC, 8> {
        OVR_W::new(self)
    }
    #[doc = "Bit 9 - Channel Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn evd(&mut self) -> EVD_W<INTPEND_SPEC, 9> {
        EVD_W::new(self)
    }
    #[doc = "Bit 14 - Ready"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<INTPEND_SPEC, 14> {
        READY_W::new(self)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<INTPEND_SPEC, 15> {
        BUSY_W::new(self)
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
#[doc = "Channel Pending Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets INTPEND to value 0x4000"]
impl crate::Resettable for INTPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
