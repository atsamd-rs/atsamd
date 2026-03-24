#[doc = "Register `RTCCTRL` reader"]
pub type R = crate::R<RtcctrlSpec>;
#[doc = "Register `RTCCTRL` writer"]
pub type W = crate::W<RtcctrlSpec>;
#[doc = "RTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcselselect {
    #[doc = "0: 1.024kHz from 32kHz internal ULP oscillator"]
    Ulp1k = 0,
    #[doc = "1: 32.768kHz from 32kHz internal ULP oscillator"]
    Ulp32k = 1,
    #[doc = "4: 1.024kHz from 32.768kHz internal oscillator"]
    Xosc1k = 4,
    #[doc = "5: 32.768kHz from 32.768kHz external crystal oscillator"]
    Xosc32k = 5,
}
impl From<Rtcselselect> for u8 {
    #[inline(always)]
    fn from(variant: Rtcselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcselselect {
    type Ux = u8;
}
impl crate::IsEnum for Rtcselselect {}
#[doc = "Field `RTCSEL` reader - RTC Clock Selection"]
pub type RtcselR = crate::FieldReader<Rtcselselect>;
impl RtcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcselselect> {
        match self.bits {
            0 => Some(Rtcselselect::Ulp1k),
            1 => Some(Rtcselselect::Ulp32k),
            4 => Some(Rtcselselect::Xosc1k),
            5 => Some(Rtcselselect::Xosc32k),
            _ => None,
        }
    }
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp1k(&self) -> bool {
        *self == Rtcselselect::Ulp1k
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn is_ulp32k(&self) -> bool {
        *self == Rtcselselect::Ulp32k
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn is_xosc1k(&self) -> bool {
        *self == Rtcselselect::Xosc1k
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == Rtcselselect::Xosc32k
    }
}
#[doc = "Field `RTCSEL` writer - RTC Clock Selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rtcselselect>;
impl<'a, REG> RtcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.024kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Ulp1k)
    }
    #[doc = "32.768kHz from 32kHz internal ULP oscillator"]
    #[inline(always)]
    pub fn ulp32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Ulp32k)
    }
    #[doc = "1.024kHz from 32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn xosc1k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc1k)
    }
    #[doc = "32.768kHz from 32.768kHz external crystal oscillator"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcselselect::Xosc32k)
    }
}
impl R {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RtcselW<RtcctrlSpec> {
        RtcselW::new(self, 0)
    }
}
#[doc = "RTC Clock Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctrlSpec;
impl crate::RegisterSpec for RtcctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcctrl::R`](R) reader structure"]
impl crate::Readable for RtcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctrl::W`](W) writer structure"]
impl crate::Writable for RtcctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RTCCTRL to value 0"]
impl crate::Resettable for RtcctrlSpec {
    const RESET_VALUE: u8 = 0;
}
