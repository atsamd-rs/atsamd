#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `INVEN0` reader - Output Waveform 0 Invert Enable"]
pub type INVEN0_R = crate::BitReader;
#[doc = "Field `INVEN0` writer - Output Waveform 0 Invert Enable"]
pub type INVEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVEN1` reader - Output Waveform 1 Invert Enable"]
pub type INVEN1_R = crate::BitReader;
#[doc = "Field `INVEN1` writer - Output Waveform 1 Invert Enable"]
pub type INVEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub type CPTEN0_R = crate::BitReader;
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub type CPTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub type CPTEN1_R = crate::BitReader;
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub type CPTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform 0 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inven0(&mut self) -> INVEN0_W<CTRLC_SPEC, 0> {
        INVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Output Waveform 1 Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> INVEN1_W<CTRLC_SPEC, 1> {
        INVEN1_W::new(self)
    }
    #[doc = "Bit 4 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten0(&mut self) -> CPTEN0_W<CTRLC_SPEC, 4> {
        CPTEN0_W::new(self)
    }
    #[doc = "Bit 5 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten1(&mut self) -> CPTEN1_W<CTRLC_SPEC, 5> {
        CPTEN1_W::new(self)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
