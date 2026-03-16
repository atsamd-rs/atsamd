#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LeftadjR = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LeftadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FreerunR = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub type CorrenR = crate::BitReader;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub type CorrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Conversion Result Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resselselect {
    #[doc = "0: 12-bit result"]
    _12bit = 0,
    #[doc = "1: For averaging mode output"]
    _16bit = 1,
    #[doc = "2: 10-bit result"]
    _10bit = 2,
    #[doc = "3: 8-bit result"]
    _8bit = 3,
}
impl From<Resselselect> for u8 {
    #[inline(always)]
    fn from(variant: Resselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resselselect {
    type Ux = u8;
}
impl crate::IsEnum for Resselselect {}
#[doc = "Field `RESSEL` reader - Conversion Result Resolution"]
pub type ResselR = crate::FieldReader<Resselselect>;
impl ResselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resselselect {
        match self.bits {
            0 => Resselselect::_12bit,
            1 => Resselselect::_16bit,
            2 => Resselselect::_10bit,
            3 => Resselselect::_8bit,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Resselselect::_12bit
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Resselselect::_16bit
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Resselselect::_10bit
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Resselselect::_8bit
    }
}
#[doc = "Field `RESSEL` writer - Conversion Result Resolution"]
pub type ResselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resselselect, crate::Safe>;
impl<'a, REG> ResselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit result"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_12bit)
    }
    #[doc = "For averaging mode output"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_16bit)
    }
    #[doc = "10-bit result"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_10bit)
    }
    #[doc = "8-bit result"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_8bit)
    }
}
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No window mode (default)"]
    Disable = 0,
    #[doc = "1: RESULT > WINLT"]
    Mode1 = 1,
    #[doc = "2: RESULT &lt; WINUT"]
    Mode2 = 2,
    #[doc = "3: WINLT &lt; RESULT &lt; WINUT"]
    Mode3 = 3,
    #[doc = "4: !(WINLT &lt; RESULT &lt; WINUT)"]
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
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Winmodeselect::Mode1
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Winmodeselect::Mode2
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Winmodeselect::Mode3
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
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
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode1)
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode2)
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode3)
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode4)
    }
}
#[doc = "Field `WINSS` reader - Window Single Sample"]
pub type WinssR = crate::BitReader;
#[doc = "Field `WINSS` writer - Window Single Sample"]
pub type WinssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LeftadjR {
        LeftadjR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CorrenR {
        CorrenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> ResselR {
        ResselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    pub fn winss(&self) -> WinssR {
        WinssR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Left-Adjusted Result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LeftadjW<CtrlbSpec> {
        LeftadjW::new(self, 0)
    }
    #[doc = "Bit 1 - Free Running Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FreerunW<CtrlbSpec> {
        FreerunW::new(self, 1)
    }
    #[doc = "Bit 2 - Digital Correction Logic Enable"]
    #[inline(always)]
    #[must_use]
    pub fn corren(&mut self) -> CorrenW<CtrlbSpec> {
        CorrenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Conversion Result Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> ResselW<CtrlbSpec> {
        ResselW::new(self, 3)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WinmodeW<CtrlbSpec> {
        WinmodeW::new(self, 8)
    }
    #[doc = "Bit 11 - Window Single Sample"]
    #[inline(always)]
    #[must_use]
    pub fn winss(&mut self) -> WinssW<CtrlbSpec> {
        WinssW::new(self, 11)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u16 = 0;
}
