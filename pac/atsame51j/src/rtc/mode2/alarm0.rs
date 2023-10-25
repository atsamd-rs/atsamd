#[doc = "Register `ALARM0` reader"]
pub type R = crate::R<ALARM0_SPEC>;
#[doc = "Register `ALARM0` writer"]
pub type W = crate::W<ALARM0_SPEC>;
#[doc = "Field `SECOND` reader - Second"]
pub type SECOND_R = crate::FieldReader;
#[doc = "Field `SECOND` writer - Second"]
pub type SECOND_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `MINUTE` reader - Minute"]
pub type MINUTE_R = crate::FieldReader;
#[doc = "Field `MINUTE` writer - Minute"]
pub type MINUTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `HOUR` reader - Hour"]
pub type HOUR_R = crate::FieldReader<HOURSELECT_A>;
#[doc = "Hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HOURSELECT_A {
    #[doc = "0: Morning hour"]
    AM = 0,
    #[doc = "16: Afternoon hour"]
    PM = 16,
}
impl From<HOURSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HOURSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HOURSELECT_A {
    type Ux = u8;
}
impl HOUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HOURSELECT_A> {
        match self.bits {
            0 => Some(HOURSELECT_A::AM),
            16 => Some(HOURSELECT_A::PM),
            _ => None,
        }
    }
    #[doc = "Morning hour"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == HOURSELECT_A::AM
    }
    #[doc = "Afternoon hour"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == HOURSELECT_A::PM
    }
}
#[doc = "Field `HOUR` writer - Hour"]
pub type HOUR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O, HOURSELECT_A>;
impl<'a, REG, const O: u8> HOUR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Morning hour"]
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> {
        self.variant(HOURSELECT_A::AM)
    }
    #[doc = "Afternoon hour"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(HOURSELECT_A::PM)
    }
}
#[doc = "Field `DAY` reader - Day"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `DAY` writer - Day"]
pub type DAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `MONTH` reader - Month"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month"]
pub type MONTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `YEAR` writer - Year"]
pub type YEAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    #[must_use]
    pub fn second(&mut self) -> SECOND_W<ALARM0_SPEC, 0> {
        SECOND_W::new(self)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    #[must_use]
    pub fn minute(&mut self) -> MINUTE_W<ALARM0_SPEC, 6> {
        MINUTE_W::new(self)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<ALARM0_SPEC, 12> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<ALARM0_SPEC, 17> {
        DAY_W::new(self)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<ALARM0_SPEC, 22> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<ALARM0_SPEC, 26> {
        YEAR_W::new(self)
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
#[doc = "MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_SPEC;
impl crate::RegisterSpec for ALARM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0::R`](R) reader structure"]
impl crate::Readable for ALARM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0::W`](W) writer structure"]
impl crate::Writable for ALARM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARM0 to value 0"]
impl crate::Resettable for ALARM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
