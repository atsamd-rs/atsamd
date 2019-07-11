//! PyPortal pins

use super::{hal, target_device};

use crate::hal::gpio::{self, *};
use hal::define_pins;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin speaker = a2,
    /// enable speaker amplifier
    pin speaker_enable = a27,

    /// Light sensor
    pin light = a7,

    // STEMMA connectors
    /// Pin D3
    pin d3 = a4,
    /// Pin D4
    pin d4 = a5,

    /// D13 LED
    pin d13 = b23,
    /// Neopixel status LED
    pin neopixel = b22,

    // TFT(Thin-film-transistor liquid-crystal display) control ports
    /// TFT Reset
    pin tft_reset = a0,
    /// TFT RD
    pin tft_rd = b4,
    /// TFT RS
    pin tft_rs = b5,
    /// TFT CS
    pin tft_cs = b6,
    /// TFT TE
    pin tft_te = b7,
    /// TFT WR
    pin tft_wr = b9,
    /// TFT Backlight
    pin tft_backlight = b31,

    /// LCD Data 0
    pin lcd_data0 = a16,
    /// LCD Data 1
    pin lcd_data1 = a17,
    /// LCD Data 2
    pin lcd_data2 = a18,
    /// LCD Data 3
    pin lcd_data3 = a19,
    /// LCD Data 4
    pin lcd_data4 = a20,
    /// LCD Data 5
    pin lcd_data5 = a21,
    /// LCD Data 6
    pin lcd_data6 = a22,
    /// LCD Data 7
    pin lcd_data7 = a23,

    /// Touchscreen pins
    /// Touch YD
    pin touch_yd = b0,
    /// Touch XL
    pin touch_xl = b1,
    /// Touch YU
    pin touch_yu = a6,
    /// Touch XR
    pin touch_xr = b8,

    // ESP control - ESP32 WiFi
    /// Pin ESP CS
    pin esp_cs = b14,
    /// Pin ESP GPIO0
    pin esp_gpio0 = b15,
    /// Pin ESP Busy
    pin esp_busy = b16,
    /// Pin ESP Reset
    pin esp_reset = b17,
    /// Pin ESP RTS
    pin esp_rts = a15,

    // UART - Universal Asynchronous Receiver/Transmitter (connected to ESP32)
    /// Pin TX
    pin esp_tx = b12,
    /// Pin RX
    pin esp_rx = b13,

    // SPI - Serial Peripheral Interface
    /// Pin MOSI
    pin mosi = a12,
    /// Pin SCK
    pin sck = a13,
    /// Pin MISO
    pin miso = a14,

    // I2C - ADT7410 Analog Devices temperature sensor
    /// Pin SDA
    pin sda = b2,
    /// Pin SCL
    pin scl = b3,

    /// Pin SD CS
    pin sd_cs = b30,
    /// Pin SD card detect
    pin sd_card_detect = a1,
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

        Sets {
            port: self.port,
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
        }
    }
}

pub struct Sets {
    pub port: Port,
    pub display: Display,
    pub d13: Pb23<Input<Floating>>,
    pub neopixel: Pb22<Input<Floating>>,
    pub esp: Esp,
    pub light: Pa7<Input<Floating>>,
    pub i2c: I2C,
    pub sd: SdCard,
    pub speaker: Speaker,
    pub spi: Spi,
    pub stemma: Stemma,
    pub touchscreen: Touchscreen,
    pub esp_uart: EspUart,
}

pub struct Display {
    pub tft_reset: Pa0<Input<Floating>>,
    pub tft_rd: Pb4<Input<Floating>>,
    pub tft_rs: Pb5<Input<Floating>>,
    pub tft_cs: Pb6<Input<Floating>>,
    pub tft_te: Pb7<Input<Floating>>,
    pub tft_wr: Pb9<Input<Floating>>,
    pub tft_backlight: Pb31<Input<Floating>>,
    pub lcd_data0: Pa16<Input<Floating>>,
    pub lcd_data1: Pa17<Input<Floating>>,
    pub lcd_data2: Pa18<Input<Floating>>,
    pub lcd_data3: Pa19<Input<Floating>>,
    pub lcd_data4: Pa20<Input<Floating>>,
    pub lcd_data5: Pa21<Input<Floating>>,
    pub lcd_data6: Pa22<Input<Floating>>,
    pub lcd_data7: Pa23<Input<Floating>>,
}

pub struct Esp {
    pub cs: Pb14<Input<Floating>>,
    pub gpio0: Pb15<Input<Floating>>,
    pub busy: Pb16<Input<Floating>>,
    pub reset: Pb17<Input<Floating>>,
    pub rts: Pa15<Input<Floating>>,
}

pub struct I2C {
    pub sda: Pb2<Input<Floating>>,
    pub scl: Pb3<Input<Floating>>,
}

pub struct SdCard {
    pub cs: Pb30<Input<Floating>>,
    pub card_detect: Pa1<Input<Floating>>,
}

pub struct Speaker {
    pub speaker: Pa2<Input<Floating>>,
    pub enable: Pa27<Input<Floating>>,
}

pub struct Spi {
    pub mosi: Pa12<Input<Floating>>,
    pub sck: Pa13<Input<Floating>>,
    pub miso: Pa14<Input<Floating>>,
}

pub struct Stemma {
    pub d3: Pa4<Input<Floating>>,
    pub d4: Pa5<Input<Floating>>,
}

pub struct Touchscreen {
    pub yd: Pb0<Input<Floating>>,
    pub xl: Pb1<Input<Floating>>,
    pub yu: Pa6<Input<Floating>>,
    pub xr: Pb8<Input<Floating>>,
}

pub struct EspUart {
    pub tx: Pb12<Input<Floating>>,
    pub rx: Pb13<Input<Floating>>,
}
