//! SAM E54 XPlained Pro Pin Definitions

use super::hal;

hal::bsp_pins!(
    PA02 {
        /// PA02: ADC/DAC header pin
        aliases: {
            AlternateB: AdcDacHeader
            AlternateB: Adc0Analog0
        }
    }
    PA03 {
        /// PA03: Extension 2 ADC pin
        aliases: {
            AlternateB: Ext2AdcMinus
            AlternateB: Adc0Analog1
        }
    }
    PA04 {
        /// PA04: Extension 1 UART transmit pin
        aliases: {
            AlternateB: Adc0Analog4
            AlternateD: Ext1UartTx
        }
    }
    PA05 {
        /// PA05: Extension 1 UART receive pin
        aliases: {
            AlternateB: Adc0Analog5
            AlternateD: Ext1UartRx
        }
    }
    PA06 {
        /// PA06: ADC 0 analog pin 6, Extension 1 GPIO pin 1,
        /// or Extension 1 UART request to send (RTS) pin
        aliases: {
            AlternateB: Adc0Analog6
            FloatingInput: Ext1Gpio1In
            PushPullOutput: Ext1Gpio1Out
            AlternateD: Ext1UartRts
        }
    }
    PA07 {
        /// PA07: ADC 0 analog pin 6, Extension 1 GPIO pin 2,
        /// or Extension 1 UART clear to send (CTS) pin
        aliases: {
            FloatingInput: Ext1Gpio2In
            PushPullOutput: Ext1Gpio2Out
            AlternateB: Adc0Analog7
            AlternateD: Ext1UartCts
        }
    }
    PA08 {
        /// PA08: QSPI data pin 0
        aliases: {
            AlternateH: QspiData0
        }
    }
    PA09 {
        /// PA09: QSPI data pin 1
        aliases: {
            AlternateH: QspiData1
        }
    }
    PA10 {
        /// PA10: QSPI data pin 2
        aliases: {
            AlternateH: QspiData2
        }
    }
    PA11 {
        /// PA11: QSPI data pin 3
        aliases: {
            AlternateH: QspiData3
        }
    }
    PA12 {
        /// PA12: PCC VSync pin or Ethernet receive data pin 1
        aliases: {
            AlternateK: PccVsync
            AlternateL: EthRxd1
        }
    }
    PA13 {
        /// PA13: PCC HSync pin or Ethernet receive data pin 0
        aliases: {
            AlternateK: PccHsync
            AlternateL: EthRxd0
        }
    }
    PA14 {
        /// PA14: PCC PCLK pin or Ethernet ref clock pin
        aliases: {
            AlternateK: PccPclk
            AlternateL: EthRefClk
        }
    }
    PA15 {
        /// PA15: PCC XCLK pin or Ethernet receive error pin
        aliases: {
            AlternateK: PccXclk
            AlternateL: EthRxer
        }
    }
    PA16 {
        /// PA16: Capacitive touch button or PCC Data out pin 00
        aliases: {
            AlternateB: QtButton
            AlternateK: PccDout00
        }
    }
    PA17 {
        /// PA17: PCC data out pin 01 or Ethernet transmit enable pin
        aliases: {
            AlternateK: PccDout01
            AlternateL: EthTxen
        }
    }
    PA18 {
        /// PA18: PCC data out pin 02 or Ethernet transmit data pin 0
        aliases: {
            AlternateK: PccDout02
            AlternateL: EthTxd0
        }
    }
    PA19 {
        /// PA19: PCC data out pin 03 or Ethernet transmit data pin 1
        aliases: {
            AlternateK: PccDout03
            AlternateL: EthTxd1
        }
    }
    PA20 {
        /// PA20: SD card command pin, I2S frame sync 0 pin, or PCC data out pin 04
        aliases: {
            AlternateI: SdCmd
            AlternateJ: I2sFs0
            AlternateK: PccDout04
        }
    }
    PA21 {
        /// PA21: SD card clock pin, I2S serial data out pin, or PCC data out pin 05
        aliases: {
            AlternateI: SdClk
            AlternateJ: I2sSdo
            AlternateK: PccDout05
        }
    }
    PA22 {
        /// PA22: Extension 1 I2c data pin, I2S serial data pin, or PCC data out pin 06
        aliases: {
            AlternateC: Ext1I2cSda
            AlternateJ: I2sSdi
            AlternateK: PccDout06
        }
    }
    PA23 {
        /// PA23: Extension 1 I2c clock pin, I2S frame sync 1 pin, or PCC data out pin 07
        aliases: {
            AlternateC: Ext1I2cScl
            AlternateJ: I2sFs1
            AlternateK: PccDout07
        }
    }
    PA24 {
        /// PA24: USB Data- pin
        aliases: {
            AlternateH: UsbDm
        }
    }
    PA25 {
        /// PA25: USB Data+ pin
        aliases: {
            AlternateH: UsbDp
        }
    }
    PA27 {
        /// PA27: Extension 1 SPI chip select pin B
        aliases: {
            PushPullOutput: Ext1SpiCsB
        }
    }
    PA30 {
        /// PA30: Serial wire clock pin
        aliases: {
            AlternateH: Swclk
        }
    }
    PA31 {
        /// PA31: Serial wire data in/out
        aliases: {
            AlternateH: Swdio
        }
    }
    PB00 {
        /// PB00: Extension 2 ADC pin
        aliases: {
            AlternateB: Ext2AdcPlus
        }
    }
    PB01 {
        /// PB01: Extension 2 GPIO pin 1
        aliases: {
            FloatingInput: Ext2Gpio1In
            PushPullOutput: Ext2Gpio1Out
        }
    }
    PB02 {
        /// PB02: Extension 2 SPI chip select pin B
        aliases: {
            PushPullOutput: Ext2SpiCsB
        }
    }
    PB04 {
        /// PB04: Extension 1 ADC plus pin, or ADC 1 analog pin 6
        aliases: {
            AlternateB: Ext1AdcPlus
            AlternateB: Adc1Analog6
        }
    }
    PB05 {
        /// PB05: Extension 1 ADC minus pin, or ADC 1 analog pin 7
        aliases: {
            AlternateB: Ext1AdcMinus
            AlternateB: Adc1Analog7
        }
    }
    PB06 {
        /// PB06: Extension 2 GPIO pin 2, or ADC 1 analog pin 8
        aliases: {
            FloatingInput: Ext2Gpio2In
            PushPullOutput: Ext2Gpio2Out
            AlternateB: Adc1Analog8
        }
    }
    PB07 {
        /// PB07: Extension 2 interrupt request/GPIO pin, or ADC 1 analog pin 9
        aliases: {
            FloatingInput: Ext1IrqGpioIn
            PushPullOutput: Ext1IrqGpioOut
            AlternateB: Adc1Analog9
        }
    }
    PB08 {
        /// PB08: Extension 1 PWM+ pin, ADC 0 analog pin 2, or ADC 1 analog pin 0
        aliases: {
            AlternateB: Adc0Analog2
            AlternateB: Adc1Analog0
            AlternateE: Ext1PwmPlus
        }
    }
    PB09 {
        /// PB09: Extension 1 PWM- pin, ADC 0 analog pin 3, or ADC 1 analog pin 1
        aliases: {
            AlternateB: Adc0Analog3
            AlternateB: Adc1Analog1
            AlternateE: Ext1PwmMinus
        }
    }
    PB10 {
        /// PB10: QSPI serial clock
        aliases: {
            AlternateH: QspiSck
        }
    }
    PB11 {
        /// PB11: QSPI chip select pin
        aliases: {
            AlternateH: QspiScs
        }
    }
    PB12 {
        /// PB12: ATA6561 on-board CAN device transmit pin
        aliases: {
            AlternateH: Ata6561Tx
        }
    }
    PB13 {
        /// PB13: ATA6561 on-board CAN device receive pin
        aliases: {
            AlternateH: Ata6561Rx
        }
    }
    PB14 {
        /// PB14: Extension 2 PWM+ pin or PCC data out pin 08
        aliases: {
            AlternateF: Ext2PwmPlus
            AlternateK: PccDout08
        }
    }
    PB15 {
        /// PB15: Extension 2 PWM- pin or PCC data out pin 09
        aliases: {
            AlternateF: Ext2PwmMinus
            AlternateK: PccDout09
        }
    }
    PB16 {
        /// PB16: Extension 2 UART transmit pin
        aliases: {
            AlternateC: Ext2UartTx
        }
    }
    PB17 {
        /// PB17: Extension 2 UART receive pin
        aliases: {
            AlternateC: Ext2UartRx
        }
    }
    PB18 {
        /// PB18: SD card data pin 0
        aliases: {
            AlternateI: SdData0
        }
    }
    PB19 {
        /// PB19: SD card data pin 1
        aliases: {
            AlternateI: SdData1
        }
    }
    PB20 {
        /// PB20: SD card data pin 2
        aliases: {
            AlternateI: SdData2
        }
    }
    PB21 {
        /// PB21: SD card data pin 3
        aliases: {
            AlternateI: SdData3
        }
    }
    PB22 {
        /// PB22: Xosc1 XIn/clock pin
        aliases: {
            FloatingDisabled: Xosc1XIn
            FloatingDisabled: Xosc1Clock
        }
    }
    PB23 {
        /// PB23: Xosc1 XOut pin
        aliases: {
            FloatingDisabled: Xosc1XOut
        }
    }
    PB24 {
        /// PB24: EDBG connection UART receive pin
        aliases: {
            AlternateD: EdbgUartRx
        }
    }
    PB25 {
        /// PB25: EDBG connection UART transmit pin
        aliases: {
            AlternateD: EdbgUartTx
        }
    }
    PB26 {
        /// PB26: Extension 1 SPI serial clock pin
        aliases: {
            AlternateD: Ext1SpiSck
        }
    }
    PB27 {
        /// PB27: Extension 1 SPI MOSI pin
        aliases: {
            AlternateD: Ext1SpiMosi
        }
    }
    PB28 {
        /// PB28: Extension 1 SPI chip select pin A
        aliases: {
            PushPullOutput: Ext1SpiCsA
        }
    }
    PB29 {
        /// PB29: Extension 1 SPI MISO pin
        aliases: {
            AlternateD: Ext1SpiMiso
        }
    }
    PB30 {
        /// PB30: Serial wire out pin
        aliases: {
            AlternateH: Swo
        }
    }
    PB31 {
        /// PB31: Switch 0, user-controlled button
        aliases: {
            PullUpInput: Button
        }
    }
    PC01 {
        /// PC01: Extension 3 GPIO pin 1
        aliases: {
            FloatingInput: Ext3Gpio1In
            PushPullOutput: Ext3Gpio1Out
        }
    }
    PC02 {
        /// PC02: Extension 3 ADC+ pin, or ADC 1 analog pin 4
        aliases: {
            AlternateB: Ext3AdcPlus
            AlternateB: Adc1Analog4
        }
    }
    PC03 {
        /// PC03: Extension 3 ADC- pin, or ADC 1 analog pin 5
        aliases: {
            AlternateB: Ext3AdcMinus
            AlternateB: Adc1Analog5
        }
    }
    PC04 {
        /// PC04: SPI MOSI pin for extension 2, 3, or data gateway interface
        aliases: {
            AlternateC: Ext2SpiMosi
            AlternateC: Ext3SpiMosi
            AlternateC: DgiSpiMosi
        }
    }
    PC05 {
        /// PC05: SPI serial clock pin for extension 2, 3, or data gateway interface
        aliases: {
            AlternateC: Ext2SpiSck
            AlternateC: Ext3SpiSck
            AlternateC: DgiSpiSck
        }
    }
    PC06 {
        /// PC06: Extension 2 SPI chip select pin A
        aliases: {
            PushPullOutput: Ext2SpiCsA
        }
    }
    PC07 {
        /// PC07: SPI MISO pin for extension 2, 3, or data gateway interface
        aliases: {
            AlternateC: Ext2SpiMiso
            AlternateC: Ext3SpiMiso
            AlternateC: DgiSpiMiso
        }
    }
    PC10 {
        /// PC10: Extension 3 GPIO pin 2
        aliases: {
            FloatingInput: Ext3Gpio2In
            PushPullOutput: Ext3Gpio2Out
        }
    }
    PC11 {
        /// PC11: PCC PWDN pin, or Ethernet MDC pin
        aliases: {
            PushPullOutput: PccPwdn
            AlternateL: EthMdc
        }
    }
    PC12 {
        /// PC12: PCC reset pin, or ethernet GMDIO pin
        aliases: {
            PushPullOutput: PccReset
            AlternateL: EthGmdio
        }
    }
    PC13 {
        /// PC13: ATA6561 on-board CAN device standby pin
        aliases: {
            PushPullOutput: Ata6561Standby
        }
    }
    PC14 {
        /// PC14: Extension 3 SPI chip select pin A
        aliases: {
            PushPullOutput: Ext3SpiCsA
        }
    }
    PC16 {
        /// PC16: Position decoder phase A pin, or embedded debugger GPIO pin 0
        aliases: {
            AlternateG: PdecPhaseA
            FloatingInput: EdbgGpio0In
            PushPullOutput: EdbgGpio0Out
        }
    }
    PC17 {
        /// PC17: Position decoder phase B pin, or embedded debugger GPIO pin 1
        aliases: {
            AlternateG: PdecPhaseB
            FloatingInput: EdbgGpio1In
            PushPullOutput: EdbgGpio1Out
        }
    }
    PC18 {
        /// PC18: LED output, position decoder index pin, or embedded debugger GPIO pin 2
        aliases: {
            PushPullOutput: Led
            AlternateG: PdecIndex
            FloatingInput: EdbgGpio2In
            PushPullOutput: EdbgGpio2Out
        }
    }
    PC20 {
        /// PC20: Ethernet carrier sense / data valid pin
        aliases: {
            AlternateL: EthCrsDv
        }
    }
    PC21 {
        /// PC21: Ethernet reset pin
        aliases: {
            FloatingInput: EthReset
        }
    }
    PC22 {
        /// PC22: Extension 3 UART transmit pin
        aliases: {
            AlternateC: Ext3UartTx
        }
    }
    PC23 {
        /// PC23: Extension 3 UART receive pin
        aliases: {
            AlternateC: Ext3UartRx
        }
    }
    PC30 {
        /// PC30: Extension 3 interrupt request/GPIO pin
        aliases: {
            FloatingInput: Ext3IrqGpioIn
            PushPullOutput: Ext3IrqGpioOut
        }
    }
    PC31 {
        /// PC31: Extension 3 SPI chip select pin B
        aliases: {
            PushPullOutput: Ext3SpiCsB
        }
    }
    PD00 {
        /// PD00: Extension 2 interrupt request/GPIO pin, or ADC 1 analog pin 14
        aliases: {
            FloatingInput: Ext2IrqGpioIn
            PushPullOutput: Ext2IrqGpioOut
            AlternateB: Adc1Analog14
        }
    }
    PD01 {
        /// PD01: Data gateway interface chip select pin, or ADC 1 analog pin 15
        aliases: {
            PushPullOutput: DgiSpiCs
            AlternateB: Adc1Analog15
        }
    }
    PD08 {
        /// PD08: Extension 2, 3, and data gateway interface I2C serial data pin
        aliases: {
            AlternateC: Ext2I2cSda
            AlternateC: Ext3I2cSda
            AlternateC: DgiI2cSda
        }
    }
    PD09 {
        /// PD09: Extension 2, 3, and data gateway interface I2C serial clock pin
        aliases: {
            AlternateC: Ext2I2cScl
            AlternateC: Ext3I2cScl
            AlternateC: DgiI2cScl
        }
    }
    PD10 {
        /// PD10: Extension 3 PWM+ pin
        aliases: {
            AlternateF: Ext3PwmPlus
        }
    }
    PD11 {
        /// PD11: Extension 3 PWM- pin
        aliases: {
            AlternateF: Ext3PwmMinus
        }
    }
    PD12 {
        /// PD12: Ethernet interrupt pin
        aliases: {
            FloatingInput: EthInterrupt
        }
    }
    PD20 {
        /// PD20: SD card card detect pin
        aliases: {
            AlternateI: SdCd
        }
    }
    PD21 {
        /// PD21: SD card write protect pin
        aliases: {
            AlternateI: SdWp
        }
    }
);
