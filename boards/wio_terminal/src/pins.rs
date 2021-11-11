use atsamd_hal::{self as hal, gpio::v2::*};

use super::buttons::ButtonPins;
use super::display::Display;
use super::sensors::{Accelerometer, LightSensor};
use super::serial::{Uart, Usb};
use super::sound::{Buzzer, Microphone};
use super::storage::{QSPIFlash, SDCard};
use super::wifi::WifiPins;

hal::bsp_pins!(
    PA15 {
    /// User LED
        name: user_led,
        aliases: {
            PushPullOutput: UserLed
        }
    }
    PC26 {
        name: button1
    }
    PC27 {
        name: button2
    }
    PC28 {
        name: button3
    }
    PD08 {
        name: switch_x
    }
    PD09 {
        name: switch_y
    }
    PD10 {
        name: switch_z
    }
    PD12 {
        name: switch_b
    }
    PD20 {
        name: switch_u
    }
    PA12 {
        name: i2c0_scl,
        aliases: {
            AlternateD: I2c0Scl
        }
    }
    PA13 {
        name: i2c0_sda,
        aliases: {
            AlternateD: I2c0Sda
        }
    }
    PA16 {
        name: i2c1_scl
    }
    PA17 {
        name: i2c1_sda
    }
    PB00 {
        name: spi_miso
    }
    PB01 {
        name: spi_cs
    }
    PB02 {
        name: spi_mosi
    }
    PB03 {
        name: spi_sck
    }
    PB26 {
        name: uart_tx,
        aliases: {
            AlternateC: UartTx
        }
    }
    PB27 {
        name: uart_rx,
        aliases: {
            AlternateC: UartRx
        }
    }
    PA24 {
        name: usb_dm
    }
    PA25 {
        name: usb_dp
    }
    PA27 {
        name: usb_host_en
    }
    PB18 {
        name: lcd_miso
    }
    PB19 {
        name: lcd_mosi
    }
    PB20 {
        name: lcd_sck
    }
    PB21 {
        name: lcd_cs
    }
    PC05 {
        name: lcd_backlight
    }
    PC06 {
        name: lcd_dc
    }
    PC07 {
        name: lcd_reset
    }
    PC10 {
        name: lcd_xl
    }
    PC11 {
        name: lcd_yu
    }
    PC12 {
        name: lcd_xr
    }
    PC13 {
        name: lcd_yd
    }
    PC21 {
        name: gyroscope_int1
    }
    PA20 {
        name: i2s_lrclk
    }
    PA21 {
        name: i2s_sdin
    }
    PA22 {
        name: i2s_sdout
    }
    PB16 {
        name: i2s_blck
    }
    PD11 {
    /// BUZZER
        name: buzzer_ctr
    }
    PC30 {
    /// MICROPHONE
        name: mic_output,
        aliases: {
            AlternateB: MicOutput,
        }
    }
    PA08 {
        name: mcu_flash_qspi_io0
    }
    PA09 {
        name: mcu_flash_qspi_io1
    }
    PA10 {
        name: mcu_flash_qspi_io2
    }
    PA11 {
        name: mcu_flash_qspi_io3
    }
    PB10 {
        name: mcu_flash_qspi_clk
    }
    PB11 {
        name: mcu_flash_qspi_cs
    }
    PC16 {
        name: sd_mosi,
        aliases: {
            AlternateC: SdMosi
        }
    }
    PC17 {
        name: sd_sck,
        aliases: {
            AlternateC: SdSck
        }
    }
    PC18 {
        name: sd_miso,
        aliases: {
            AlternateC: SdMiso
        }
    }
    PC19 {
        name: sd_cs,
        aliases: {
            PushPullOutput: SdCs
        }
    }
    PD21 {
        name: sd_det,
    }
    PA18 {
        name: rtl8720d_chip_pu
    }
    PB24 {
        name: rtl8720d_hspi_mosi,
        aliases: {
            AlternateC: WifiTx
        }
    }
    PB25 {
        name: rtl8720d_hspi_clk
    }
    PC22 {
        name: rtl8720d_rxd
    }
    PC23 {
        name: rtl8720d_txd
    }
    PC24 {
        name: rtl8720d_hspi_miso,
        aliases: {
            AlternateC: WifiRx
        }
    }
    PC25 {
        name: rtl8720d_hspi_cs
    }
    PC20 {
        name: rtl8720d_data_ready
    }
    PA19 {
        name: rtl8720d_dir
    }
    PB08 {
        name: a0_d0
    }
    PB09 {
        name: a1_d1
    }
    PA07 {
        name: a2_d2
    }
    PB04 {
        name: a3_d3
    }
    PB05 {
        name: a4_d4
    }
    PB06 {
        name: a5_d5
    }
    PA04 {
        name: a6_d6
    }
    PB07 {
        name: a7_d7
    }
    PA06 {
        name: a8_d8
    }
    PB28 {
        name: fpc_d3_pwm3
    }
    PB17 {
        name: fpc_d4_pwm4
    }
    PB29 {
        name: fpc_d5_pwm5
    }
    PA14 {
        name: fpc_d6_pwm6
    }
    PC01 {
        name: fpc_d7_a7
    }
    PC02 {
        name: fpc_d8_a8
    }
    PC03 {
        name: fpc_d9_a9
    }
    PC04 {
        name: fpc_d10_pwm10
    }
    PC31 {
        name: fpc_d11_a11
    }
    PD00 {
        name: fpc_d12_a12
    }
    PD01 {
        name: fpc_d13_a13,
        aliases: {
            AlternateB: LightSensorAdc
        }
    }
    PA02 {
        name: dac0
    }
    PA05 {
        name: dac1
    }
    PB14 {
        name: gpclk0
    }
    PB12 {
        name: gpclk1
    }
    PB13 {
        name: gpclk2
    }
    PA30 {
        name: swdclk
    }
    PA31 {
        name: swdio
    }
    PB22 {
        name: xin
    }
    PB23 {
        name: xout
    }
    PB30 {
        name: swo
    }
    PB31 {
        name: ir_ctl
    }
    PC14 {
        name: output_ctr_5v
    }
    PC15 {
        name: output_ctr_3v3
    },
);

/// Sets of pins split apart by category
pub struct Sets {
    /// Accelerometer I2C pins
    pub accelerometer: Accelerometer<Pin<PA12, Disabled<Floating>>, Pin<PA13, Disabled<Floating>>>,

    /// Buzzer pins
    pub buzzer: Buzzer<Pin<PD11, Disabled<Floating>>>,

    /// LCD display pins
    pub display: Display,

    /// QSPI Flash pins
    pub flash: QSPIFlash,

    /// Analog Light Sensor pins
    pub light_sensor: LightSensor<Pin<PD01, Disabled<Floating>>>,

    /// Microphone output pins
    pub microphone: Microphone<Pin<PC30, Disabled<Floating>>>,

    /// SD Card pins
    pub sd_card: SDCard,

    /// UART (external pinout) pins
    pub uart: Uart<Pin<PB27, Disabled<Floating>>, Pin<PB26, Disabled<Floating>>>,

    /// USB pins
    pub usb: Usb<Pin<PA24, Disabled<Floating>>, Pin<PA25, Disabled<Floating>>>,

    /// LED pin
    pub user_led: Pin<PA15, PushPullOutput>,

    pub buttons: ButtonPins,
    // WiFi pins
    pub wifi: WifiPins<
        Pin<PA18, Disabled<Floating>>,
        Pin<PC22, Disabled<Floating>>,
        Pin<PC23, Disabled<Floating>>,
        Pin<PB24, Disabled<Floating>>,
        Pin<PB25, Disabled<Floating>>,
        Pin<PC24, Disabled<Floating>>,
        Pin<PC25, Disabled<Floating>>,
        Pin<PC20, Disabled<Floating>>,
        Pin<PA19, Disabled<Floating>>,
    >,

    pub header_pins: HeaderPins,
}

impl Pins {
    /// Split the device pins into subsets
    pub fn split(mut self) -> Sets {
        let accelerometer = Accelerometer {
            scl: self.i2c0_scl,
            sda: self.i2c0_sda,
        };

        let buzzer = Buzzer {
            ctr: self.buzzer_ctr,
        };

        let display = Display {
            miso: self.lcd_miso,
            mosi: self.lcd_mosi,
            sck: self.lcd_sck,
            cs: self.lcd_cs,
            dc: self.lcd_dc,
            reset: self.lcd_reset,
            backlight: self.lcd_backlight,
        };

        let flash = QSPIFlash {
            sck: self.mcu_flash_qspi_clk,
            cs: self.mcu_flash_qspi_cs,
            d0: self.mcu_flash_qspi_io0,
            d1: self.mcu_flash_qspi_io1,
            d2: self.mcu_flash_qspi_io2,
            d3: self.mcu_flash_qspi_io3,
        };

        let light_sensor = LightSensor {
            pd1: self.fpc_d13_a13,
        };

        let microphone = Microphone {
            mic: self.mic_output,
        };

        let sd_card = SDCard {
            cs: self.sd_cs,
            mosi: self.sd_mosi,
            sck: self.sd_sck,
            miso: self.sd_miso,
            det: self.sd_det,
        };

        let uart = Uart {
            rx: self.uart_rx,
            tx: self.uart_tx,
        };

        let usb = Usb {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let user_led = self.user_led.into();
        let header_pins = HeaderPins {
            a0_d0: self.a0_d0,
            a1_d1: self.a1_d1,
            a2_d2: self.a2_d2,
            a3_d3: self.a3_d3,
            a4_d4: self.a4_d4,
            a5_d5: self.a5_d5,
            a6_d6: self.a6_d6,
            a7_d7: self.a7_d7,
            a8_d8: self.a8_d8,
        };

        let buttons = ButtonPins {
            button1: self.button1,
            button2: self.button2,
            button3: self.button3,
            switch_x: self.switch_x,
            switch_y: self.switch_y,
            switch_z: self.switch_z,
            switch_u: self.switch_u,
            switch_b: self.switch_b,
        };

        let wifi = WifiPins {
            pwr: self.rtl8720d_chip_pu,
            rxd: self.rtl8720d_rxd,
            txd: self.rtl8720d_txd,
            mosi: self.rtl8720d_hspi_mosi,
            clk: self.rtl8720d_hspi_clk,
            miso: self.rtl8720d_hspi_miso,
            cs: self.rtl8720d_hspi_cs,
            ready: self.rtl8720d_data_ready,
            dir: self.rtl8720d_dir,
        };

        Sets {
            accelerometer,
            buzzer,
            display,
            flash,
            light_sensor,
            microphone,
            sd_card,
            uart,
            usb,
            user_led,
            buttons,
            wifi,
            header_pins,
        }
    }
}

/// Other pins broken out to the RPi-compatible header.
pub struct HeaderPins {
    pub a0_d0: Pin<PB08, Disabled<Floating>>,
    pub a1_d1: Pin<PB09, Disabled<Floating>>,
    pub a2_d2: Pin<PA07, Disabled<Floating>>,
    pub a3_d3: Pin<PB04, Disabled<Floating>>,
    pub a4_d4: Pin<PB05, Disabled<Floating>>,
    pub a5_d5: Pin<PB06, Disabled<Floating>>,
    pub a6_d6: Pin<PA04, Disabled<Floating>>,
    pub a7_d7: Pin<PB07, Disabled<Floating>>,
    pub a8_d8: Pin<PA06, Disabled<Floating>>,
}
