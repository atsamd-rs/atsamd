#[doc = "Register `SWTRIGCTRL` reader"]
pub type R = crate::R<SWTRIGCTRL_SPEC>;
#[doc = "Register `SWTRIGCTRL` writer"]
pub type W = crate::W<SWTRIGCTRL_SPEC>;
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub type SWTRIG0_R = crate::BitReader;
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub type SWTRIG0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub type SWTRIG1_R = crate::BitReader;
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub type SWTRIG1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG2` reader - Channel 2 Software Trigger"]
pub type SWTRIG2_R = crate::BitReader;
#[doc = "Field `SWTRIG2` writer - Channel 2 Software Trigger"]
pub type SWTRIG2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG3` reader - Channel 3 Software Trigger"]
pub type SWTRIG3_R = crate::BitReader;
#[doc = "Field `SWTRIG3` writer - Channel 3 Software Trigger"]
pub type SWTRIG3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG4` reader - Channel 4 Software Trigger"]
pub type SWTRIG4_R = crate::BitReader;
#[doc = "Field `SWTRIG4` writer - Channel 4 Software Trigger"]
pub type SWTRIG4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG5` reader - Channel 5 Software Trigger"]
pub type SWTRIG5_R = crate::BitReader;
#[doc = "Field `SWTRIG5` writer - Channel 5 Software Trigger"]
pub type SWTRIG5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG6` reader - Channel 6 Software Trigger"]
pub type SWTRIG6_R = crate::BitReader;
#[doc = "Field `SWTRIG6` writer - Channel 6 Software Trigger"]
pub type SWTRIG6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG7` reader - Channel 7 Software Trigger"]
pub type SWTRIG7_R = crate::BitReader;
#[doc = "Field `SWTRIG7` writer - Channel 7 Software Trigger"]
pub type SWTRIG7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG8` reader - Channel 8 Software Trigger"]
pub type SWTRIG8_R = crate::BitReader;
#[doc = "Field `SWTRIG8` writer - Channel 8 Software Trigger"]
pub type SWTRIG8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG9` reader - Channel 9 Software Trigger"]
pub type SWTRIG9_R = crate::BitReader;
#[doc = "Field `SWTRIG9` writer - Channel 9 Software Trigger"]
pub type SWTRIG9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG10` reader - Channel 10 Software Trigger"]
pub type SWTRIG10_R = crate::BitReader;
#[doc = "Field `SWTRIG10` writer - Channel 10 Software Trigger"]
pub type SWTRIG10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWTRIG11` reader - Channel 11 Software Trigger"]
pub type SWTRIG11_R = crate::BitReader;
#[doc = "Field `SWTRIG11` writer - Channel 11 Software Trigger"]
pub type SWTRIG11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> SWTRIG0_R {
        SWTRIG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> SWTRIG1_R {
        SWTRIG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> SWTRIG2_R {
        SWTRIG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> SWTRIG3_R {
        SWTRIG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> SWTRIG4_R {
        SWTRIG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> SWTRIG5_R {
        SWTRIG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> SWTRIG6_R {
        SWTRIG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> SWTRIG7_R {
        SWTRIG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> SWTRIG8_R {
        SWTRIG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> SWTRIG9_R {
        SWTRIG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> SWTRIG10_R {
        SWTRIG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> SWTRIG11_R {
        SWTRIG11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig0(&mut self) -> SWTRIG0_W<SWTRIGCTRL_SPEC, 0> {
        SWTRIG0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<SWTRIGCTRL_SPEC, 1> {
        SWTRIG1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<SWTRIGCTRL_SPEC, 2> {
        SWTRIG2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig3(&mut self) -> SWTRIG3_W<SWTRIGCTRL_SPEC, 3> {
        SWTRIG3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig4(&mut self) -> SWTRIG4_W<SWTRIGCTRL_SPEC, 4> {
        SWTRIG4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig5(&mut self) -> SWTRIG5_W<SWTRIGCTRL_SPEC, 5> {
        SWTRIG5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig6(&mut self) -> SWTRIG6_W<SWTRIGCTRL_SPEC, 6> {
        SWTRIG6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig7(&mut self) -> SWTRIG7_W<SWTRIGCTRL_SPEC, 7> {
        SWTRIG7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig8(&mut self) -> SWTRIG8_W<SWTRIGCTRL_SPEC, 8> {
        SWTRIG8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig9(&mut self) -> SWTRIG9_W<SWTRIGCTRL_SPEC, 9> {
        SWTRIG9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig10(&mut self) -> SWTRIG10_W<SWTRIGCTRL_SPEC, 10> {
        SWTRIG10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig11(&mut self) -> SWTRIG11_W<SWTRIGCTRL_SPEC, 11> {
        SWTRIG11_W::new(self)
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
#[doc = "Software Trigger Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrigctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrigctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRIGCTRL_SPEC;
impl crate::RegisterSpec for SWTRIGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrigctrl::R`](R) reader structure"]
impl crate::Readable for SWTRIGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swtrigctrl::W`](W) writer structure"]
impl crate::Writable for SWTRIGCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SWTRIGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
