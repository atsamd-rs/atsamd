//! Pin definitions for the Arduino Mkr1000 board
//!
//! Built-in pin definitions are defined here for each pin,
//! and additional pin capabilities, such as the timer/counter or waveform
//! output, and alternate sercom pad definitsions must be defined separately.

use super::hal;

hal::bsp_pins! {
    PA02 {
        /// PA02: Analog DAC output / ADC input 0, digital pin 15.
        name: d15
        aliases: {
            PushPullOutput: D15
            AlternateB: Dac0
            AlternateB: A0
            // FIXME: how to reach TCC3?
            AlternateF: D15Pwm // tcc3 wo0
        }
    }
    PA03 {
        /// PA03: ADC/DAC reference voltage.
        name: aref
        aliases: {
            AlternateB: ARef
            // FIXME: how to reach TCC3?
            AlternateF: PA03Pwm // tcc3 wo1
        }
    }
    PA04 {
        /// PA04: Analog input 3, digital pin 18.
        name: d18
        aliases: {
            PushPullOutput: D18
            AlternateB: A3
            AlternateE: D18Pwm // tcc0 wo0
            // FIXME: how to reach TCC3?
            AlternateF: D18PwmF // tcc3 wo2
        }
    }
    PA05 {
        /// PA05: Analog input 4, digital pin 19.
        name: d19
        aliases: {
            PushPullOutput: D19
            AlternateB: A4
            AlternateE: D19Pwm // tcc0 wo1
            // FIXME: how to reach TCC3?
            AlternateF: D19PwmF // tcc3 wo3
        }
    }
    PA06 {
        /// PA06: Analog input 5, digital pin 20.
        name: d20
        aliases: {
            PushPullOutput: D20
            AlternateB: A5
            AlternateE: D20Pwm // tcc1 wo0
            // FIXME: how to reach TCC3?
            AlternateF: D20PwmF // tcc3 wo4
        }
    }
    PA07 {
        /// PA07: Analog input 6, digital pin 21.
        name: d21
        aliases: {
            PushPullOutput: D21
            AlternateB: A6
            AlternateE: D21Pwm // tcc1 wo1
            // FIXME: how to reach TCC3?
            AlternateF: D21PwmF // tcc3 wo5
        }
    }
    PA08 {
        /// PA08: Sercom 2 SDA, digital pin 11.
        name: d11
        aliases: {
            PushPullOutput: D11
            AlternateD: I2cSda
            AlternateE: D11Pwm // tcc0 wo0
            AlternateF: D11PwmF // tcc1 wo2
        }
    }
    PA09 {
        /// PA09: Sercom 2 SCL, digital pin 12.
        name: d12
        aliases: {
            PushPullOutput: D12
            AlternateD: I2cScl
            // FIXME: none of these PWM work
            AlternateE: D12Pwm // tcc0 wo0
            AlternateF: D12PwmF // tcc1 wo3
        }
    }
    PA10 {
        /// PA10: Digital pin 2.
        name: d2
        aliases: {
            PushPullOutput: D2
            AlternateE: D2Pwm // tcc1 wo0
            AlternateF: D2PwmF // tcc0 wo2
        }
    }
    PA11 {
        /// PA11: Digital pin 3.
        name: d3
        aliases: {
            PushPullOutput: D3
            AlternateE: D3Pwm // tcc1 wo1
            AlternateF: D3PwmF // tcc0 wo3
        }
    }
    PA16 {
        /// PA16: Sercom 1 SPI COPI, digital pin 8.
        name: d8
        aliases: {
            PushPullOutput: D8
            AlternateC: SpiCopi
            AlternateE: D8Pwm // tcc2 wo0
            AlternateF: D8PwmF // tcc0 wo6 -> ch2
        }
    }
    PA17 {
        /// PA17: Sercom 1 SPI SCK, digital pin 9
        name: d9
        aliases: {
            PushPullOutput: D9
            AlternateC: SpiSck
            AlternateE: D9Pwm // tcc2 wo1
            AlternateF: D9PwmF // tcc0 wo7 -> ch3
        }
    }
    PA18 {
        /// PA18: USB Input Detection
        aliases: {
            FloatingDisabled: UsbId
        }
    }
    PA19 {
        /// PA19: Main SPI CIPO, digital pin 10
        name: d10
        aliases: {
            PushPullOutput: D10
            AlternateC: SpiCipo
            AlternateE: D10Pwm // tc3 wo0
            AlternateF: D10PwmF // tcc0 wo3
        }
    }
    PA20 {
        /// PA20: Digital pin 6, also board led
        name: led
        aliases: {
            PushPullOutput: Led
            AlternateE: D6Pwm // tc7 wo0
            AlternateF: D6PwmF // tcc0 wo6 -> ch2
        }
    }
    PA21 {
        /// PA21: Digital pin 7
        name: d7
        aliases: {
            PushPullOutput: D7
            AlternateE: D7Pwm // tc7 wo1
            AlternateF: D7PwmF // tcc0 wo7 -> ch3
        }
    }
    PA22 {
        /// PA22: Digital pin 0
        name: d0
        aliases: {
            PushPullOutput: D0
            AlternateE: D0Pwm // tc4 wo0
            AlternateF: D0PwmF // tcc0 wo4 -> ch0
        }
    }
    PA23 {
        /// PA23: Digital pin 1
        name: d1
        aliases: {
            PushPullOutput: D1
            AlternateE: D1Pwm // tc4 wo1
            AlternateF: D1PwmF // tcc0 wo5 -> ch1
        }
    }
    PA24 {
        /// PA24: USB Negative
        aliases: {
            AlternateG: UsbN
        }
    }
    PA25 {
        /// PA25: USB Positive
        aliases: {
            AlternateG: UsbP
        }
    }
    PB02 {
        /// PB02: Analog input 1, digital pin 16
        name: d16
        aliases: {
            AlternateB: A1
            PushPullOutput: D16
            AlternateE: D16Pwm // tc6 wo0
            // FIXME: how to reach TCC3?
            AlternateF: D16PwmF // tcc3 wo0
        }
    }
    PB03 {
        /// PB03: Analog input 2, digital pin 17
        name: d17
        aliases: {
            AlternateB: A2
            PushPullOutput: D17
            AlternateE: D17Pwm // tc6 wo1
            // FIXME: how to reach TCC3?
            AlternateF: D17PwmF // tcc3 wo3
        }
    }
    PB09 {
        /// PB09: Battery voltage sensor
        ///
        /// This pin is not visible on MKR1000 board pinout.
        aliases: {
            AlternateB: VBatt
        }
    }
    PB10 {
        /// PB10: Digital pin 4
        name: d4
        aliases: {
            PushPullOutput: D4
            AlternateE: D4Pwm // tc5 wo0
            AlternateF: D4PwmF // tcc0 wo4 -> ch0
        }
    }
    PB11 {
        /// PB11: Digital pin 5
        name: d5
        aliases: {
            PushPullOutput: D5
            AlternateE: D5Pwm // tc5 wo1
            AlternateF: D5PwmF // tcc0 wo5 -> ch1
        }
    }
    PB22 {
        /// PB22: UART Tx
        aliases: {
            AlternateD: UartTx
        }
    }
    PB23 {
        /// PB22: UART Rx
        aliases: {
            AlternateD: UartRx
        }
    }
}
