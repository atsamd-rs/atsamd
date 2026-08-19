#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: USART with external clock"]
    UsartExtClk = 0,
    #[doc = "1: USART with internal clock"]
    UsartIntClk = 1,
    #[doc = "2: SPI in slave operation"]
    SpiSlave = 2,
    #[doc = "3: SPI in master operation"]
    SpiMaster = 3,
    #[doc = "4: I2C slave operation"]
    I2cSlave = 4,
    #[doc = "5: I2C master operation"]
    I2cMaster = 5,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::UsartExtClk),
            1 => Some(Modeselect::UsartIntClk),
            2 => Some(Modeselect::SpiSlave),
            3 => Some(Modeselect::SpiMaster),
            4 => Some(Modeselect::I2cSlave),
            5 => Some(Modeselect::I2cMaster),
            _ => None,
        }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == Modeselect::UsartExtClk
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == Modeselect::UsartIntClk
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == Modeselect::SpiSlave
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == Modeselect::SpiMaster
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == Modeselect::I2cSlave
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == Modeselect::I2cMaster
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartExtClk)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartIntClk)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiSlave)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiMaster)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cSlave)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cMaster)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IbonR = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IbonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Out Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Doposelect {
    #[doc = "0: DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    Pad0 = 0,
    #[doc = "2: DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    Pad2 = 2,
}
impl From<Doposelect> for u8 {
    #[inline(always)]
    fn from(variant: Doposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Doposelect {
    type Ux = u8;
}
impl crate::IsEnum for Doposelect {}
#[doc = "Field `DOPO` reader - Data Out Pinout"]
pub type DopoR = crate::FieldReader<Doposelect>;
impl DopoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Doposelect> {
        match self.bits {
            0 => Some(Doposelect::Pad0),
            2 => Some(Doposelect::Pad2),
            _ => None,
        }
    }
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Doposelect::Pad0
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Doposelect::Pad2
    }
}
#[doc = "Field `DOPO` writer - Data Out Pinout"]
pub type DopoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Doposelect>;
impl<'a, REG> DopoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DO on PAD\\[0\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Doposelect::Pad0)
    }
    #[doc = "DO on PAD\\[3\\], SCK on PAD\\[1\\]
and SS on PAD\\[2\\]"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Doposelect::Pad2)
    }
}
#[doc = "Data In Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diposelect {
    #[doc = "0: SERCOM PAD\\[0\\]
is used as data input"]
    Pad0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]
is used as data input"]
    Pad1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]
is used as data input"]
    Pad2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]
is used as data input"]
    Pad3 = 3,
}
impl From<Diposelect> for u8 {
    #[inline(always)]
    fn from(variant: Diposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diposelect {
    type Ux = u8;
}
impl crate::IsEnum for Diposelect {}
#[doc = "Field `DIPO` reader - Data In Pinout"]
pub type DipoR = crate::FieldReader<Diposelect>;
impl DipoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diposelect {
        match self.bits {
            0 => Diposelect::Pad0,
            1 => Diposelect::Pad1,
            2 => Diposelect::Pad2,
            3 => Diposelect::Pad3,
            _ => unreachable!(),
        }
    }
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Diposelect::Pad0
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Diposelect::Pad1
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Diposelect::Pad2
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Diposelect::Pad3
    }
}
#[doc = "Field `DIPO` writer - Data In Pinout"]
pub type DipoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diposelect, crate::Safe>;
impl<'a, REG> DipoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SERCOM PAD\\[0\\]
is used as data input"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Diposelect::Pad0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used as data input"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Diposelect::Pad1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used as data input"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Diposelect::Pad2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used as data input"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Diposelect::Pad3)
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Formselect {
    #[doc = "0: SPI Frame"]
    SpiFrame = 0,
    #[doc = "2: SPI Frame with Addr"]
    SpiFrameWithAddr = 2,
}
impl From<Formselect> for u8 {
    #[inline(always)]
    fn from(variant: Formselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Formselect {
    type Ux = u8;
}
impl crate::IsEnum for Formselect {}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FormR = crate::FieldReader<Formselect>;
impl FormR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Formselect> {
        match self.bits {
            0 => Some(Formselect::SpiFrame),
            2 => Some(Formselect::SpiFrameWithAddr),
            _ => None,
        }
    }
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn is_spi_frame(&self) -> bool {
        *self == Formselect::SpiFrame
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn is_spi_frame_with_addr(&self) -> bool {
        *self == Formselect::SpiFrameWithAddr
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FormW<'a, REG> = crate::FieldWriter<'a, REG, 4, Formselect>;
impl<'a, REG> FormW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Frame"]
    #[inline(always)]
    pub fn spi_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::SpiFrame)
    }
    #[doc = "SPI Frame with Addr"]
    #[inline(always)]
    pub fn spi_frame_with_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::SpiFrameWithAddr)
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cphaselect {
    #[doc = "0: The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    LeadingEdge = 0,
    #[doc = "1: The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    TrailingEdge = 1,
}
impl From<Cphaselect> for bool {
    #[inline(always)]
    fn from(variant: Cphaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CphaR = crate::BitReader<Cphaselect>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cphaselect {
        match self.bits {
            false => Cphaselect::LeadingEdge,
            true => Cphaselect::TrailingEdge,
        }
    }
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn is_leading_edge(&self) -> bool {
        *self == Cphaselect::LeadingEdge
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn is_trailing_edge(&self) -> bool {
        *self == Cphaselect::TrailingEdge
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cphaselect>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is sampled on a leading SCK edge and changed on a trailing SCK edge"]
    #[inline(always)]
    pub fn leading_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cphaselect::LeadingEdge)
    }
    #[doc = "The data is sampled on a trailing SCK edge and changed on a leading SCK edge"]
    #[inline(always)]
    pub fn trailing_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Cphaselect::TrailingEdge)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpolselect {
    #[doc = "0: SCK is low when idle"]
    IdleLow = 0,
    #[doc = "1: SCK is high when idle"]
    IdleHigh = 1,
}
impl From<Cpolselect> for bool {
    #[inline(always)]
    fn from(variant: Cpolselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader<Cpolselect>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpolselect {
        match self.bits {
            false => Cpolselect::IdleLow,
            true => Cpolselect::IdleHigh,
        }
    }
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == Cpolselect::IdleLow
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == Cpolselect::IdleHigh
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpolselect>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK is low when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleLow)
    }
    #[doc = "SCK is high when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleHigh)
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dordselect {
    #[doc = "0: MSB is transferred first"]
    Msb = 0,
    #[doc = "1: LSB is transferred first"]
    Lsb = 1,
}
impl From<Dordselect> for bool {
    #[inline(always)]
    fn from(variant: Dordselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DordR = crate::BitReader<Dordselect>;
impl DordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dordselect {
        match self.bits {
            false => Dordselect::Msb,
            true => Dordselect::Lsb,
        }
    }
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Dordselect::Msb
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Dordselect::Lsb
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DordW<'a, REG> = crate::BitWriter<'a, REG, Dordselect>;
impl<'a, REG> DordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB is transferred first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Msb)
    }
    #[doc = "LSB is transferred first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Lsb)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IbonR {
        IbonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    pub fn dopo(&self) -> DopoR {
        DopoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    pub fn dipo(&self) -> DipoR {
        DipoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FormR {
        FormR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DordR {
        DordR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IbonW<CtrlaSpec> {
        IbonW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Data Out Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dopo(&mut self) -> DopoW<CtrlaSpec> {
        DopoW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Data In Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn dipo(&mut self) -> DipoW<CtrlaSpec> {
        DipoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FormW<CtrlaSpec> {
        FormW::new(self, 24)
    }
    #[doc = "Bit 28 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<CtrlaSpec> {
        CphaW::new(self, 28)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<CtrlaSpec> {
        CpolW::new(self, 29)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DordW<CtrlaSpec> {
        DordW::new(self, 30)
    }
}
#[doc = "SPIM Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0;
}
