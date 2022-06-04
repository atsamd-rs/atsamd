use super::buttons::ButtonPins;
use super::display::Display;
use super::sensors::{Accelerometer, LightSensor};
use super::serial::{Uart, Usb};
use super::sound::{Buzzer, Microphone};
use super::storage::{QSPIFlash, SDCard};
use super::wifi::WifiPins;

/// [`Pin`](atsamd_hal::gpio::Pin) aliases defined by the
/// [`bsp_pins!`](atsamd_hal::bsp_pins) macro
pub mod aliases {
    atsamd_hal::bsp_pins!(
        PA15 {
        /// User LED
            name: user_led,
            aliases: {
                PushPullOutput: UserLed,
                Reset: UserLedReset
            }
        }
        PC26 {
            name: button1
            aliases: {
                FloatingInterrupt: Button1,
                Reset: Button1Reset
            }
        }
        PC27 {
            name: button2
            aliases: {
                FloatingInterrupt: Button2,
                Reset: Button2Reset
            }
        }
        PC28 {
            name: button3
            aliases: {
                FloatingInterrupt: Button3,
                Reset: Button3Reset
            }
        }
        PD08 {
            name: switch_x
            aliases: {
                FloatingInterrupt: SwitchX,
                Reset: SwitchXReset
            }
        }
        PD09 {
            name: switch_y
            aliases: {
                FloatingInterrupt: SwitchY,
                Reset: SwitchYReset
            }
        }
        PD10 {
            name: switch_z
            aliases: {
                FloatingInterrupt: SwitchZ,
                Reset: SwitchZReset
            }
        }
        PD12 {
            name: switch_b
            aliases: {
                FloatingInterrupt: SwitchB,
                Reset: SwitchBReset
            }
        }
        PD20 {
            name: switch_u
            aliases: {
                FloatingInterrupt: SwitchU,
                Reset: SwitchUReset
            }
        }
        PA12 {
            name: i2c0_scl,
            aliases: {
                AlternateD: I2c0Scl,
                Reset: I2c0SclReset
            }
        }
        PA13 {
            name: i2c0_sda,
            aliases: {
                AlternateD: I2c0Sda,
                Reset: I2c0SdaReset
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
                AlternateC: UartTx,
                Reset: UartTxReset
            }
        }
        PB27 {
            name: uart_rx,
            aliases: {
                AlternateC: UartRx,
                Reset: UartRxReset
            }
        }
        PA24 {
            name: usb_dm
            aliases: {
                AlternateH: UsbDm,
                Reset: UsbDmReset
            }
        }
        PA25 {
            name: usb_dp
            aliases: {
                AlternateH: UsbDp,
                Reset: UsbDpReset
            }
        }
        PA27 {
            name: usb_host_en
        }
        PB18 {
            name: lcd_miso
            aliases: {
                AlternateD: LcdMiso,
                Reset: LcdMisoReset
            }
        }
        PB19 {
            name: lcd_mosi
            aliases: {
                AlternateD: LcdMosi,
                Reset: LcdMosiReset
            }
        }
        PB20 {
            name: lcd_sck
            aliases: {
                AlternateD: LcdSck,
                Reset: LcdSckReset
            }
        }
        PB21 {
            name: lcd_cs
            aliases: {
                PushPullOutput: LcdCs,
                Reset: LcdCsReset
            }
        }
        PC05 {
            name: lcd_backlight
            aliases: {
                PushPullOutput: LcdBacklight,
                Reset: LcdBacklightReset
            }
        }
        PC06 {
            name: lcd_dc
            aliases: {
                PushPullOutput: LcdDc,
                Reset: LcdDcReset
            }
        }
        PC07 {
            name: lcd_reset
            aliases: {
                PushPullOutput: LcdReset,
                Reset: LcdResetReset
            }
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
            aliases: {
                AlternateF: BuzzerCtrl,
                Reset: BuzzerCtrlReset
            }
        }
        PC30 {
            /// MICROPHONE
            name: mic_output,
            aliases: {
                AlternateB: MicOutput,
                Reset: MicOutputReset
            }
        }
        PA08 {
            name: mcu_flash_qspi_io0
            aliases: {
                AlternateH: QspiD0,
                Reset: QspiD0Reset
            }
        }
        PA09 {
            name: mcu_flash_qspi_io1
            aliases: {
                AlternateH: QspiD1,
                Reset: QspiD1Reset
            }
        }
        PA10 {
            name: mcu_flash_qspi_io2
            aliases: {
                AlternateH: QspiD2,
                Reset: QspiD2Reset
            }
        }
        PA11 {
            name: mcu_flash_qspi_io3
            aliases: {
                AlternateH: QspiD3,
                Reset: QspiD3Reset
            }
        }
        PB10 {
            name: mcu_flash_qspi_clk
            aliases: {
                AlternateH: QspiSck,
                Reset: QspiSckReset
            }
        }
        PB11 {
            name: mcu_flash_qspi_cs
            aliases: {
                AlternateH: QspiCs,
                Reset: QspiCsReset
            }
        }
        PC16 {
            name: sd_mosi,
            aliases: {
                AlternateC: SdMosi,
                Reset: SdMosiReset
            }
        }
        PC17 {
            name: sd_sck,
            aliases: {
                AlternateC: SdSck,
                Reset: SdSckReset
            }
        }
        PC18 {
            name: sd_miso,
            aliases: {
                AlternateC: SdMiso,
                Reset: SdMisoReset
            }
        }
        PC19 {
            name: sd_cs,
            aliases: {
                PushPullOutput: SdCs,
                Reset: SdCsReset
            }
        }
        PD21 {
            name: sd_det,
            aliases: {
                FloatingInput: SdDet,
                Reset: SdDetReset
            }
        }
        PA18 {
            name: rtl8720d_chip_pu
            aliases: {
                PushPullOutput: WifiPwr,
                Reset: WifiPwrReset,
            }
        }
        PB24 {
            name: rtl8720d_hspi_mosi,
            aliases: {
                AlternateC: WifiTx,
                Reset: WifiTxReset
            }
        }
        PB25 {
            name: rtl8720d_hspi_clk
            aliases: {
                Reset: WifiClkReset
            }
        }
        PC22 {
            name: rtl8720d_rxd
            aliases: {
                Reset: WifiRxdReset
            }
        }
        PC23 {
            name: rtl8720d_txd
            aliases: {
                Reset: WifiTxdReset
            }
        }
        PC24 {
            name: rtl8720d_hspi_miso,
            aliases: {
                AlternateC: WifiRx,
                Reset: WifiRxReset
            }
        }
        PC25 {
            name: rtl8720d_hspi_cs
            aliases: {
                Reset: WifiCsReset
            }
        }
        PC20 {
            name: rtl8720d_data_ready
            aliases: {
                Reset: WifiReadyReset
            }
        }
        PA19 {
            name: rtl8720d_dir
            aliases: {
                Reset: WifiDirReset
            }
        }
        PB08 {
            name: a0_d0
            aliases: {
                Reset: A0D0Reset
            }
        }
        PB09 {
            name: a1_d1
            aliases: {
                Reset: A1D1Reset
            }
        }
        PA07 {
            name: a2_d2
            aliases: {
                Reset: A2D2Reset
            }
        }
        PB04 {
            name: a3_d3
            aliases: {
                Reset: A3D3Reset
            }
        }
        PB05 {
            name: a4_d4
            aliases: {
                Reset: A4D4Reset
            }
        }
        PB06 {
            name: a5_d5
            aliases: {
                Reset: A5D5Reset
            }
        }
        PA04 {
            name: a6_d6
            aliases: {
                Reset: A6D6Reset
            }
        }
        PB07 {
            name: a7_d7
            aliases: {
                Reset: A7D7Reset
            }
        }
        PA06 {
            name: a8_d8
            aliases: {
                Reset: A8D8Reset
            }
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
                AlternateB: LightSensorAdc,
                Reset: LightSensorAdcReset
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
}
pub use aliases::Pins;
use aliases::*;

/// Sets of pins split apart by category
pub struct Sets {
    /// Accelerometer I2C pins
    pub accelerometer: Accelerometer,

    /// Buzzer pins
    pub buzzer: Buzzer,

    /// LCD display pins
    pub display: Display,

    /// QSPI Flash pins
    pub flash: QSPIFlash,

    /// Analog Light Sensor pins
    pub light_sensor: LightSensor,

    /// Microphone output pins
    pub microphone: Microphone,

    /// SD Card pins
    pub sd_card: SDCard,

    /// UART (external pinout) pins
    pub uart: Uart,

    /// USB pins
    pub usb: Usb,

    /// LED pin
    pub user_led: UserLedReset,

    pub buttons: ButtonPins,
    // WiFi pins
    pub wifi: WifiPins,

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

        let user_led = self.user_led;

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
    pub a0_d0: A0D0Reset,
    pub a1_d1: A1D1Reset,
    pub a2_d2: A2D2Reset,
    pub a3_d3: A3D3Reset,
    pub a4_d4: A4D4Reset,
    pub a5_d5: A5D5Reset,
    pub a6_d6: A6D6Reset,
    pub a7_d7: A7D7Reset,
    pub a8_d8: A8D8Reset,
}
