#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: USART with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI in slave operation"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI in master operation"]
    SPI_MASTER = 3,
    #[doc = "4: I2C slave operation"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C master operation"]
    I2C_MASTER = 5,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::USART_EXT_CLK),
            1 => Some(MODESELECT_A::USART_INT_CLK),
            2 => Some(MODESELECT_A::SPI_SLAVE),
            3 => Some(MODESELECT_A::SPI_MASTER),
            4 => Some(MODESELECT_A::I2C_SLAVE),
            5 => Some(MODESELECT_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, MODESELECT_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader<bool>;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - Transmit Data Invert"]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - Transmit Data Invert"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `RXINV` reader - Receive Data Invert"]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - Receive Data Invert"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `SAMPR` reader - Sample"]
pub type SAMPR_R = crate::FieldReader<u8, SAMPRSELECT_A>;
#[doc = "Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPRSELECT_A {
    #[doc = "0: 16x over-sampling using arithmetic baudrate generation"]
    _16X_ARITHMETIC = 0,
    #[doc = "1: 16x over-sampling using fractional baudrate generation"]
    _16X_FRACTIONAL = 1,
    #[doc = "2: 8x over-sampling using arithmetic baudrate generation"]
    _8X_ARITHMETIC = 2,
    #[doc = "3: 8x over-sampling using fractional baudrate generation"]
    _8X_FRACTIONAL = 3,
    #[doc = "4: 3x over-sampling using arithmetic baudrate generation"]
    _3X_ARITHMETIC = 4,
}
impl From<SAMPRSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPRSELECT_A) -> Self {
        variant as _
    }
}
impl SAMPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAMPRSELECT_A> {
        match self.bits {
            0 => Some(SAMPRSELECT_A::_16X_ARITHMETIC),
            1 => Some(SAMPRSELECT_A::_16X_FRACTIONAL),
            2 => Some(SAMPRSELECT_A::_8X_ARITHMETIC),
            3 => Some(SAMPRSELECT_A::_8X_FRACTIONAL),
            4 => Some(SAMPRSELECT_A::_3X_ARITHMETIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_16x_arithmetic(&self) -> bool {
        *self == SAMPRSELECT_A::_16X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_16X_FRACTIONAL`"]
    #[inline(always)]
    pub fn is_16x_fractional(&self) -> bool {
        *self == SAMPRSELECT_A::_16X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_8X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_8x_arithmetic(&self) -> bool {
        *self == SAMPRSELECT_A::_8X_ARITHMETIC
    }
    #[doc = "Checks if the value of the field is `_8X_FRACTIONAL`"]
    #[inline(always)]
    pub fn is_8x_fractional(&self) -> bool {
        *self == SAMPRSELECT_A::_8X_FRACTIONAL
    }
    #[doc = "Checks if the value of the field is `_3X_ARITHMETIC`"]
    #[inline(always)]
    pub fn is_3x_arithmetic(&self) -> bool {
        *self == SAMPRSELECT_A::_3X_ARITHMETIC
    }
}
#[doc = "Field `SAMPR` writer - Sample"]
pub type SAMPR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, SAMPRSELECT_A, 3, O>;
impl<'a, const O: u8> SAMPR_W<'a, O> {
    #[doc = "16x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _16x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRSELECT_A::_16X_ARITHMETIC)
    }
    #[doc = "16x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _16x_fractional(self) -> &'a mut W {
        self.variant(SAMPRSELECT_A::_16X_FRACTIONAL)
    }
    #[doc = "8x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _8x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRSELECT_A::_8X_ARITHMETIC)
    }
    #[doc = "8x over-sampling using fractional baudrate generation"]
    #[inline(always)]
    pub fn _8x_fractional(self) -> &'a mut W {
        self.variant(SAMPRSELECT_A::_8X_FRACTIONAL)
    }
    #[doc = "3x over-sampling using arithmetic baudrate generation"]
    #[inline(always)]
    pub fn _3x_arithmetic(self) -> &'a mut W {
        self.variant(SAMPRSELECT_A::_3X_ARITHMETIC)
    }
}
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TXPO_R = crate::FieldReader<u8, TXPOSELECT_A>;
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXPOSELECT_A {
    #[doc = "0: TxD on PAD0, XCK on PAD1"]
    TXPO_0 = 0,
    #[doc = "2: TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    TXPO_2 = 2,
    #[doc = "3: TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    TXPO_3 = 3,
}
impl From<TXPOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOSELECT_A) -> Self {
        variant as _
    }
}
impl TXPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXPOSELECT_A> {
        match self.bits {
            0 => Some(TXPOSELECT_A::TXPO_0),
            2 => Some(TXPOSELECT_A::TXPO_2),
            3 => Some(TXPOSELECT_A::TXPO_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TXPO_0`"]
    #[inline(always)]
    pub fn is_txpo_0(&self) -> bool {
        *self == TXPOSELECT_A::TXPO_0
    }
    #[doc = "Checks if the value of the field is `TXPO_2`"]
    #[inline(always)]
    pub fn is_txpo_2(&self) -> bool {
        *self == TXPOSELECT_A::TXPO_2
    }
    #[doc = "Checks if the value of the field is `TXPO_3`"]
    #[inline(always)]
    pub fn is_txpo_3(&self) -> bool {
        *self == TXPOSELECT_A::TXPO_3
    }
}
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TXPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, TXPOSELECT_A, 2, O>;
impl<'a, const O: u8> TXPO_W<'a, O> {
    #[doc = "TxD on PAD0, XCK on PAD1"]
    #[inline(always)]
    pub fn txpo_0(self) -> &'a mut W {
        self.variant(TXPOSELECT_A::TXPO_0)
    }
    #[doc = "TxD on PAD0, RTS/TE on PAD2, CTS on PAD3"]
    #[inline(always)]
    pub fn txpo_2(self) -> &'a mut W {
        self.variant(TXPOSELECT_A::TXPO_2)
    }
    #[doc = "TxD on PAD0, XCK on PAD1, RTS/TE on PAD2"]
    #[inline(always)]
    pub fn txpo_3(self) -> &'a mut W {
        self.variant(TXPOSELECT_A::TXPO_3)
    }
}
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RXPO_R = crate::FieldReader<u8, RXPOSELECT_A>;
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPOSELECT_A {
    #[doc = "0: SERCOM PAD\\[0\\]
is used for data reception"]
    PAD0 = 0,
    #[doc = "1: SERCOM PAD\\[1\\]
is used for data reception"]
    PAD1 = 1,
    #[doc = "2: SERCOM PAD\\[2\\]
is used for data reception"]
    PAD2 = 2,
    #[doc = "3: SERCOM PAD\\[3\\]
is used for data reception"]
    PAD3 = 3,
}
impl From<RXPOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPOSELECT_A) -> Self {
        variant as _
    }
}
impl RXPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPOSELECT_A {
        match self.bits {
            0 => RXPOSELECT_A::PAD0,
            1 => RXPOSELECT_A::PAD1,
            2 => RXPOSELECT_A::PAD2,
            3 => RXPOSELECT_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == RXPOSELECT_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == RXPOSELECT_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == RXPOSELECT_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == RXPOSELECT_A::PAD3
    }
}
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RXPO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRLA_SPEC, u8, RXPOSELECT_A, 2, O>;
impl<'a, const O: u8> RXPO_W<'a, O> {
    #[doc = "SERCOM PAD\\[0\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD0)
    }
    #[doc = "SERCOM PAD\\[1\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD1)
    }
    #[doc = "SERCOM PAD\\[2\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD2)
    }
    #[doc = "SERCOM PAD\\[3\\]
is used for data reception"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD3)
    }
}
#[doc = "Field `SAMPA` reader - Sample Adjustment"]
pub type SAMPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPA` writer - Sample Adjustment"]
pub type SAMPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<u8, FORMSELECT_A>;
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMSELECT_A {
    #[doc = "0: USART frame"]
    USART_FRAME_NO_PARITY = 0,
    #[doc = "1: USART frame with parity"]
    USART_FRAME_WITH_PARITY = 1,
    #[doc = "2: LIN Master - Break and sync generation"]
    USART_FRAME_LIN_MASTER_MODE = 2,
    #[doc = "4: Auto-baud - break detection and auto-baud"]
    USART_FRAME_AUTO_BAUD_NO_PARITY = 4,
    #[doc = "5: Auto-baud - break detection and auto-baud with parity"]
    USART_FRAME_AUTO_BAUD_WITH_PARITY = 5,
    #[doc = "7: ISO 7816"]
    USART_FRAME_ISO_7816 = 7,
}
impl From<FORMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMSELECT_A) -> Self {
        variant as _
    }
}
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMSELECT_A> {
        match self.bits {
            0 => Some(FORMSELECT_A::USART_FRAME_NO_PARITY),
            1 => Some(FORMSELECT_A::USART_FRAME_WITH_PARITY),
            2 => Some(FORMSELECT_A::USART_FRAME_LIN_MASTER_MODE),
            4 => Some(FORMSELECT_A::USART_FRAME_AUTO_BAUD_NO_PARITY),
            5 => Some(FORMSELECT_A::USART_FRAME_AUTO_BAUD_WITH_PARITY),
            7 => Some(FORMSELECT_A::USART_FRAME_ISO_7816),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_NO_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_no_parity(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_WITH_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_with_parity(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_LIN_MASTER_MODE`"]
    #[inline(always)]
    pub fn is_usart_frame_lin_master_mode(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_LIN_MASTER_MODE
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_NO_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_no_parity(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_AUTO_BAUD_NO_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_AUTO_BAUD_WITH_PARITY`"]
    #[inline(always)]
    pub fn is_usart_frame_auto_baud_with_parity(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_AUTO_BAUD_WITH_PARITY
    }
    #[doc = "Checks if the value of the field is `USART_FRAME_ISO_7816`"]
    #[inline(always)]
    pub fn is_usart_frame_iso_7816(&self) -> bool {
        *self == FORMSELECT_A::USART_FRAME_ISO_7816
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, FORMSELECT_A, 4, O>;
impl<'a, const O: u8> FORM_W<'a, O> {
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn usart_frame_no_parity(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_NO_PARITY)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn usart_frame_with_parity(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_WITH_PARITY)
    }
    #[doc = "LIN Master - Break and sync generation"]
    #[inline(always)]
    pub fn usart_frame_lin_master_mode(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_LIN_MASTER_MODE)
    }
    #[doc = "Auto-baud - break detection and auto-baud"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_no_parity(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_AUTO_BAUD_NO_PARITY)
    }
    #[doc = "Auto-baud - break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn usart_frame_auto_baud_with_parity(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_AUTO_BAUD_WITH_PARITY)
    }
    #[doc = "ISO 7816"]
    #[inline(always)]
    pub fn usart_frame_iso_7816(self) -> &'a mut W {
        self.variant(FORMSELECT_A::USART_FRAME_ISO_7816)
    }
}
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CMODE_R = crate::BitReader<CMODESELECT_A>;
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMODESELECT_A {
    #[doc = "0: Asynchronous Communication"]
    ASYNC = 0,
    #[doc = "1: Synchronous Communication"]
    SYNC = 1,
}
impl From<CMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMODESELECT_A {
        match self.bits {
            false => CMODESELECT_A::ASYNC,
            true => CMODESELECT_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == CMODESELECT_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == CMODESELECT_A::SYNC
    }
}
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, CMODESELECT_A, O>;
impl<'a, const O: u8> CMODE_W<'a, O> {
    #[doc = "Asynchronous Communication"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(CMODESELECT_A::ASYNC)
    }
    #[doc = "Synchronous Communication"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(CMODESELECT_A::SYNC)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    IDLE_LOW = 0,
    #[doc = "1: TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
    IDLE_HIGH = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::IDLE_LOW,
            true => CPOLSELECT_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, CPOLSELECT_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "TxD Change:- Rising XCK edge, RxD Sample:- Falling XCK edge"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_LOW)
    }
    #[doc = "TxD Change:- Falling XCK edge, RxD Sample:- Rising XCK edge"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_HIGH)
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader<DORDSELECT_A>;
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORDSELECT_A {
    #[doc = "0: MSB is transmitted first"]
    MSB = 0,
    #[doc = "1: LSB is transmitted first"]
    LSB = 1,
}
impl From<DORDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DORDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DORDSELECT_A {
        match self.bits {
            false => DORDSELECT_A::MSB,
            true => DORDSELECT_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == DORDSELECT_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == DORDSELECT_A::LSB
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, DORDSELECT_A, O>;
impl<'a, const O: u8> DORD_W<'a, O> {
    #[doc = "MSB is transmitted first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DORDSELECT_A::MSB)
    }
    #[doc = "LSB is transmitted first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DORDSELECT_A::LSB)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&self) -> SAMPR_R {
        SAMPR_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TXPO_R {
        TXPO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RXPO_R {
        RXPO_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SAMPA_R {
        SAMPA_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CMODE_R {
        CMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IBON_W<8> {
        IBON_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<9> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<10> {
        RXINV_W::new(self)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    #[must_use]
    pub fn sampr(&mut self) -> SAMPR_W<13> {
        SAMPR_W::new(self)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn txpo(&mut self) -> TXPO_W<16> {
        TXPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn rxpo(&mut self) -> RXPO_W<20> {
        RXPO_W::new(self)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn sampa(&mut self) -> SAMPA_W<22> {
        SAMPA_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<24> {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmode(&mut self) -> CMODE_W<28> {
        CMODE_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<29> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<30> {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_EXT Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
