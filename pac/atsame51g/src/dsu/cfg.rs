#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `LQOS` reader - Latency Quality Of Service"]
pub type LQOS_R = crate::FieldReader;
#[doc = "Field `LQOS` writer - Latency Quality Of Service"]
pub type LQOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DCCDMALEVEL` reader - DMA Trigger Level"]
pub type DCCDMALEVEL_R = crate::FieldReader<DCCDMALEVELSELECT_A>;
#[doc = "DMA Trigger Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCCDMALEVELSELECT_A {
    #[doc = "0: Trigger rises when DCC is empty"]
    EMPTY = 0,
    #[doc = "1: Trigger rises when DCC is full"]
    FULL = 1,
}
impl From<DCCDMALEVELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DCCDMALEVELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCCDMALEVELSELECT_A {
    type Ux = u8;
}
impl DCCDMALEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCCDMALEVELSELECT_A> {
        match self.bits {
            0 => Some(DCCDMALEVELSELECT_A::EMPTY),
            1 => Some(DCCDMALEVELSELECT_A::FULL),
            _ => None,
        }
    }
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == DCCDMALEVELSELECT_A::EMPTY
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DCCDMALEVELSELECT_A::FULL
    }
}
#[doc = "Field `DCCDMALEVEL` writer - DMA Trigger Level"]
pub type DCCDMALEVEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, DCCDMALEVELSELECT_A>;
impl<'a, REG, const O: u8> DCCDMALEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger rises when DCC is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(DCCDMALEVELSELECT_A::EMPTY)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DCCDMALEVELSELECT_A::FULL)
    }
}
#[doc = "Field `ETBRAMEN` reader - Trace Control"]
pub type ETBRAMEN_R = crate::BitReader;
#[doc = "Field `ETBRAMEN` writer - Trace Control"]
pub type ETBRAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    pub fn lqos(&self) -> LQOS_R {
        LQOS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    pub fn dccdmalevel(&self) -> DCCDMALEVEL_R {
        DCCDMALEVEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    pub fn etbramen(&self) -> ETBRAMEN_R {
        ETBRAMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline(always)]
    #[must_use]
    pub fn lqos(&mut self) -> LQOS_W<CFG_SPEC, 0> {
        LQOS_W::new(self)
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn dccdmalevel(&mut self) -> DCCDMALEVEL_W<CFG_SPEC, 2> {
        DCCDMALEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline(always)]
    #[must_use]
    pub fn etbramen(&mut self) -> ETBRAMEN_W<CFG_SPEC, 4> {
        ETBRAMEN_W::new(self)
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
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x02"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
