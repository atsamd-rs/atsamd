#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `EVACT` reader - Event Action"]
pub type EVACT_R = crate::FieldReader<EVACTSELECT_A>;
#[doc = "Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVACTSELECT_A {
    #[doc = "0: Event action disabled"]
    OFF = 0,
    #[doc = "1: Start, restart or retrigger on event"]
    RETRIGGER = 1,
    #[doc = "2: Count on event"]
    COUNT = 2,
}
impl From<EVACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVACTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EVACTSELECT_A {
    type Ux = u8;
}
impl EVACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EVACTSELECT_A> {
        match self.bits {
            0 => Some(EVACTSELECT_A::OFF),
            1 => Some(EVACTSELECT_A::RETRIGGER),
            2 => Some(EVACTSELECT_A::COUNT),
            _ => None,
        }
    }
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EVACTSELECT_A::OFF
    }
    #[doc = "Start, restart or retrigger on event"]
    #[inline(always)]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACTSELECT_A::RETRIGGER
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn is_count(&self) -> bool {
        *self == EVACTSELECT_A::COUNT
    }
}
#[doc = "Field `EVACT` writer - Event Action"]
pub type EVACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, EVACTSELECT_A>;
impl<'a, REG, const O: u8> EVACT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event action disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::OFF)
    }
    #[doc = "Start, restart or retrigger on event"]
    #[inline(always)]
    pub fn retrigger(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline(always)]
    pub fn count(self) -> &'a mut crate::W<REG> {
        self.variant(EVACTSELECT_A::COUNT)
    }
}
#[doc = "Field `EVINV` reader - Inverted Event Input Enable"]
pub type EVINV_R = crate::FieldReader;
#[doc = "Field `EVINV` writer - Inverted Event Input Enable"]
pub type EVINV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `EVEI` reader - Event Input Enable"]
pub type EVEI_R = crate::FieldReader;
#[doc = "Field `EVEI` writer - Event Input Enable"]
pub type EVEI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
pub type OVFEO_R = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
pub type OVFEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERREO` reader - Error Output Event Enable"]
pub type ERREO_R = crate::BitReader;
#[doc = "Field `ERREO` writer - Error Output Event Enable"]
pub type ERREO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIREO` reader - Direction Output Event Enable"]
pub type DIREO_R = crate::BitReader;
#[doc = "Field `DIREO` writer - Direction Output Event Enable"]
pub type DIREO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VLCEO` reader - Velocity Output Event Enable"]
pub type VLCEO_R = crate::BitReader;
#[doc = "Field `VLCEO` writer - Velocity Output Event Enable"]
pub type VLCEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCEO0` reader - Match Channel 0 Event Output Enable"]
pub type MCEO0_R = crate::BitReader;
#[doc = "Field `MCEO0` writer - Match Channel 0 Event Output Enable"]
pub type MCEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCEO1` reader - Match Channel 1 Event Output Enable"]
pub type MCEO1_R = crate::BitReader;
#[doc = "Field `MCEO1` writer - Match Channel 1 Event Output Enable"]
pub type MCEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Event Action"]
    #[inline(always)]
    pub fn evact(&self) -> EVACT_R {
        EVACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn evinv(&self) -> EVINV_R {
        EVINV_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    pub fn evei(&self) -> EVEI_R {
        EVEI_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    pub fn erreo(&self) -> ERREO_R {
        ERREO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    pub fn direo(&self) -> DIREO_R {
        DIREO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    pub fn vlceo(&self) -> VLCEO_R {
        VLCEO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> MCEO0_R {
        MCEO0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> MCEO1_R {
        MCEO1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn evact(&mut self) -> EVACT_W<EVCTRL_SPEC, 0> {
        EVACT_W::new(self)
    }
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evinv(&mut self) -> EVINV_W<EVCTRL_SPEC, 2> {
        EVINV_W::new(self)
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evei(&mut self) -> EVEI_W<EVCTRL_SPEC, 5> {
        EVEI_W::new(self)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OVFEO_W<EVCTRL_SPEC, 8> {
        OVFEO_W::new(self)
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erreo(&mut self) -> ERREO_W<EVCTRL_SPEC, 9> {
        ERREO_W::new(self)
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn direo(&mut self) -> DIREO_W<EVCTRL_SPEC, 10> {
        DIREO_W::new(self)
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlceo(&mut self) -> VLCEO_W<EVCTRL_SPEC, 11> {
        VLCEO_W::new(self)
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> MCEO0_W<EVCTRL_SPEC, 12> {
        MCEO0_W::new(self)
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> MCEO1_W<EVCTRL_SPEC, 13> {
        MCEO1_W::new(self)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u16;
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
