#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `OVR0` reader - Channel 0 Overrun Interrupt Enable"]
pub type OVR0_R = crate::BitReader;
#[doc = "Field `OVR0` writer - Channel 0 Overrun Interrupt Enable"]
pub type OVR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR1` reader - Channel 1 Overrun Interrupt Enable"]
pub type OVR1_R = crate::BitReader;
#[doc = "Field `OVR1` writer - Channel 1 Overrun Interrupt Enable"]
pub type OVR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR2` reader - Channel 2 Overrun Interrupt Enable"]
pub type OVR2_R = crate::BitReader;
#[doc = "Field `OVR2` writer - Channel 2 Overrun Interrupt Enable"]
pub type OVR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR3` reader - Channel 3 Overrun Interrupt Enable"]
pub type OVR3_R = crate::BitReader;
#[doc = "Field `OVR3` writer - Channel 3 Overrun Interrupt Enable"]
pub type OVR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR4` reader - Channel 4 Overrun Interrupt Enable"]
pub type OVR4_R = crate::BitReader;
#[doc = "Field `OVR4` writer - Channel 4 Overrun Interrupt Enable"]
pub type OVR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVR5` reader - Channel 5 Overrun Interrupt Enable"]
pub type OVR5_R = crate::BitReader;
#[doc = "Field `OVR5` writer - Channel 5 Overrun Interrupt Enable"]
pub type OVR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD0` reader - Channel 0 Event Detection Interrupt Enable"]
pub type EVD0_R = crate::BitReader;
#[doc = "Field `EVD0` writer - Channel 0 Event Detection Interrupt Enable"]
pub type EVD0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD1` reader - Channel 1 Event Detection Interrupt Enable"]
pub type EVD1_R = crate::BitReader;
#[doc = "Field `EVD1` writer - Channel 1 Event Detection Interrupt Enable"]
pub type EVD1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD2` reader - Channel 2 Event Detection Interrupt Enable"]
pub type EVD2_R = crate::BitReader;
#[doc = "Field `EVD2` writer - Channel 2 Event Detection Interrupt Enable"]
pub type EVD2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD3` reader - Channel 3 Event Detection Interrupt Enable"]
pub type EVD3_R = crate::BitReader;
#[doc = "Field `EVD3` writer - Channel 3 Event Detection Interrupt Enable"]
pub type EVD3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD4` reader - Channel 4 Event Detection Interrupt Enable"]
pub type EVD4_R = crate::BitReader;
#[doc = "Field `EVD4` writer - Channel 4 Event Detection Interrupt Enable"]
pub type EVD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVD5` reader - Channel 5 Event Detection Interrupt Enable"]
pub type EVD5_R = crate::BitReader;
#[doc = "Field `EVD5` writer - Channel 5 Event Detection Interrupt Enable"]
pub type EVD5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&self) -> OVR4_R {
        OVR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&self) -> OVR5_R {
        OVR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&self) -> EVD0_R {
        EVD0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&self) -> EVD1_R {
        EVD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&self) -> EVD2_R {
        EVD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&self) -> EVD3_R {
        EVD3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&self) -> EVD4_R {
        EVD4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&self) -> EVD5_R {
        EVD5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr0(&mut self) -> OVR0_W<INTENCLR_SPEC, 0> {
        OVR0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr1(&mut self) -> OVR1_W<INTENCLR_SPEC, 1> {
        OVR1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr2(&mut self) -> OVR2_W<INTENCLR_SPEC, 2> {
        OVR2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr3(&mut self) -> OVR3_W<INTENCLR_SPEC, 3> {
        OVR3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr4(&mut self) -> OVR4_W<INTENCLR_SPEC, 4> {
        OVR4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr5(&mut self) -> OVR5_W<INTENCLR_SPEC, 5> {
        OVR5_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd0(&mut self) -> EVD0_W<INTENCLR_SPEC, 8> {
        EVD0_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd1(&mut self) -> EVD1_W<INTENCLR_SPEC, 9> {
        EVD1_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd2(&mut self) -> EVD2_W<INTENCLR_SPEC, 10> {
        EVD2_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd3(&mut self) -> EVD3_W<INTENCLR_SPEC, 11> {
        EVD3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd4(&mut self) -> EVD4_W<INTENCLR_SPEC, 12> {
        EVD4_W::new(self)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evd5(&mut self) -> EVD5_W<INTENCLR_SPEC, 13> {
        EVD5_W::new(self)
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
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
