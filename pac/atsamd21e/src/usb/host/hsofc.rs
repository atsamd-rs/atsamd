#[doc = "Register `HSOFC` reader"]
pub type R = crate::R<HSOFC_SPEC>;
#[doc = "Register `HSOFC` writer"]
pub type W = crate::W<HSOFC_SPEC>;
#[doc = "Field `FLENC` reader - Frame Length Control"]
pub type FLENC_R = crate::FieldReader;
#[doc = "Field `FLENC` writer - Frame Length Control"]
pub type FLENC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FLENCE` reader - Frame Length Control Enable"]
pub type FLENCE_R = crate::BitReader;
#[doc = "Field `FLENCE` writer - Frame Length Control Enable"]
pub type FLENCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&self) -> FLENC_R {
        FLENC_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&self) -> FLENCE_R {
        FLENCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn flenc(&mut self) -> FLENC_W<HSOFC_SPEC, 0> {
        FLENC_W::new(self)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flence(&mut self) -> FLENCE_W<HSOFC_SPEC, 7> {
        FLENCE_W::new(self)
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
#[doc = "HOST Host Start Of Frame Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsofc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsofc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSOFC_SPEC;
impl crate::RegisterSpec for HSOFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hsofc::R`](R) reader structure"]
impl crate::Readable for HSOFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsofc::W`](W) writer structure"]
impl crate::Writable for HSOFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSOFC to value 0"]
impl crate::Resettable for HSOFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
