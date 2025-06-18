#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<InputctrlSpec>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<InputctrlSpec>;
#[doc = "Positive Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxposselect {
    #[doc = "0: ADC AIN0 Pin"]
    Ain0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    Ain1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    Ain2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Ain3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Ain4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Ain5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Ain6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Ain7 = 7,
    #[doc = "8: ADC AIN8 Pin"]
    Ain8 = 8,
    #[doc = "9: ADC AIN9 Pin"]
    Ain9 = 9,
    #[doc = "10: ADC AIN10 Pin"]
    Ain10 = 10,
    #[doc = "11: ADC AIN11 Pin"]
    Ain11 = 11,
    #[doc = "12: ADC AIN12 Pin"]
    Ain12 = 12,
    #[doc = "13: ADC AIN13 Pin"]
    Ain13 = 13,
    #[doc = "14: ADC AIN14 Pin"]
    Ain14 = 14,
    #[doc = "15: ADC AIN15 Pin"]
    Ain15 = 15,
    #[doc = "24: 1/4 Scaled Core Supply"]
    Scaledcorevcc = 24,
    #[doc = "25: 1/4 Scaled VBAT Supply"]
    Scaledvbat = 25,
    #[doc = "26: 1/4 Scaled I/O Supply"]
    Scalediovcc = 26,
    #[doc = "27: Bandgap Voltage"]
    Bandgap = 27,
    #[doc = "28: Temperature Sensor TSENSP"]
    Ptat = 28,
    #[doc = "29: Temperature Sensor TSENSC"]
    Ctat = 29,
    #[doc = "30: DAC Output"]
    Dac = 30,
    #[doc = "31: PTC output (only on ADC0)"]
    Ptc = 31,
}
impl From<Muxposselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxposselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxposselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxposselect {}
#[doc = "Field `MUXPOS` reader - Positive Mux Input Selection"]
pub type MuxposR = crate::FieldReader<Muxposselect>;
impl MuxposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxposselect> {
        match self.bits {
            0 => Some(Muxposselect::Ain0),
            1 => Some(Muxposselect::Ain1),
            2 => Some(Muxposselect::Ain2),
            3 => Some(Muxposselect::Ain3),
            4 => Some(Muxposselect::Ain4),
            5 => Some(Muxposselect::Ain5),
            6 => Some(Muxposselect::Ain6),
            7 => Some(Muxposselect::Ain7),
            8 => Some(Muxposselect::Ain8),
            9 => Some(Muxposselect::Ain9),
            10 => Some(Muxposselect::Ain10),
            11 => Some(Muxposselect::Ain11),
            12 => Some(Muxposselect::Ain12),
            13 => Some(Muxposselect::Ain13),
            14 => Some(Muxposselect::Ain14),
            15 => Some(Muxposselect::Ain15),
            24 => Some(Muxposselect::Scaledcorevcc),
            25 => Some(Muxposselect::Scaledvbat),
            26 => Some(Muxposselect::Scalediovcc),
            27 => Some(Muxposselect::Bandgap),
            28 => Some(Muxposselect::Ptat),
            29 => Some(Muxposselect::Ctat),
            30 => Some(Muxposselect::Dac),
            31 => Some(Muxposselect::Ptc),
            _ => None,
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == Muxposselect::Ain0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == Muxposselect::Ain1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == Muxposselect::Ain2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == Muxposselect::Ain3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == Muxposselect::Ain4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == Muxposselect::Ain5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == Muxposselect::Ain6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == Muxposselect::Ain7
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == Muxposselect::Ain8
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == Muxposselect::Ain9
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == Muxposselect::Ain10
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == Muxposselect::Ain11
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn is_ain12(&self) -> bool {
        *self == Muxposselect::Ain12
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn is_ain13(&self) -> bool {
        *self == Muxposselect::Ain13
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn is_ain14(&self) -> bool {
        *self == Muxposselect::Ain14
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn is_ain15(&self) -> bool {
        *self == Muxposselect::Ain15
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn is_scaledcorevcc(&self) -> bool {
        *self == Muxposselect::Scaledcorevcc
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn is_scaledvbat(&self) -> bool {
        *self == Muxposselect::Scaledvbat
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn is_scalediovcc(&self) -> bool {
        *self == Muxposselect::Scalediovcc
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == Muxposselect::Bandgap
    }
    #[doc = "Temperature Sensor TSENSP"]
    #[inline(always)]
    pub fn is_ptat(&self) -> bool {
        *self == Muxposselect::Ptat
    }
    #[doc = "Temperature Sensor TSENSC"]
    #[inline(always)]
    pub fn is_ctat(&self) -> bool {
        *self == Muxposselect::Ctat
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Muxposselect::Dac
    }
    #[doc = "PTC output (only on ADC0)"]
    #[inline(always)]
    pub fn is_ptc(&self) -> bool {
        *self == Muxposselect::Ptc
    }
}
#[doc = "Field `MUXPOS` writer - Positive Mux Input Selection"]
pub type MuxposW<'a, REG> = crate::FieldWriter<'a, REG, 5, Muxposselect>;
impl<'a, REG> MuxposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain7)
    }
    #[doc = "ADC AIN8 Pin"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain8)
    }
    #[doc = "ADC AIN9 Pin"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain9)
    }
    #[doc = "ADC AIN10 Pin"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain10)
    }
    #[doc = "ADC AIN11 Pin"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain11)
    }
    #[doc = "ADC AIN12 Pin"]
    #[inline(always)]
    pub fn ain12(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain12)
    }
    #[doc = "ADC AIN13 Pin"]
    #[inline(always)]
    pub fn ain13(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain13)
    }
    #[doc = "ADC AIN14 Pin"]
    #[inline(always)]
    pub fn ain14(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain14)
    }
    #[doc = "ADC AIN15 Pin"]
    #[inline(always)]
    pub fn ain15(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ain15)
    }
    #[doc = "1/4 Scaled Core Supply"]
    #[inline(always)]
    pub fn scaledcorevcc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Scaledcorevcc)
    }
    #[doc = "1/4 Scaled VBAT Supply"]
    #[inline(always)]
    pub fn scaledvbat(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Scaledvbat)
    }
    #[doc = "1/4 Scaled I/O Supply"]
    #[inline(always)]
    pub fn scalediovcc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Scalediovcc)
    }
    #[doc = "Bandgap Voltage"]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Bandgap)
    }
    #[doc = "Temperature Sensor TSENSP"]
    #[inline(always)]
    pub fn ptat(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ptat)
    }
    #[doc = "Temperature Sensor TSENSC"]
    #[inline(always)]
    pub fn ctat(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ctat)
    }
    #[doc = "DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Dac)
    }
    #[doc = "PTC output (only on ADC0)"]
    #[inline(always)]
    pub fn ptc(self) -> &'a mut crate::W<REG> {
        self.variant(Muxposselect::Ptc)
    }
}
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DiffmodeR = crate::BitReader;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DiffmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Negative Mux Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxnegselect {
    #[doc = "0: ADC AIN0 Pin"]
    Ain0 = 0,
    #[doc = "1: ADC AIN1 Pin"]
    Ain1 = 1,
    #[doc = "2: ADC AIN2 Pin"]
    Ain2 = 2,
    #[doc = "3: ADC AIN3 Pin"]
    Ain3 = 3,
    #[doc = "4: ADC AIN4 Pin"]
    Ain4 = 4,
    #[doc = "5: ADC AIN5 Pin"]
    Ain5 = 5,
    #[doc = "6: ADC AIN6 Pin"]
    Ain6 = 6,
    #[doc = "7: ADC AIN7 Pin"]
    Ain7 = 7,
    #[doc = "24: Internal Ground"]
    Gnd = 24,
}
impl From<Muxnegselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxnegselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxnegselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxnegselect {}
#[doc = "Field `MUXNEG` reader - Negative Mux Input Selection"]
pub type MuxnegR = crate::FieldReader<Muxnegselect>;
impl MuxnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxnegselect> {
        match self.bits {
            0 => Some(Muxnegselect::Ain0),
            1 => Some(Muxnegselect::Ain1),
            2 => Some(Muxnegselect::Ain2),
            3 => Some(Muxnegselect::Ain3),
            4 => Some(Muxnegselect::Ain4),
            5 => Some(Muxnegselect::Ain5),
            6 => Some(Muxnegselect::Ain6),
            7 => Some(Muxnegselect::Ain7),
            24 => Some(Muxnegselect::Gnd),
            _ => None,
        }
    }
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == Muxnegselect::Ain0
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == Muxnegselect::Ain1
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == Muxnegselect::Ain2
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == Muxnegselect::Ain3
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == Muxnegselect::Ain4
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == Muxnegselect::Ain5
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == Muxnegselect::Ain6
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == Muxnegselect::Ain7
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Muxnegselect::Gnd
    }
}
#[doc = "Field `MUXNEG` writer - Negative Mux Input Selection"]
pub type MuxnegW<'a, REG> = crate::FieldWriter<'a, REG, 5, Muxnegselect>;
impl<'a, REG> MuxnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain0)
    }
    #[doc = "ADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain1)
    }
    #[doc = "ADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain2)
    }
    #[doc = "ADC AIN3 Pin"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain3)
    }
    #[doc = "ADC AIN4 Pin"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain4)
    }
    #[doc = "ADC AIN5 Pin"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain5)
    }
    #[doc = "ADC AIN6 Pin"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain6)
    }
    #[doc = "ADC AIN7 Pin"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Ain7)
    }
    #[doc = "Internal Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Muxnegselect::Gnd)
    }
}
#[doc = "Field `DSEQSTOP` reader - Stop DMA Sequencing"]
pub type DseqstopR = crate::BitReader;
#[doc = "Field `DSEQSTOP` writer - Stop DMA Sequencing"]
pub type DseqstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MuxposR {
        MuxposR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DiffmodeR {
        DiffmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MuxnegR {
        MuxnegR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline(always)]
    pub fn dseqstop(&self) -> DseqstopR {
        DseqstopR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Positive Mux Input Selection"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MuxposW<InputctrlSpec> {
        MuxposW::new(self, 0)
    }
    #[doc = "Bit 7 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&mut self) -> DiffmodeW<InputctrlSpec> {
        DiffmodeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Negative Mux Input Selection"]
    #[inline(always)]
    pub fn muxneg(&mut self) -> MuxnegW<InputctrlSpec> {
        MuxnegW::new(self, 8)
    }
    #[doc = "Bit 15 - Stop DMA Sequencing"]
    #[inline(always)]
    pub fn dseqstop(&mut self) -> DseqstopW<InputctrlSpec> {
        DseqstopW::new(self, 15)
    }
}
#[doc = "Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputctrlSpec;
impl crate::RegisterSpec for InputctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for InputctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for InputctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for InputctrlSpec {}
