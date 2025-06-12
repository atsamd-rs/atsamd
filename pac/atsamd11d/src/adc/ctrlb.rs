#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DiffmodeR = crate::BitReader;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DiffmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LeftadjR = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LeftadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FreerunR = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enabled"]
pub type CorrenR = crate::BitReader;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enabled"]
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
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Peripheral clock divided by 4"]
    Div4 = 0,
    #[doc = "1: Peripheral clock divided by 8"]
    Div8 = 1,
    #[doc = "2: Peripheral clock divided by 16"]
    Div16 = 2,
    #[doc = "3: Peripheral clock divided by 32"]
    Div32 = 3,
    #[doc = "4: Peripheral clock divided by 64"]
    Div64 = 4,
    #[doc = "5: Peripheral clock divided by 128"]
    Div128 = 5,
    #[doc = "6: Peripheral clock divided by 256"]
    Div256 = 6,
    #[doc = "7: Peripheral clock divided by 512"]
    Div512 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler Configuration"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescalerselect {
        match self.bits {
            0 => Prescalerselect::Div4,
            1 => Prescalerselect::Div8,
            2 => Prescalerselect::Div16,
            3 => Prescalerselect::Div32,
            4 => Prescalerselect::Div64,
            5 => Prescalerselect::Div128,
            6 => Prescalerselect::Div256,
            7 => Prescalerselect::Div512,
            _ => unreachable!(),
        }
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescalerselect::Div128
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
    #[doc = "Peripheral clock divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Prescalerselect::Div512
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Configuration"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescalerselect, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
    #[doc = "Peripheral clock divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div512)
    }
}
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DiffmodeR {
        DiffmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LeftadjR {
        LeftadjR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline(always)]
    pub fn corren(&self) -> CorrenR {
        CorrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> ResselR {
        ResselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&mut self) -> DiffmodeW<CtrlbSpec> {
        DiffmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&mut self) -> LeftadjW<CtrlbSpec> {
        LeftadjW::new(self, 1)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FreerunW<CtrlbSpec> {
        FreerunW::new(self, 2)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enabled"]
    #[inline(always)]
    pub fn corren(&mut self) -> CorrenW<CtrlbSpec> {
        CorrenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&mut self) -> ResselW<CtrlbSpec> {
        ResselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlbSpec> {
        PrescalerW::new(self, 8)
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
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
