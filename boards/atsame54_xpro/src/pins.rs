//! SAM E54 XPlained Pro Pin Definitions

use super::hal;

hal::bsp_pins!(
    PA02 {
        aliases: {
            AlternateB: AdcDacHeader
            AlternateB: Adc0Analog0
        }
    }
    PA03 {
        aliases: {
            AlternateB: Ext2AdcMinus
            AlternateB: Adc0Analog1
        }
    }
    PA04 {
        aliases: {
            AlternateB: Adc0Analog4
            AlternateD: Ext1UsartTx
        }
    }
    PA05 {
        aliases: {
            AlternateB: Adc0Analog5
            AlternateD: Ext1UsartRx
        }
    }
    PA06 {
        aliases: {
            AlternateB: Adc0Analog6
            FloatingInput: Ext1Gpio1In
            PushPullOutput: Ext1Gpio1Out
            AlternateD: Ext1UsartRts
        }
    }
    PA07 {
        aliases: {
            FloatingInput: Ext1Gpio2In
            PushPullOutput: Ext1Gpio2Out
            AlternateB: Adc0Analog7
            AlternateD: Ext1UsartCts
        }
    }
    PA08 {
        aliases: {
            AlternateH: QspiData0
        }
    }
    PA09 {
        aliases: {
            AlternateH: QspiData1
        }
    }
    PA10 {
        aliases: {
            AlternateH: QspiData2
        }
    }
    PA11 {
        aliases: {
            AlternateH: QspiData3
        }
    }
    PA12 {
        aliases: {
            AlternateK: PccVsync
            AlternateL: EthRxd1
        }
    }
    PA13 {
        aliases: {
            AlternateK: PccHsync
            AlternateL: EthRxd0
        }
    }
    PA14 {
        aliases: {
            AlternateK: PccPclk
            AlternateL: EthRefClk
        }
    }
    PA15 {
        aliases: {
            AlternateK: PccXclk
            AlternateL: EthRxer
        }
    }
    PA16 {
        aliases: {
            AlternateB: QtButton
            AlternateK: PccDout00
        }
    }
    PA17 {
        aliases: {
            AlternateK: PccDout01
            AlternateL: EthTxen
        }
    }
    PA18 {
        aliases: {
            AlternateK: PccDout02
            AlternateL: EthTxd0
        }
    }
    PA19 {
        aliases: {
            AlternateK: PccDout03
            AlternateL: EthTxd1
        }
    }
    PA20 {
        aliases: {
            AlternateI: SdCmd
            AlternateJ: I2sFs0
            AlternateK: PccDout04
        }
    }
    PA21 {
        aliases: {
            AlternateI: SdClk
            AlternateJ: I2sSdo
            AlternateK: PccDout05
        }
    }
    PA22 {
        aliases: {
            AlternateC: Ext1I2cSda
            AlternateJ: I2sSdi
            AlternateK: PccDout06
        }
    }
    PA23 {
        aliases: {
            AlternateC: Ext1I2cScl
            AlternateJ: I2sFs1
            AlternateK: PccDout07
        }
    }
    PA24 {
        aliases: {
            AlternateH: UsbDm
        }
    }
    PA25 {
        aliases: {
            AlternateH: UsbDp
        }
    }
    PA27 {
        aliases: {
            PushPullOutput: Ext1SpiCsB
        }
    }
    PA30 {
        aliases: {
            AlternateH: Swclk
        }
    }
    PA31 {
        aliases: {
            AlternateH: Swdio
        }
    }
    PB00 {
        aliases: {
            AlternateB: Ext2AdcPlus
        }
    }
    PB01 {
        aliases: {
            FloatingInput: Ext2Gpio1In
            PushPullOutput: Ext2Gpio1Out
        }
    }
    PB02 {
        aliases: {
            PushPullOutput: Ext2SpiCsB
        }
    }
    PB04 {
        aliases: {
            AlternateB: Ext1AdcPlus
            AlternateB: Adc1Analog6
        }
    }
    PB05 {
        aliases: {
            AlternateB: Ext1AdcMinus
            AlternateB: Adc1Analog7
        }
    }
    PB06 {
        aliases: {
            FloatingInput: Ext2Gpio2In
            PushPullOutput: Ext2Gpio2Out
            AlternateB: Adc1Analog8
        }
    }
    PB07 {
        aliases: {
            FloatingInput: Ext1IrqGpioIn
            PushPullOutput: Ext1IrqGpioOut
            AlternateB: Adc1Analog9
        }
    }
    PB08 {
        aliases: {
            AlternateB: Adc0Analog2
            AlternateB: Adc1Analog0
            AlternateE: Ext1PwmPlus
        }
    }
    PB09 {
        aliases: {
            AlternateB: Adc0Analog3
            AlternateB: Adc1Analog1
            AlternateE: Ext1PwmMinus
        }
    }
    PB10 {
        aliases: {
            AlternateH: QspiSck
        }
    }
    PB11 {
        aliases: {
            AlternateH: QspiScs
        }
    }
    PB12 {
        aliases: {
            AlternateH: Ata6561Tx
        }
    }
    PB13 {
        aliases: {
            AlternateH: Ata6561Rx
        }
    }
    PB14 {
        aliases: {
            AlternateF: Ext2PwmPlus
            AlternateK: PccDout08
        }
    }
    PB15 {
        aliases: {
            AlternateF: Ext2PwmMinus
            AlternateK: PccDout09
        }
    }
    PB16 {
        aliases: {
            AlternateC: Ext2UsartTx
        }
    }
    PB17 {
        aliases: {
            AlternateC: Ext2UsartRx
        }
    }
    PB18 {
        aliases: {
            AlternateI: SdData0
        }
    }
    PB19 {
        aliases: {
            AlternateI: SdData1
        }
    }
    PB20 {
        aliases: {
            AlternateI: SdData2
        }
    }
    PB21 {
        aliases: {
            AlternateI: SdData3
        }
    }
    PB24 {
        aliases: {
            AlternateD: EdbgUartRx
        }
    }
    PB25 {
        aliases: {
            AlternateD: EdbgUartTx
        }
    }
    PB26 {
        aliases: {
            AlternateD: Ext1SpiSck
        }
    }
    PB27 {
        aliases: {
            AlternateD: Ext1SpiMosi
        }
    }
    PB28 {
        aliases: {
            PushPullOutput: Ext1SpiCsA
        }
    }
    PB29 {
        aliases: {
            AlternateD: Ext1SpiMiso
        }
    }
    PB30 {
        aliases: {
            AlternateH: Swo
        }
    }
    PB31 {
        aliases: {
            PullUpInput: Sw0Button
        }
    }
    PC01 {
        aliases: {
            FloatingInput: Ext3Gpio1In
            PushPullOutput: Ext3Gpio1Out
        }
    }
    PC02 {
        aliases: {
            AlternateB: Ext3AdcPlus
            AlternateB: Adc1Analog4
        }
    }
    PC03 {
        aliases: {
            AlternateB: Ext3AdcMinus
            AlternateB: Adc1Analog5
        }
    }
    PC04 {
        aliases: {
            AlternateC: Ext2SpiMosi
            AlternateC: Ext3SpiMosi
            AlternateC: DgiSpiMosi
        }
    }
    PC05 {
        aliases: {
            AlternateC: Ext2SpiSck
            AlternateC: Ext3SpiSck
            AlternateC: DgiSpiSck
        }
    }
    PC06 {
        aliases: {
            PushPullOutput: Ext2SpiCsA
        }
    }
    PC07 {
        aliases: {
            AlternateC: Ext2SpiMiso
            AlternateC: Ext3SpiMiso
            AlternateC: DgiSpiMiso
        }
    }
    PC10 {
        aliases: {
            FloatingInput: Ext3Gpio2In
            PushPullOutput: Ext3Gpio2Out
        }
    }
    PC11 {
        aliases: {
            PushPullOutput: PccPwdn
            AlternateL: EthGmdc
        }
    }
    PC12 {
        aliases: {
            PushPullOutput: PccReset
            AlternateL: EthGmdio
        }
    }
    PC13 {
        aliases: {
            PushPullOutput: Ata6561Standby
        }
    }
    PC14 {
        aliases: {
            PushPullOutput: Ext3SpiCsA
        }
    }
    PC16 {
        aliases: {
            AlternateG: PdecPhaseA
            FloatingInput: EdbgGpio0In
            PushPullOutput: EdbgGpio0Out
        }
    }
    PC17 {
        aliases: {
            AlternateG: PdecPhaseB
            FloatingInput: EdbgGpio1In
            PushPullOutput: EdbgGpio1Out
        }
    }
    PC18 {
        aliases: {
            PushPullOutput: Led
            AlternateG: PdecIndex
            FloatingInput: EdbgGpio2In
            PushPullOutput: EdbgGpio2Out
        }
    }
    PC20 {
        aliases: {
            AlternateL: EthCrsDv
        }
    }
    PC21 {
        aliases: {
            FloatingInput: EthReset
        }
    }
    PC22 {
        aliases: {
            AlternateC: Ext3UsartTx
        }
    }
    PC23 {
        aliases: {
            AlternateC: Ext3UsartRx
        }
    }
    PC30 {
        aliases: {
            FloatingInput: Ext3IrqGpioIn
            PushPullOutput: Ext3IrqGpioOut
        }
    }
    PC31 {
        aliases: {
            PushPullOutput: Ext3SpiCsB
        }
    }
    PD00 {
        aliases: {
            FloatingInput: Ext2IrqGpioIn
            PushPullOutput: Ext2IrqGpioOut
            AlternateB: Adc1Analog14
        }
    }
    PD01 {
        aliases: {
            PushPullOutput: DgiSpiCs
            AlternateB: Adc1Analog15
        }
    }
    PD08 {
        aliases: {
            AlternateC: Ext2I2cSda
            AlternateC: Ext3I2cSda
            AlternateC: DgiI2cSda
        }
    }
    PD09 {
        aliases: {
            AlternateC: Ext2I2cScl
            AlternateC: Ext3I2cScl
            AlternateC: DgiI2cScl
        }
    }
    PD10 {
        aliases: {
            AlternateF: Ext3PwmPlus
        }
    }
    PD11 {
        aliases: {
            AlternateF: Ext3PwmMinus
        }
    }
    PD12 {
        aliases: {
            FloatingInput: EthInterrupt
        }
    }
    PD20 {
        aliases: {
            AlternateI: SdCd
        }
    }
    PD21 {
        aliases: {
            AlternateI: SdWp
        }
    }
);
