use atsamd_hal::gpio::{self, *};
use atsamd_hal::{define_pins, target_device};

use super::buttons::ButtonPins;
use super::display::Display;
use super::sensors::{Accelerometer, LightSensor};
use super::serial::{UART, USB};
use super::sound::{Buzzer, Microphone};
use super::storage::{QSPIFlash, SDCard};
use super::wifi::WifiPins;

define_pins!(
    /// Map the desired pin names to their physical pins
    struct Pins,
    target_device: target_device,

    // NOTE:
    // The following pin names were adapted from the labels in the schematic.
    // They will likely evolve over time.
    // They're not in any particular order.

    /// USER_LED
    pin user_led = a15,

    /// BUTTONS
    pin button1 = c26,
    pin button2 = c27,
    pin button3 = c28,

    /// SWITCHES
    pin switch_x = d8,
    pin switch_y = d9,
    pin switch_z = d10,
    pin switch_b = d12,
    pin switch_u = d20,

    /// I2C
    pin i2c0_scl = a12,
    pin i2c0_sda = a13,
    pin i2c1_scl = a16,
    pin i2c1_sda = a17,

    /// SPI
    pin spi_miso = b0,
    pin spi_cs = b1,
    pin spi_mosi = b2,
    pin spi_sck = b3,

    /// UART
    pin txd = b26,
    pin rxd = b27,

    /// USB
    pin usb_dm = a24,
    pin usb_dp = a25,
    pin usb_host_en = a27,

    /// LCD
    pin lcd_miso = b18,
    pin lcd_mosi = b19,
    pin lcd_sck = b20,
    pin lcd_cs = b21,
    pin lcd_backlight = c5,
    pin lcd_dc = c6,
    pin lcd_reset = c7,
    pin lcd_xl = c10,
    pin lcd_yu = c11,
    pin lcd_xr = c12,
    pin lcd_yd = c13,

    /// GYROSCOPE
    pin gyroscope_int1 = c21,

    /// I2S
    pin i2s_lrclk = a20,
    pin i2s_sdin = a21,
    pin i2s_sdout = a22,
    pin i2s_blck = b16,

    /// BUZZER
    pin buzzer_ctr = d11,

    /// MICROPHONE
    pin mic_output = c30,

    /// MCU FLASH
    pin mcu_flash_qspi_io0 = a8,
    pin mcu_flash_qspi_io1 = a9,
    pin mcu_flash_qspi_io2 = a10,
    pin mcu_flash_qspi_io3 = a11,
    pin mcu_flash_qspi_clk = b10,
    pin mcu_flash_qspi_cs = b11,

    /// SD CARD
    pin sd_mosi = c16,
    pin sd_sck = c17,
    pin sd_miso = c18,
    pin sd_cs = c19,
    pin sd_det = d21,

    /// WIFI/BLE
    pin rtl8720d_chip_pu = a18,
    pin rtl8720d_hspi_mosi = b24,
    pin rtl8720d_hspi_clk = b25,
    pin rtl8720d_rxd = c22,
    pin rtl8720d_txd = c23,
    pin rtl8720d_hspi_miso = c24,
    pin rtl8720d_hspi_cs = c25,
    pin rtl8720d_data_ready = c20,
    pin rtl8720d_dir = a19,

    /// GPIO
    pin a0_d0 = b8,
    pin a1_d1 = b9,
    pin a2_d2 = a7,
    pin a3_d3 = b4,
    pin a4_d4 = b5,
    pin a5_d5 = b6,
    pin a6_d6 = a4,
    pin a7_d7 = b7,
    pin a8_d8 = a6,

    /// FPC
    pin fpc_d3_pwm3 = b28,
    pin fpc_d4_pwm4 = b17,
    pin fpc_d5_pwm5 = b29,
    pin fpc_d6_pwm6 = a14,
    pin fpc_d7_a7 = c1,
    pin fpc_d8_a8 = c2,
    pin fpc_d9_a9 = c3,
    pin fpc_d10_pwm10 = c4,
    pin fpc_d11_a11 = c31,
    pin fpc_d12_a12 = d0,
    pin fpc_d13_a13 = d1,

    /// DAC
    pin dac0 = a2,
    pin dac1 = a5,

    /// GPCLK
    pin gpclk0 = b14,
    pin gpclk1 = b12,
    pin gpclk2 = b13,

    /// SWD
    pin swdclk = a30,
    pin swdio = a31,

    /// XIN/XOUT
    pin xin = b22,
    pin xout = b23,

    /// MISCELLANEOUS
    pin swo = b30,
    pin ir_ctl = b31,
    pin output_ctr_5v = c14,
    pin output_ctr_3v3 = c15,
);

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

    /// GPIO port
    pub port: Port,

    /// SD Card pins
    pub sd_card: SDCard,

    /// UART (external pinout) pins
    pub uart: UART,

    /// USB pins
    pub usb: USB,

    /// LED pin
    pub user_led: Pa15<Input<Floating>>,

    pub buttons: ButtonPins,

    // WiFi pins
    pub wifi: WifiPins,

    pub header_pins: HeaderPins,
}

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
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

        let port = self.port;

        let sd_card = SDCard {
            cs: self.sd_cs,
            mosi: self.sd_mosi,
            sck: self.sd_sck,
            miso: self.sd_miso,
            det: self.sd_det,
        };

        let uart = UART {
            rx: self.rxd,
            tx: self.txd,
        };

        let usb = USB {
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
            port,
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
    pub a0_d0: Pb8<Input<Floating>>,
    pub a1_d1: Pb9<Input<Floating>>,
    pub a2_d2: Pa7<Input<Floating>>,
    pub a3_d3: Pb4<Input<Floating>>,
    pub a4_d4: Pb5<Input<Floating>>,
    pub a5_d5: Pb6<Input<Floating>>,
    pub a6_d6: Pa4<Input<Floating>>,
    pub a7_d7: Pb7<Input<Floating>>,
    pub a8_d8: Pa6<Input<Floating>>,
}
