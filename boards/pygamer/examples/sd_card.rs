//! Place a series of bitmap images on the screen
//!
//! Ferris pngs from <https://rustacean.net/> , convert png to .bmp bytes:
//! * Resize and export images directly from image editor by saving as .bmp and
//!   choosing 16bit R5 G6 B5
//! * OR Convert with imagemagick: convert rustacean-flat-noshadow.png -type
//!   truecolor -define bmp:subtype=RGB565 -depth 16 -strip -resize 86x64
//!   ferris.bmp
//! * SD card should have one (or at least, the first) primary partition of type
//!   W95 FAT32, formatted eg `sudo mkfs.fat /dev/path/to/sdcardp1`
//! * Put assets/ferris*.bmp in the root directory of the sd card `cp
//!   assets/ferris*.bmp /Volumes/SDCARD/`

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::{image::Image, pixelcolor::Rgb565};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{Mode, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::nb;
use hal::prelude::*;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use pac::{CorePeripherals, Peripherals};
use tinybmp::Bmp;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.port).split();

    let mut red_led: RedLed = pins.led_pin.into();

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.sercom4,
            &mut peripherals.mclk,
            peripherals.tc2,
            &mut delay,
        )
        .unwrap();

    // black out the screen
    Rectangle::with_corners(Point::new(0, 0), Point::new(160, 128))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc4_tc5(&gclk0).unwrap();
    let mut khz_timer = TimerCounter::tc4_(&timer_clock, peripherals.tc4, &mut peripherals.mclk);
    InterruptDrivenTimer::start(&mut khz_timer, Hertz::kHz(1).into_duration());

    let sdmmc_cs = pins.sd_cs_pin.into_push_pull_output();
    let sdmmc_spi_bus = pins.spi.init(
        &mut clocks,
        3.MHz(),
        peripherals.sercom1,
        &mut peripherals.mclk,
    );

    let sdmmc_spi =
        ExclusiveDevice::new_no_delay(sdmmc_spi_bus, sdmmc_cs).expect("Failed to create SpiDevice");

    let card = embedded_sdmmc::SdCard::new(sdmmc_spi, delay);

    let mut volume_mgr = VolumeManager::new(card, Clock {});
    let mut volume = volume_mgr
        .open_volume(VolumeIdx(0))
        .expect("Failed to open volume");

    let mut dir = volume.open_root_dir().expect("Failed to open root dir");

    let mut scratch = [0u8; 11008];

    // "animation" order of files to open
    let images = ["ferris.bmp", "ferris1.bmp", "ferris2.bmp", "ferris1.bmp"];

    loop {
        for image in images.iter() {
            match dir.open_file_in_dir(*image, Mode::ReadOnly) {
                Ok(mut f) => {
                    let _ = f.read(&mut scratch);
                    let raw_image: Bmp<Rgb565> = Bmp::from_slice(&scratch).unwrap();
                    let ferris = Image::new(&raw_image, Point::new(32, 32));
                    let _ = ferris.draw(&mut display);
                    let _ = f.close();
                }
                Err(_err) => {
                    red_led.set_high().ok();
                }
            }

            for _ in 0..100 {
                nb::block!(InterruptDrivenTimer::wait(&mut khz_timer)).unwrap();
            }
        }
    }
}

struct Clock;

impl TimeSource for Clock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
