#[doc = "Register `MASK0` reader"]
pub type R = crate::R<Mask0Spec>;
#[doc = "Register `MASK0` writer"]
pub type W = crate::W<Mask0Spec>;
#[doc = "Alarm Mask Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selselect {
    #[doc = "0: Alarm Disabled"]
    Off = 0,
    #[doc = "1: Match seconds only"]
    Ss = 1,
    #[doc = "2: Match seconds and minutes only"]
    Mmss = 2,
    #[doc = "3: Match seconds, minutes, and hours only"]
    Hhmmss = 3,
    #[doc = "4: Match seconds, minutes, hours, and days only"]
    Ddhhmmss = 4,
    #[doc = "5: Match seconds, minutes, hours, days, and months only"]
    Mmddhhmmss = 5,
    #[doc = "6: Match seconds, minutes, hours, days, months, and years"]
    Yymmddhhmmss = 6,
}
impl From<Selselect> for u8 {
    #[inline(always)]
    fn from(variant: Selselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selselect {
    type Ux = u8;
}
impl crate::IsEnum for Selselect {}
#[doc = "Field `SEL` reader - Alarm Mask Selection"]
pub type SelR = crate::FieldReader<Selselect>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selselect> {
        match self.bits {
            0 => Some(Selselect::Off),
            1 => Some(Selselect::Ss),
            2 => Some(Selselect::Mmss),
            3 => Some(Selselect::Hhmmss),
            4 => Some(Selselect::Ddhhmmss),
            5 => Some(Selselect::Mmddhhmmss),
            6 => Some(Selselect::Yymmddhhmmss),
            _ => None,
        }
    }
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Selselect::Off
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn is_ss(&self) -> bool {
        *self == Selselect::Ss
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn is_mmss(&self) -> bool {
        *self == Selselect::Mmss
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn is_hhmmss(&self) -> bool {
        *self == Selselect::Hhmmss
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn is_ddhhmmss(&self) -> bool {
        *self == Selselect::Ddhhmmss
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn is_mmddhhmmss(&self) -> bool {
        *self == Selselect::Mmddhhmmss
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn is_yymmddhhmmss(&self) -> bool {
        *self == Selselect::Yymmddhhmmss
    }
}
#[doc = "Field `SEL` writer - Alarm Mask Selection"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selselect>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alarm Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Off)
    }
    #[doc = "Match seconds only"]
    #[inline(always)]
    pub fn ss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Ss)
    }
    #[doc = "Match seconds and minutes only"]
    #[inline(always)]
    pub fn mmss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Mmss)
    }
    #[doc = "Match seconds, minutes, and hours only"]
    #[inline(always)]
    pub fn hhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Hhmmss)
    }
    #[doc = "Match seconds, minutes, hours, and days only"]
    #[inline(always)]
    pub fn ddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Ddhhmmss)
    }
    #[doc = "Match seconds, minutes, hours, days, and months only"]
    #[inline(always)]
    pub fn mmddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Mmddhhmmss)
    }
    #[doc = "Match seconds, minutes, hours, days, months, and years"]
    #[inline(always)]
    pub fn yymmddhhmmss(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::Yymmddhhmmss)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Mask Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Mask0Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mask0Spec;
impl crate::RegisterSpec for Mask0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mask0::R`](R) reader structure"]
impl crate::Readable for Mask0Spec {}
#[doc = "`write(|w| ..)` method takes [`mask0::W`](W) writer structure"]
impl crate::Writable for Mask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for Mask0Spec {
    const RESET_VALUE: u8 = 0;
}
