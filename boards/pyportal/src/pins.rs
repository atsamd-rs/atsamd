//! PyPortal pins

use super::hal;

hal::bsp_pins!(
    PA02 {
        /// Analog pin 0.  Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        name: speaker,
        aliases: {
            Reset: SpeakerReset,
        }
    }
    PA27 {
        /// enable speaker amplifier
        name: speaker_enable,
        aliases: {
            Reset: SpeakerEnableReset,
        }
    }
    PA07 {
        /// Light sensor
        name: light,
        aliases: {
            AlternateB: LightSensorAdc,
            Reset: LightSensorReset,
        }
    }
    PA04 {
        // STEMMA connectors
        /// Pin D3
        name: d3,
        aliases: {
            Reset: D3Reset,
        }
    }
    PA05 {
        /// Pin D4
        name: d4,
        aliases: {
            Reset: D4Reset,
        }
    }
    PB23 {
        /// D13 LED
        name: d13,
        aliases: {
            PushPullOutput: RedLed,
            Reset: RedLedReset,
        }
    }
    PB22 {
        /// Neopixel status LED
        name: neopixel,
        aliases: {
            Reset: NeopixelReset,
            PushPullOutput: Neopixel,
        }
    }

    PA00 {
        // TFT(Thin-film-transistor liquid-crystal display) control ports
        /// TFT Reset
        name: tft_reset,
        aliases: {
            PushPullOutput: TftReset,
            Reset: TftResetReset,
        }
    }
    PB04 {
        /// TFT RD
        name: tft_rd,
        aliases: {
            PushPullOutput: TftRd,
            Reset: TftRdReset,
        }
    }
    PB05 {
        /// TFT RS
        name: tft_rs,
        aliases: {
            PushPullOutput: TftRs,
            Reset: TftRsReset,
        }
    }
    PB06 {
        /// TFT CS
        name: tft_cs,
        aliases: {
            PushPullOutput: TftCs,
            Reset: TftCsReset,
        }
    }
    PB07 {
        /// TFT TE
        name: tft_te,
        aliases: {
            Reset: TftTeReset,
        }
    }
    PB09 {
        /// TFT WR
        name: tft_wr,
        aliases: {
            PushPullOutput: TftWr,
            Reset: TftWrReset,
        }
    }
    PB31 {
        /// TFT Backlight
        name: tft_backlight,
        aliases: {
            PushPullOutput: TftBacklight,
            AlternateE: TftBacklightPwm
            Reset: TftBacklightReset,
        }
    }
    PA16 {
        /// LCD Data 0
        name: lcd_data0,
        aliases: {
            PushPullOutput: LcdData0,
            Reset: LcdData0Reset,
        }
    }
    PA17 {
        /// LCD Data 1
        name: lcd_data1,
        aliases: {
            PushPullOutput: LcdData1,
            Reset: LcdData1Reset,
        }
    }
    PA18 {
        /// LCD Data 2
        name: lcd_data2,
        aliases: {
            PushPullOutput: LcdData2,
            Reset: LcdData2Reset,
        }
    }
    PA19 {
        /// LCD Data 3
        name: lcd_data3,
        aliases: {
            PushPullOutput: LcdData3,
            Reset: LcdData3Reset,
        }
    }
    PA20 {
        /// LCD Data 4
        name: lcd_data4,
        aliases: {
            PushPullOutput: LcdData4,
            Reset: LcdData4Reset,
        }
    }
    PA21 {
        /// LCD Data 5
        name: lcd_data5,
        aliases: {
            PushPullOutput: LcdData5,
            Reset: LcdData5Reset,
        }
    }
    PA22 {
        /// LCD Data 6
        name: lcd_data6,
        aliases: {
            PushPullOutput: LcdData6,
            Reset: LcdData6Reset,
        }
    }
    PA23 {
        /// LCD Data 7
        name: lcd_data7,
        aliases: {
            PushPullOutput: LcdData7,
            Reset: LcdData7Reset,
        }
    }

    PB00 {
        // Touchscreen pins
        /// Touch YD
        name: touch_yd,
        aliases: {
            AlternateB: TouchYd,
            Reset: TouchYdReset,
        }
    }
    PB01 {
        /// Touch XL
        name: touch_xl,
        aliases: {
            AlternateB: TouchXl,
            Reset: TouchXlReset,
        }
    }
    PA06 {
        /// Touch YU
        name: touch_yu,
        aliases: {
            AlternateB: TouchYu,
            Reset: TouchYuReset,
        }
    }
    PB08 {
        /// Touch XR
        name: touch_xr,
        aliases: {
            AlternateB: TouchXr,
            Reset: TouchXrReset,
        }
    }

    PB14 {
        // ESP control - ESP32 WiFi
        /// Pin ESP CS
        name: esp_cs,
        aliases: {
            Reset: EspCsReset,
        }
    }
    PB15 {
        /// Pin ESP GPIO0
        name: esp_gpio0,
        aliases: {
            Reset: EspGpio0Reset
        }
    }
    PB16 {
        /// Pin ESP Busy
        name: esp_busy,
        aliases: {
            Reset: EspBusyReset,
        }
    }
    PB17 {
        /// Pin ESP Reset
        name: esp_reset,
        aliases: {
            Reset: EspResetReset,
        }
    }
    PA15 {
        /// Pin ESP RTS
        name: esp_rts,
        aliases: {
            Reset: EspRtsReset,
        }
    }

    PB12 {
        // UART - Universal Asynchronous Receiver/Transmitter (connected to ESP32)
        /// Pin TX
        name: esp_tx,
        aliases: {
            AlternateC: EspUartTx,
            Reset: EspUartTxReset,
        }
    }
    PB13 {
        /// Pin RX
        name: esp_rx,
        aliases: {
            AlternateC: EspUartRx,
            Reset: EspUartRxReset,
        }
    }

    PA12 {
        // SPI - Serial Peripheral Interface
        /// Pin MOSI
        name: mosi,
        aliases: {
            AlternateC: Mosi,
            Reset: MosiReset,
        }
    }
    PA13 {
        /// Pin SCK
        name: sck,
        aliases: {
            AlternateC: Sck,
            Reset: SckReset,
        }
    }
    PA14 {
        /// Pin MISO
        name: miso,
        aliases: {
            AlternateC: Miso,
            Reset: MisoReset,
        }
    }

    PB02 {
        // I2C - ADT7410 Analog Devices temperature sensor
        /// Pin SDA
        name: sda,
        aliases: {
            AlternateD: Sda,
            Reset: SdaReset,
        }
    }
    PB03 {
        /// Pin SCL
        name: scl,
        aliases: {
            AlternateD: Scl,
            Reset: SclReset,
        }
    }

    PB30 {
        /// Pin SD CS
        name: sd_cs,
        aliases: {
            Reset: SdCsReset,
        }
    }
    PA01 {
        /// Pin SD card detect
        name: sd_card_detect,
        aliases: {
            Reset: SdCardDetectReset,
        }
    }

    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm,
            Reset: UsbDmReset,
        }
    }
    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp,
            Reset: UsbDpReset,
        }
    }
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let speaker = Speaker {
            speaker: self.speaker,
            enable: self.speaker_enable,
        };

        let stemma = Stemma {
            d3: self.d3,
            d4: self.d4,
        };

        let display = Display {
            tft_reset: self.tft_reset,
            tft_rd: self.tft_rd,
            tft_rs: self.tft_rs,
            tft_cs: self.tft_cs,
            tft_te: self.tft_te,
            tft_wr: self.tft_wr,
            tft_backlight: self.tft_backlight,

            lcd_data0: self.lcd_data0,
            lcd_data1: self.lcd_data1,
            lcd_data2: self.lcd_data2,
            lcd_data3: self.lcd_data3,
            lcd_data4: self.lcd_data4,
            lcd_data5: self.lcd_data5,
            lcd_data6: self.lcd_data6,
            lcd_data7: self.lcd_data7,
        };

        let touchscreen = Touchscreen {
            yd: self.touch_yd,
            xl: self.touch_xl,
            yu: self.touch_yu,
            xr: self.touch_xr,
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
            sck: self.sck,
            miso: self.miso,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let sd = SdCard {
            cs: self.sd_cs,
            card_detect: self.sd_card_detect,
        };

        let usb = Usb {
            usb_dm: self.usb_dm,
            usb_dp: self.usb_dp,
        };

        Sets {
            display,
            esp,
            light: self.light,
            i2c,
            sd,
            speaker,
            spi,
            stemma,
            touchscreen,
            esp_uart,
            d13: self.d13,
            neopixel: self.neopixel,
            usb,
        }
    }
}

pub struct Sets {
    pub display: Display,
    pub d13: RedLedReset,
    pub neopixel: NeopixelReset,
    pub esp: Esp,
    pub light: LightSensorReset,
    pub i2c: I2C,
    pub sd: SdCard,
    pub speaker: Speaker,
    pub spi: Spi,
    pub stemma: Stemma,
    pub touchscreen: Touchscreen,
    pub esp_uart: EspUart,
    pub usb: Usb,
}

pub struct Display {
    pub tft_reset: TftResetReset,
    pub tft_rd: TftRdReset,
    pub tft_rs: TftRsReset,
    pub tft_cs: TftCsReset,
    pub tft_te: TftTeReset,
    pub tft_wr: TftWrReset,
    pub tft_backlight: TftBacklightReset,
    pub lcd_data0: LcdData0Reset,
    pub lcd_data1: LcdData1Reset,
    pub lcd_data2: LcdData2Reset,
    pub lcd_data3: LcdData3Reset,
    pub lcd_data4: LcdData4Reset,
    pub lcd_data5: LcdData5Reset,
    pub lcd_data6: LcdData6Reset,
    pub lcd_data7: LcdData7Reset,
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

pub struct SdCard {
    pub cs: SdCsReset,
    pub card_detect: SdCardDetectReset,
}

pub struct Speaker {
    pub speaker: SpeakerReset,
    pub enable: SpeakerEnableReset,
}

pub struct Spi {
    pub mosi: MosiReset,
    pub sck: SckReset,
    pub miso: MisoReset,
}

pub struct Stemma {
    pub d3: D3Reset,
    pub d4: D4Reset,
}

pub struct Touchscreen {
    pub yd: TouchYdReset,
    pub xl: TouchXlReset,
    pub yu: TouchYuReset,
    pub xr: TouchXrReset,
}

pub struct EspUart {
    pub tx: EspUartTxReset,
    pub rx: EspUartRxReset,
}

pub struct Usb {
    pub usb_dm: UsbDmReset,
    pub usb_dp: UsbDpReset,
}
