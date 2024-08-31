#[doc = "Register `WINCTRL` reader"]
pub type R = crate::R<WinctrlSpec>;
#[doc = "Register `WINCTRL` writer"]
pub type W = crate::W<WinctrlSpec>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No window mode (default)"]
    Disable = 0,
    #[doc = "1: Mode 1: RESULT > WINLT"]
    Mode1 = 1,
    #[doc = "2: Mode 2: RESULT &lt; WINUT"]
    Mode2 = 2,
    #[doc = "3: Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    Mode3 = 3,
    #[doc = "4: Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    Mode4 = 4,
}
impl From<Winmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Winmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Winmodeselect {}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WinmodeR = crate::FieldReader<Winmodeselect>;
impl WinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Winmodeselect> {
        match self.bits {
            0 => Some(Winmodeselect::Disable),
            1 => Some(Winmodeselect::Mode1),
            2 => Some(Winmodeselect::Mode2),
            3 => Some(Winmodeselect::Mode3),
            4 => Some(Winmodeselect::Mode4),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Winmodeselect::Disable
    }
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Winmodeselect::Mode1
    }
    #[doc = "Mode 2: RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Winmodeselect::Mode2
    }
    #[doc = "Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Winmodeselect::Mode3
    }
    #[doc = "Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Winmodeselect::Mode4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WinmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Winmodeselect>;
impl<'a, REG> WinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Disable)
    }
    #[doc = "Mode 1: RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode1)
    }
    #[doc = "Mode 2: RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode2)
    }
    #[doc = "Mode 3: WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode3)
    }
    #[doc = "Mode 4: !(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WinmodeW<WinctrlSpec> {
        WinmodeW::new(self, 0)
    }
}
#[doc = "Window Monitor Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinctrlSpec;
impl crate::RegisterSpec for WinctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`winctrl::R`](R) reader structure"]
impl crate::Readable for WinctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`winctrl::W`](W) writer structure"]
impl crate::Writable for WinctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WINCTRL to value 0"]
impl crate::Resettable for WinctrlSpec {
    const RESET_VALUE: u8 = 0;
}
