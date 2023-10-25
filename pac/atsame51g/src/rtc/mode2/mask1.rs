#[doc = "Register `MASK1` reader"]
pub type R = crate::R<MASK1_SPEC>;
#[doc = "Register `MASK1` writer"]
pub type W = crate::W<MASK1_SPEC>;
#[doc = "Field `SEL` reader - Alarm Mask Selection"]
pub type SEL_R = crate::FieldReader<SELSELECT_A>;
#[doc = "Alarm Mask Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELSELECT_A {
    #[doc = "0: Alarm Disabled"]
    OFF = 0,
    #[doc = "1: Match seconds only"]
    SS = 1,
    #[doc = "2: Match seconds and minutes only"]
    MMSS = 2,
    #[doc = "3: Match seconds, minutes, and hours only"]
    HHMMSS = 3,
    #[doc = "4: Match seconds, minutes, hours, and days only"]
    DDHHMMSS = 4,
    #[doc = "5: Match seconds, minutes, hours, days, and months only"]
    MMDDHHMMSS = 5,
    #[doc = "6: Match seconds, minutes, hours, days, months, and years"]
    YYMMDDHHMMSS = 6,
}
impl From<SELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELSELECT_A {
    type Ux = u8;
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELSELECT_A> {
        match self.bits {
            0 => Some(SELSELECT_A::OFF),
            1 => Some(SELSELECT_A::SS),
            2 => Some(SELSELECT_A::MMSS),
            3 => Some(SELSELECT_A::HHMMSS),
            4 => Some(SELSELECT_A::DDHHMMSS),
            5 => Some(SELSELECT_A::MMDDHHMMSS),
            6 => Some(SELSELECT_A::YYMMDDHHMMSS),
            _ => None,
        }
    }
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SELSELECT_A::OFF
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn is_ss(&self) -> bool {
        *self == SELSELECT_A::SS
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn is_mmss(&self) -> bool {
        *self == SELSELECT_A::MMSS
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn is_hhmmss(&self) -> bool {
        *self == SELSELECT_A::HHMMSS
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn is_ddhhmmss(&self) -> bool {
        *self == SELSELECT_A::DDHHMMSS
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn is_mmddhhmmss(&self) -> bool {
        *self == SELSELECT_A::MMDDHHMMSS
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn is_yymmddhhmmss(&self) -> bool {
        *self == SELSELECT_A::YYMMDDHHMMSS
    }
}
#[doc = "Field `SEL` writer - Alarm Mask Selection"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SELSELECT_A>;
impl<'a, REG, const O: u8> SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::OFF)
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn ss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::SS)
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn mmss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::MMSS)
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn hhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::HHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn ddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::DDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn mmddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::MMDDHHMMSS)
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn yymmddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(SELSELECT_A::YYMMDDHHMMSS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<MASK1_SPEC, 0> {
        SEL_W::new(self)
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
#[doc = "MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASK1_SPEC;
impl crate::RegisterSpec for MASK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mask1::R`](R) reader structure"]
impl crate::Readable for MASK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mask1::W`](W) writer structure"]
impl crate::Writable for MASK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK1 to value 0"]
impl crate::Resettable for MASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
