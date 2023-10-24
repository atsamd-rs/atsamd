#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<INTFLAG_SPEC>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<INTFLAG_SPEC>;
#[doc = "Field `OVF` reader - OVF Interrupt Flag"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - OVF Interrupt Flag"]
pub type OVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR` reader - ERR Interrupt Flag"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - ERR Interrupt Flag"]
pub type ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MC0` reader - MC Interrupt Flag 0"]
pub type MC0_R = crate::BitReader;
#[doc = "Field `MC0` writer - MC Interrupt Flag 0"]
pub type MC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MC1` reader - MC Interrupt Flag 1"]
pub type MC1_R = crate::BitReader;
#[doc = "Field `MC1` writer - MC Interrupt Flag 1"]
pub type MC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    pub fn mc0(&self) -> MC0_R {
        MC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    pub fn mc1(&self) -> MC1_R {
        MC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INTFLAG_SPEC, 0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<INTFLAG_SPEC, 1> {
        ERR_W::new(self)
    }
    #[doc = "Bit 4 - MC Interrupt Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn mc0(&mut self) -> MC0_W<INTFLAG_SPEC, 4> {
        MC0_W::new(self)
    }
    #[doc = "Bit 5 - MC Interrupt Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn mc1(&mut self) -> MC1_W<INTFLAG_SPEC, 5> {
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
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
