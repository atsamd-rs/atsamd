#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `STARTEI` reader - Start Conversion Event In"]
pub type STARTEI_R = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Conversion Event In"]
pub type STARTEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCEI` reader - Synchronization Event In"]
pub type SYNCEI_R = crate::BitReader;
#[doc = "Field `SYNCEI` writer - Synchronization Event In"]
pub type SYNCEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub type RESRDYEO_R = crate::BitReader;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub type RESRDYEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub type WINMONEO_R = crate::BitReader;
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub type WINMONEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    pub fn startei(&self) -> STARTEI_R {
        STARTEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    pub fn syncei(&self) -> SYNCEI_R {
        SYNCEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> RESRDYEO_R {
        RESRDYEO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WINMONEO_R {
        WINMONEO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    #[must_use]
    pub fn startei(&mut self) -> STARTEI_W<EVCTRL_SPEC, 0> {
        STARTEI_W::new(self)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    #[must_use]
    pub fn syncei(&mut self) -> SYNCEI_W<EVCTRL_SPEC, 1> {
        SYNCEI_W::new(self)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    #[must_use]
    pub fn resrdyeo(&mut self) -> RESRDYEO_W<EVCTRL_SPEC, 4> {
        RESRDYEO_W::new(self)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    #[must_use]
    pub fn winmoneo(&mut self) -> WINMONEO_W<EVCTRL_SPEC, 5> {
        WINMONEO_W::new(self)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
