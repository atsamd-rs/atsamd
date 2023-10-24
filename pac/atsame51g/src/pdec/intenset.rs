#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `OVF` reader - Overflow/Underflow Interrupt Enable"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow/Underflow Interrupt Enable"]
pub type OVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR` reader - Error Interrupt Enable"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - Error Interrupt Enable"]
pub type ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIR` reader - Direction Interrupt Enable"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction Interrupt Enable"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VLC` reader - Velocity Interrupt Enable"]
pub type VLC_R = crate::BitReader;
#[doc = "Field `VLC` writer - Velocity Interrupt Enable"]
pub type VLC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MC0` reader - Channel 0 Compare Match Enable"]
pub type MC0_R = crate::BitReader;
#[doc = "Field `MC0` writer - Channel 0 Compare Match Enable"]
pub type MC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MC1` reader - Channel 1 Compare Match Enable"]
pub type MC1_R = crate::BitReader;
#[doc = "Field `MC1` writer - Channel 1 Compare Match Enable"]
pub type MC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overflow/Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Interrupt Enable"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Velocity Interrupt Enable"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Compare Match Enable"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Compare Match Enable"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow/Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INTENSET_SPEC, 0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<INTENSET_SPEC, 1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 2 - Direction Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<INTENSET_SPEC, 2> {
        DIR_W::new(self)
    }
    #[doc = "Bit 3 - Velocity Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<INTENSET_SPEC, 3> {
        VLC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> MC0_W<INTENSET_SPEC, 4> {
        MC0_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 Compare Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<INTENSET_SPEC, 5> {
        MC1_W::new(self)
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
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
