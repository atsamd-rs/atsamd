//! Pin definitions for the Arduino Mkr1000 board
//!
//! Built-in pin definitions are defined here for each pin,
//! and additional pin capabilities, such as the timer/counter or waveform
//! output, and alternate sercom pad definitsions must be defined separately.

use super::hal;

hal::bsp_pins! {
    PA02 {
        /// PA02: Analog DAC output / ADC input 0, digital pin 15.
        aliases: {
            PushPullOutput: D15
            AlternateB: Dac0
            AlternateB: A0
        }
    }
    PA03 {
        /// PA03: ADC/DAC reference voltage.
        aliases: {
            AlternateB: ARef
        }
    }
    PA04 {
        /// PA04: Analog input 3, digital pin 18.
        aliases: {
            PushPullOutput: D18
            AlternateB: A3
        }
    }
    PA05 {
        /// PA05: Analog input 4, digital pin 19.
        aliases: {
            PushPullOutput: D19
            AlternateB: A4
        }
    }
    PA06 {
        /// PA06: Analog input 5, digital pin 20.
        aliases: {
            PushPullOutput: D20
            AlternateB: A5
        }
    }
    PA07 {
        /// PA07: Analog input 6, digital pin 21.
        aliases: {
            PushPullOutput: D21
            AlternateB: A6
        }
    }
    PA08 {
        /// PA08: Sercom 2 SDA, digital pin 11.
        aliases: {
            PushPullOutput: D11
            AlternateD: I2cSda
            AlternateE: D11Pwm
        }
    }
    PA09 {
        /// PA09: Sercom 2 SCL, digital pin 12.
        aliases: {
            PushPullOutput: D12
            AlternateD: I2cScl
        }
    }
    PA10 {
        /// PA10: Digital pin 2.
        aliases: {
            PushPullOutput: D2
        }
    }
    PA11 {
        /// PA11: Digital pin 3.
        aliases: {
            PushPullOutput: D3
        }
    }
    PA16 {
        /// PA16: Sercom 1 SPI COPI, digital pin 8.
        aliases: {
            PushPullOutput: D8
            AlternateC: SpiCopi
        }
    }
    PA17 {
        /// PA17: Sercom 1 SPI SCK, digital pin 9
        aliases: {
            PushPullOutput: D9
            AlternateC: SpiSck
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
        aliases: {
            PushPullOutput: D10
            AlternateC: SpiCipo
        }
    }
    PA20 {
        /// PA20: Digital pin 6, also board led
        aliases: {
            PushPullOutput: Led
        }
    }
    PA21 {
        /// PA21: Digital pin 7
        aliases: {
            PushPullOutput: D7
        }
    }
    PA22 {
        /// PA22: Digital pin 0
        aliases: {
            PushPullOutput: D0
        }
    }
    PA23 {
        /// PA23: Digital pin 1
        aliases: {
            PushPullOutput: D1
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
        aliases: {
            AlternateB: A1
            PushPullOutput: D16
        }
    }
    PB03 {
        /// PB03: Analog input 2, digital pin 17
        aliases: {
            AlternateB: A2
            PushPullOutput: D17
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
        aliases: {
            PushPullOutput: D4
        }
    }
    PB11 {
        /// PB11: Digital pin 5
        aliases: {
            PushPullOutput: D5
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
