use super::pins::*;
use atsamd_hal::ehal::blocking::delay::DelayMs;
use atsamd_hal::prelude::*;
use display_interface_parallel_gpio::{Generic8BitBus, PGPIO8BitInterface};
use ili9341::{DisplaySize, Ili9341};

pub use ili9341::{DisplaySize240x320, DisplaySize320x480, Orientation, Scroller};

type LcdDataBus =
    Generic8BitBus<LcdData0, LcdData1, LcdData2, LcdData3, LcdData4, LcdData5, LcdData6, LcdData7>;

pub type Lcd = Ili9341<PGPIO8BitInterface<LcdDataBus, TftRs, TftWr>, TftReset>;

impl Display {
    /// Initialize the display. Return a tuple containing the configured display
    /// driver struct, the backlight pin, and the tearing effect pin
    pub fn init(
        self,
        display_size: impl DisplaySize,
        orientation: Orientation,
        delay: &mut impl DelayMs<u16>,
    ) -> (Lcd, TftBacklightReset, TftTeReset) {
        let bus: LcdDataBus = Generic8BitBus::new((
            self.lcd_data0.into(),
            self.lcd_data1.into(),
            self.lcd_data2.into(),
            self.lcd_data3.into(),
            self.lcd_data4.into(),
            self.lcd_data5.into(),
            self.lcd_data6.into(),
            self.lcd_data7.into(),
        ))
        .unwrap();

        let interface = PGPIO8BitInterface::new(bus, self.tft_rs.into(), self.tft_wr.into());
        // set to high when not in use
        self.tft_rd.into_push_pull_output().set_high().unwrap();

        // set to low to enable display
        self.tft_cs.into_push_pull_output().set_low().unwrap();

        let ili9341 = Ili9341::new(
            interface,
            self.tft_reset.into(),
            delay,
            orientation,
            display_size,
        )
        .unwrap();

        (ili9341, self.tft_backlight, self.tft_te)
    }
}
