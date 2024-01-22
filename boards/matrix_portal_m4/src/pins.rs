// //! matrix_portal_m4 pins

// reference URL https://github.com/tinygo-org/tinygo/blob/release/src/machine/board_matrixportal-m4.go
use super::hal;

hal::bsp_pins!(
    PA02 {
        // STEMMA connectors
        // ADC (A0)
        name: a0,
        aliases: {
            Reset: A0Reset,
        }
    }
    PA27 {
        /// Accelerometer Interrupt
        /// LIS3DH IRQ
        name: accelerometer_interrupt,
        aliases: {
            Reset: AccelerometerInterruptReset,
        }
    }
    PA07 {
        // ADC (A4)
        name: a4,
        aliases: {
            Reset: A4Reset,
        }
    }
    PA04 {
        // ADC (A2)
        // SPI SDO
        name: a2,
        aliases: {
            AlternateC: Sdo,
            Reset: A2Reset,
        }
    }
    PA05 {
        // ADC (A1)
        // SPI SCK
        name: a1,
        aliases: {
            AlternateC: Sck,
            Reset: A1Reset,
        }
    }
    PB23 {
        // Button "Down"
        name: button_down,
        aliases: {
            PushPullOutput: Button,
            Reset: ButtonDownReset,
        }
    }
    PB22 {
        // Button "Up"
        name: button_up,
        aliases: {
            PushPullOutput: ButtonUp,
            Reset: ButtonUpReset,
        }
    }

    PA00 {
        // UART TX
        name: uart_tx,
        aliases: {
            AlternateC: UartTx,
            Reset: UartTxReset,
        }
    }
    PB04 {
        // MTX G2
        // HUB75 G2
        name: mtx_g2,
        aliases: {
            Reset: MtxG2Reset,
        }
    }
    PB05 {
        // MTX B2
        // HUB75 B2
        name: mtx_b2,
        aliases: {
            Reset: MtxB2Reset,
        }
    }
    PB06 {
        // MTX CLK
        // HUB75 CLK
        name: mtx_clk,
        aliases: {
            Reset: MtxClkReset,
        }
    }
    PB07 {
        // MTX ADDRA
        // HUB75 ADDR A
        name: mtx_addra,
        aliases: {
            Reset: MtxAddraReset,
        }
    }
    PB09 {
        // MTX ADDRC
        // HUB75 ADDR C
        name: mtx_addrc,
        aliases: {
            Reset: MtxAddrcReset,
        }
    }
    PB31 {
        // Accelerometer I2C SDA
        // I2C SDA
        name: sda,
        aliases: {
            AlternateD: Sda,
            Reset: SdaReset,
        }
    }
    PA16 {
        // SCK
        // ESP32 SPI SCK
        name: esp_sck,
        aliases: {
            PushPullOutput: EspSck,
            Reset: EspSckReset,
        }
    }
    PA17 {
        // MIS0
        // ESP32 SPI SDI
        name: miso,
        aliases: {
            PushPullOutput: Miso,
            Reset: MisoReset,
        }
    }
    PA18 {
        // ESP RTS
        // ESP32 RTS
        name: esp_rts,
        aliases: {
            Reset: EspRtsReset,
        }
    }
    PA19 {
        // MOSI
        // ESP32 SPI SDO
        name: mosi,
        aliases: {
            PushPullOutput: Mosi,
            Reset: MosiReset,
        }
    }
    PA20 {
        // ESP GPIO0
        // ESP32 GPIO0
        name: esp_gpio0,
        aliases: {
            Reset: EspGpio0Reset
        }
    }
    PA21 {
        // ESP RESET
        // ESP32 Reset
        name: esp_reset,
        aliases: {
            Reset: EspResetReset,
        }
    }
    PA22 {
        // ESP BUSY
        // ESP32 Busy
        name: esp_busy,
        aliases: {
            Reset: EspBusyReset,
        }
    }
    PA23 {
        /// Neopixel status LED
        name: neopixel,
        aliases: {
            Reset: NeopixelReset,
            PushPullOutput: Neopixel,
        }
    }

    PB00 {
        // MTX R1
        // HUB75 R1
        name: mtx_r1,
        aliases: {
            AlternateB: MtxR1,
            Reset: MtxR1Reset,
        }
    }
    PB01 {
        // MTX G1
        // HUB75 G1
        name: mtx_g1,
        aliases: {
            AlternateB: MtxG1,
            Reset: MtxG1Reset,
        }
    }
    PA06 {
        // ADC (A3)
        name: a3,
        aliases: {
            AlternateB: A3,
            Reset: A3Reset,
        }
    }
    PB08 {
        // MTX ADDRB
        // HUB75 ADDR B
        name: mtx_addrb,
        aliases: {
            AlternateB: MtxAddrb,
            Reset: MtxAddrbReset,
        }
    }

    PB14 {
        // MTX LAT
        // HUB75 LAT
        name: mtx_lat,
        aliases: {
            AlternateB: MtxLat,
            Reset: MtxLatReset,
        }
    }
    PB15 {
        // MTX ADDRD
        // HUB75 ADDR D
        name: mtx_addrd,
        aliases: {
            Reset: MtxAddrdReset,
        }
    }
    PB17 {
        // ESP CS
        // ESP32 SPI CS
        name: esp_cs,
        aliases: {
            Reset: EspCsReset,
        }
    }

    PB12 {
        // MTX OE
        // HUB75 OE
        name: mtx_oe,
        aliases: {
            AlternateC: MtxOe,
            Reset: MtxOeReset,
        }
    }
    PB13 {
        // MTX ADDRE
        // HUB75 ADDR E
        name: mtx_addre,
        aliases: {
            AlternateC: MtxAddre,
            Reset: MtxAddreReset,
        }
    }

    PA12 {
        // ESP RX
        // ESP32 UART RX
        name: esp_rx,
        aliases: {
            AlternateC: EspUartRx,
            Reset: EspUartRxReset,
        }
    }
    PA13 {
        // ESP TX
        // ESP32 UART TX
        name: esp_tx,
        aliases: {
            AlternateC: EspUartTx,
            Reset: EspUartTxReset,
        }
    }
    PA14 {
        // D13 LED
        name: led,
        aliases: {
            PushPullOutput: RedLed,
            Reset: RedLedReset,
            AlternateG: RedLedPwm,
        }
    }

    PB02 {
        // MTX B1
        // HUB75 B1
        name: mtx_b1,
        aliases: {
            AlternateD: MtxB1,
            Reset: MtxB1Reset,
        }
    }
    PB03 {
        // MTX R2
        // HUB75 R2
        name: mtx_r2,
        aliases: {
            AlternateD: MtxR2,
            Reset: MtxR2Reset,
        }
    }

    PB30 {
        // Accelerometer I2C SCL
        // I2C SCL
        name: scl,
        aliases: {
            AlternateD: Scl,
            Reset: SclReset,
        }
    }
    PA01 {
        // RX
        // UART RX
        name: uart_rx,
        aliases: {
            AlternateC: UartRx,
            Reset: UartRxReset,
        }
    }

    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateH: UsbDm,
            Reset: UsbDmReset,
        }
    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateH: UsbDp,
            Reset: UsbDpReset,
        }
    }
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let stemma = Stemma { a0: self.a0 };

        let mtx = Matrix {
            r1: self.mtx_r1,
            g1: self.mtx_g1,
            b1: self.mtx_b1,
            r2: self.mtx_r2,
            g2: self.mtx_g2,
            b2: self.mtx_b2,
            clk: self.mtx_clk,
            lat: self.mtx_lat,
            oe: self.mtx_oe,
            addra: self.mtx_addra,
            addrb: self.mtx_addrb,
            addrc: self.mtx_addrc,
            addrd: self.mtx_addrd,
            addre: self.mtx_addre,
        };

        let esp = Esp {
            cs: self.esp_cs,
            gpio0: self.esp_gpio0,
            busy: self.esp_busy,
            reset: self.esp_reset,
            rts: self.esp_rts,
        };

        let esp_uart = EspUart {
            tx: self.esp_tx,
            rx: self.esp_rx,
        };

        let spi = Spi {
            mosi: self.mosi,
            sck: self.esp_sck,
            miso: self.miso,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let usb = Usb {
            usb_dm: self.usb_dm,
            usb_dp: self.usb_dp,
        };

        Sets {
            led: self.led,
            neopixel: self.neopixel,
            esp,
            i2c,
            spi,
            stemma,
            esp_uart,
            usb,
            mtx,
        }
    }
}

pub struct Sets {
    pub led: RedLedReset,
    pub neopixel: NeopixelReset,
    pub esp: Esp,
    pub i2c: I2C,
    pub spi: Spi,
    pub stemma: Stemma,
    pub esp_uart: EspUart,
    pub usb: Usb,
    pub mtx: Matrix,
}

pub struct Esp {
    pub cs: EspCsReset,
    pub gpio0: EspGpio0Reset,
    pub busy: EspBusyReset,
    pub reset: EspResetReset,
    pub rts: EspRtsReset,
}

pub struct I2C {
    pub sda: SdaReset,
    pub scl: SclReset,
}

pub struct Spi {
    pub mosi: MosiReset,
    pub sck: EspSckReset,
    pub miso: MisoReset,
}

pub struct Stemma {
    pub a0: A0Reset,
}

pub struct EspUart {
    pub tx: EspUartTxReset,
    pub rx: EspUartRxReset,
}

pub struct Usb {
    pub usb_dm: UsbDmReset,
    pub usb_dp: UsbDpReset,
}

pub struct Matrix {
    pub r1: MtxR1Reset,
    pub g1: MtxG1Reset,
    pub b1: MtxB1Reset,
    pub r2: MtxR2Reset,
    pub g2: MtxG2Reset,
    pub b2: MtxB2Reset,
    pub clk: MtxClkReset,
    pub lat: MtxLatReset,
    pub oe: MtxOeReset,
    pub addra: MtxAddraReset,
    pub addrb: MtxAddrbReset,
    pub addrc: MtxAddrcReset,
    pub addrd: MtxAddrdReset,
    pub addre: MtxAddreReset,
}
