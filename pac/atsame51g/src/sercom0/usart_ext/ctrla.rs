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
#[doc = "Field `TXINV` reader - Transmit Data Invert"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - Transmit Data Invert"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - Receive Data Invert"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - Receive Data Invert"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samprselect {
    #[doc = "0: 16x over-sampling using arithmetic baudrate generation"]
    _16xArithmetic = 0,
    #[doc = "1: 16x over-sampling using fractional baudrate generation"]
    _16xFractional = 1,
    #[doc = "2: 8x over-sampling using arithmetic baudrate generation"]
    _8xArithmetic = 2,
    #[doc = "3: 8x over-sampling using fractional baudrate generation"]
    _8xFractional = 3,
    #[doc = "4: 3x over-sampling using arithmetic baudrate generation"]
    _3xArithmetic = 4,
}
impl From<Samprselect> for u8 {
    #[inline(always)]
    fn from(variant: Samprselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samprselect {
    type Ux = u8;
}
impl crate::IsEnum for Samprselect {}
#[doc = "Field `SAMPR` reader - Sample"]
pub type SamprR = crate::FieldReader<Samprselect>;
impl SamprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Samprselect> {
        match self.bits {
            0 => Some(Samprselect::_16xArithmetic),
            1 => Some(Samprselect::_16xFractional),
            2 => Some(Samprselect::_8xArithmetic),
            3 => Some(Samprselect::_8xFractional),
            4 => Some(Samprselect::_3xArithmetic),
            _ => None,
        }
    }
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn is_16x_arithmetic(&self) -> bool {
        *self == Samprselect::_16xArithmetic
    }
    #[doc = "16x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn is_16x_fractional(&self) -> bool {
        *self == Samprselect::_16xFractional
    }
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn is_8x_arithmetic(&self) -> bool {
        *self == Samprselect::_8xArithmetic
    }
    #[doc = "8x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn is_8x_fractional(&self) -> bool {
        *self == Samprselect::_8xFractional
    }
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn is_3x_arithmetic(&self) -> bool {
        *self == Samprselect::_3xArithmetic
    }
}
#[doc = "Field `SAMPR` writer - Sample"]
pub type SamprW<'a, REG> = crate::FieldWriter<'a, REG, 3, Samprselect>;
impl<'a, REG> SamprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _16x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_16xArithmetic)
    }
    #[doc = "16x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _16x_fractional(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_16xFractional)
    }
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _8x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_8xArithmetic)
    }
    #[doc = "8x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _8x_fractional(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_8xFractional)
    }
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _3x_arithmetic(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::_3xArithmetic)
    }
}
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txposelect {
    #[doc = "0: TxD on PAD0, XCK on PAD1"]
    Txpo0 = 0,
    #[doc = "2: TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    Txpo2 = 2,
    #[doc = "3: TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    Txpo3 = 3,
}
impl From<Txposelect> for u8 {
    #[inline(always)]
    fn from(variant: Txposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txposelect {
    type Ux = u8;
}
impl crate::IsEnum for Txposelect {}
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TxpoR = crate::FieldReader<Txposelect>;
impl TxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txposelect> {
        match self.bits {
            0 => Some(Txposelect::Txpo0),
            2 => Some(Txposelect::Txpo2),
            3 => Some(Txposelect::Txpo3),
            _ => None,
        }
    }
    #[doc = "TxD on PAD0, XCK on PAD1"]
    #[inline(always)]
    pub fn is_txpo_0(&self) -> bool {
        *self == Txposelect::Txpo0
    }
    #[doc = "TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    #[inline(always)]
    pub fn is_txpo_2(&self) -> bool {
        *self == Txposelect::Txpo2
    }
    #[doc = "TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    #[inline(always)]
    pub fn is_txpo_3(&self) -> bool {
        *self == Txposelect::Txpo3
    }
}
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txposelect>;
impl<'a, REG> TxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TxD on PAD0, XCK on PAD1"]
    #[inline(always)]
    pub fn txpo_0(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Txpo0)
    }
    #[doc = "TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    #[inline(always)]
    pub fn txpo_2(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Txpo2)
    }
    #[doc = "TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    #[inline(always)]
    pub fn txpo_3(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Txpo3)
    }
}
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxposelect {
    #[doc = "0: SERCOM PAD\\[0\\]
is used for data reception"]
    Pad0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]
is used for data reception"]
    Pad1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]
is used for data reception"]
    Pad2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]
is used for data reception"]
    Pad3 = 3,
}
impl From<Rxposelect> for u8 {
    #[inline(always)]
    fn from(variant: Rxposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxposelect {
    type Ux = u8;
}
impl crate::IsEnum for Rxposelect {}
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RxpoR = crate::FieldReader<Rxposelect>;
impl RxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxposelect {
        match self.bits {
            0 => Rxposelect::Pad0,
            1 => Rxposelect::Pad1,
            2 => Rxposelect::Pad2,
            3 => Rxposelect::Pad3,
            _ => unreachable!(),
        }
    }
    #[doc = "SERCOM PAD\\[0\\]
is used for data reception"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Rxposelect::Pad0
    }
    #[doc = "SERCOM PAD\\[1\\]
is used for data reception"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Rxposelect::Pad1
    }
    #[doc = "SERCOM PAD\\[2\\]
is used for data reception"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Rxposelect::Pad2
    }
    #[doc = "SERCOM PAD\\[3\\]
is used for data reception"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Rxposelect::Pad3
    }
}
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxposelect, crate::Safe>;
impl<'a, REG> RxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SERCOM PAD\\[0\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad3)
    }
}
#[doc = "Sample Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampaselect {
    #[doc = "0: 16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    Adj0 = 0,
    #[doc = "1: 16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    Adj1 = 1,
    #[doc = "2: 16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    Adj2 = 2,
    #[doc = "3: 16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    Adj3 = 3,
}
impl From<Sampaselect> for u8 {
    #[inline(always)]
    fn from(variant: Sampaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampaselect {
    type Ux = u8;
}
impl crate::IsEnum for Sampaselect {}
#[doc = "Field `SAMPA` reader - Sample Adjustment"]
pub type SampaR = crate::FieldReader<Sampaselect>;
impl SampaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampaselect {
        match self.bits {
            0 => Sampaselect::Adj0,
            1 => Sampaselect::Adj1,
            2 => Sampaselect::Adj2,
            3 => Sampaselect::Adj3,
            _ => unreachable!(),
        }
    }
    #[doc = "16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    #[inline(always)]
    pub fn is_adj0(&self) -> bool {
        *self == Sampaselect::Adj0
    }
    #[doc = "16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    #[inline(always)]
    pub fn is_adj1(&self) -> bool {
        *self == Sampaselect::Adj1
    }
    #[doc = "16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    #[inline(always)]
    pub fn is_adj2(&self) -> bool {
        *self == Sampaselect::Adj2
    }
    #[doc = "16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    #[inline(always)]
    pub fn is_adj3(&self) -> bool {
        *self == Sampaselect::Adj3
    }
}
#[doc = "Field `SAMPA` writer - Sample Adjustment"]
pub type SampaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sampaselect, crate::Safe>;
impl<'a, REG> SampaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x Over-sampling = 7-8-9; 8x Over-sampling = 3-4-5"]
    #[inline(always)]
    pub fn adj0(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj0)
    }
    #[doc = "16x Over-sampling = 9-10-11; 8x Over-sampling = 4-5-6"]
    #[inline(always)]
    pub fn adj1(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj1)
    }
    #[doc = "16x Over-sampling = 11-12-13; 8x Over-sampling = 5-6-7"]
    #[inline(always)]
    pub fn adj2(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj2)
    }
    #[doc = "16x Over-sampling = 13-14-15; 8x Over-sampling = 6-7-8"]
    #[inline(always)]
    pub fn adj3(self) -> &'a mut crate::W<REG> {
        self.variant(Sampaselect::Adj3)
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Formselect {
    #[doc = "0: USART frame"]
    UsartFrameNoParity = 0,
    #[doc = "1: USART frame with parity"]
    UsartFrameWithParity = 1,
    #[doc = "2: LIN Master - Break and sync generation"]
    UsartFrameLinMasterMode = 2,
    #[doc = "4: Auto-baud (LIN Slave) - break detection and auto-baud"]
    UsartFrameAutoBaudNoParity = 4,
    #[doc = "5: Auto-baud - break detection and auto-baud with parity"]
    UsartFrameAutoBaudWithParity = 5,
    #[doc = "7: ISO 7816"]
    UsartFrameIso7816 = 7,
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
            0 => Some(Formselect::UsartFrameNoParity),
            1 => Some(Formselect::UsartFrameWithParity),
            2 => Some(Formselect::UsartFrameLinMasterMode),
            4 => Some(Formselect::UsartFrameAutoBaudNoParity),
            5 => Some(Formselect::UsartFrameAutoBaudWithParity),
            7 => Some(Formselect::UsartFrameIso7816),
            _ => None,
        }
    }
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn is_usart_frame_no_parity(&self) -> bool {
        *self == Formselect::UsartFrameNoParity
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn is_usart_frame_with_parity(&self) -> bool {
        *self == Formselect::UsartFrameWithParity
    }
    #[doc = "LIN Master - Break and sync generation"]
    #[inline(always)]
    pub fn is_usart_frame_lin_master_mode(&self) -> bool {
        *self == Formselect::UsartFrameLinMasterMode
    }
    #[doc = "Auto-baud (LIN Slave) - break detection and auto-baud"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_no_parity(&self) -> bool {
        *self == Formselect::UsartFrameAutoBaudNoParity
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_with_parity(&self) -> bool {
        *self == Formselect::UsartFrameAutoBaudWithParity
    }
    #[doc = "ISO 7816"]
    #[inline(always)]
    pub fn is_usart_frame_iso_7816(&self) -> bool {
        *self == Formselect::UsartFrameIso7816
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FormW<'a, REG> = crate::FieldWriter<'a, REG, 4, Formselect>;
impl<'a, REG> FormW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn usart_frame_no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameNoParity)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn usart_frame_with_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameWithParity)
    }
    #[doc = "LIN Master - Break and sync generation"]
    #[inline(always)]
    pub fn usart_frame_lin_master_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameLinMasterMode)
    }
    #[doc = "Auto-baud (LIN Slave) - break detection and auto-baud"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_no_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameAutoBaudNoParity)
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_with_parity(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameAutoBaudWithParity)
    }
    #[doc = "ISO 7816"]
    #[inline(always)]
    pub fn usart_frame_iso_7816(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::UsartFrameIso7816)
    }
}
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmodeselect {
    #[doc = "0: Asynchronous Communication"]
    Async = 0,
    #[doc = "1: Synchronous Communication"]
    Sync = 1,
}
impl From<Cmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Cmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CmodeR = crate::BitReader<Cmodeselect>;
impl CmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmodeselect {
        match self.bits {
            false => Cmodeselect::Async,
            true => Cmodeselect::Sync,
        }
    }
    #[doc = "Asynchronous Communication"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Cmodeselect::Async
    }
    #[doc = "Synchronous Communication"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Cmodeselect::Sync
    }
}
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmodeselect>;
impl<'a, REG> CmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous Communication"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Async)
    }
    #[doc = "Synchronous Communication"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Sync)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpolselect {
    #[doc = "0: TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    IdleLow = 0,
    #[doc = "1: TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
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
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == Cpolselect::IdleLow
    }
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
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
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleLow)
    }
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpolselect::IdleHigh)
    }
}
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dordselect {
    #[doc = "0: MSB is transmitted first"]
    Msb = 0,
    #[doc = "1: LSB is transmitted first"]
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
    #[doc = "MSB is transmitted first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Dordselect::Msb
    }
    #[doc = "LSB is transmitted first"]
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
    #[doc = "MSB is transmitted first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Msb)
    }
    #[doc = "LSB is transmitted first"]
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
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&self) -> SamprR {
        SamprR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TxpoR {
        TxpoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RxpoR {
        RxpoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SampaR {
        SampaR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FormR {
        FormR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new(((self.bits >> 28) & 1) != 0)
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
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<CtrlaSpec> {
        TxinvW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<CtrlaSpec> {
        RxinvW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    #[must_use]
    pub fn sampr(&mut self) -> SamprW<CtrlaSpec> {
        SamprW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn txpo(&mut self) -> TxpoW<CtrlaSpec> {
        TxpoW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn rxpo(&mut self) -> RxpoW<CtrlaSpec> {
        RxpoW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn sampa(&mut self) -> SampaW<CtrlaSpec> {
        SampaW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FormW<CtrlaSpec> {
        FormW::new(self, 24)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmode(&mut self) -> CmodeW<CtrlaSpec> {
        CmodeW::new(self, 28)
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
#[doc = "USART_EXT Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
