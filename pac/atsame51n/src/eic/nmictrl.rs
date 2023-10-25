#[doc = "Register `NMICTRL` reader"]
pub type R = crate::R<NMICTRL_SPEC>;
#[doc = "Register `NMICTRL` writer"]
pub type W = crate::W<NMICTRL_SPEC>;
#[doc = "Field `NMISENSE` reader - Non-Maskable Interrupt Sense Configuration"]
pub type NMISENSE_R = crate::FieldReader<NMISENSESELECT_A>;
#[doc = "Non-Maskable Interrupt Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NMISENSESELECT_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising-edge detection"]
    RISE = 1,
    #[doc = "2: Falling-edge detection"]
    FALL = 2,
    #[doc = "3: Both-edges detection"]
    BOTH = 3,
    #[doc = "4: High-level detection"]
    HIGH = 4,
    #[doc = "5: Low-level detection"]
    LOW = 5,
}
impl From<NMISENSESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NMISENSESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NMISENSESELECT_A {
    type Ux = u8;
}
impl NMISENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NMISENSESELECT_A> {
        match self.bits {
            0 => Some(NMISENSESELECT_A::NONE),
            1 => Some(NMISENSESELECT_A::RISE),
            2 => Some(NMISENSESELECT_A::FALL),
            3 => Some(NMISENSESELECT_A::BOTH),
            4 => Some(NMISENSESELECT_A::HIGH),
            5 => Some(NMISENSESELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NMISENSESELECT_A::NONE
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == NMISENSESELECT_A::RISE
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == NMISENSESELECT_A::FALL
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == NMISENSESELECT_A::BOTH
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NMISENSESELECT_A::HIGH
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NMISENSESELECT_A::LOW
    }
}
#[doc = "Field `NMISENSE` writer - Non-Maskable Interrupt Sense Configuration"]
pub type NMISENSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, NMISENSESELECT_A>;
impl<'a, REG, const O: u8> NMISENSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(NMISENSESELECT_A::LOW)
    }
}
#[doc = "Field `NMIFILTEN` reader - Non-Maskable Interrupt Filter Enable"]
pub type NMIFILTEN_R = crate::BitReader;
#[doc = "Field `NMIFILTEN` writer - Non-Maskable Interrupt Filter Enable"]
pub type NMIFILTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NMIASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type NMIASYNCH_R = crate::BitReader<NMIASYNCHSELECT_A>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIASYNCHSELECT_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<NMIASYNCHSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NMIASYNCHSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIASYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NMIASYNCHSELECT_A {
        match self.bits {
            false => NMIASYNCHSELECT_A::SYNC,
            true => NMIASYNCHSELECT_A::ASYNC,
        }
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == NMIASYNCHSELECT_A::SYNC
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == NMIASYNCHSELECT_A::ASYNC
    }
}
#[doc = "Field `NMIASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type NMIASYNCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NMIASYNCHSELECT_A>;
impl<'a, REG, const O: u8> NMIASYNCH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(NMIASYNCHSELECT_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(NMIASYNCHSELECT_A::ASYNC)
    }
}
impl R {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&self) -> NMISENSE_R {
        NMISENSE_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NMIFILTEN_R {
        NMIFILTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn nmiasynch(&self) -> NMIASYNCH_R {
        NMIASYNCH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nmisense(&mut self) -> NMISENSE_W<NMICTRL_SPEC, 0> {
        NMISENSE_W::new(self)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmifilten(&mut self) -> NMIFILTEN_W<NMICTRL_SPEC, 3> {
        NMIFILTEN_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn nmiasynch(&mut self) -> NMIASYNCH_W<NMICTRL_SPEC, 4> {
        NMIASYNCH_W::new(self)
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
#[doc = "Non-Maskable Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmictrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmictrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMICTRL_SPEC;
impl crate::RegisterSpec for NMICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nmictrl::R`](R) reader structure"]
impl crate::Readable for NMICTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmictrl::W`](W) writer structure"]
impl crate::Writable for NMICTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NMICTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
