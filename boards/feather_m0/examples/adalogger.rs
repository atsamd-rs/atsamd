#![no_std]
#![no_main]

use feather_m0 as hal;
use panic_halt as _;

use core::fmt::Write;
use core::sync::atomic;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use embedded_sdmmc::{Controller, SdMmcSpi, VolumeIdx};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use feather_m0::hal::clock::{ClockGenId, ClockSource, GenericClockController};
use feather_m0::hal::delay::Delay;
use feather_m0::hal::pac::{interrupt, CorePeripherals, Peripherals};
use feather_m0::hal::prelude::*;
use feather_m0::hal::rtc;
use feather_m0::hal::time::U32Ext;
use feather_m0::hal::usb::UsbBus;
use hal::entry;

use heapless::consts::U1024;
use heapless::String;

#[entry]
fn main() -> ! {
    // setup basic peripherals
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // configure the peripherals we'll need
    // get the internal 32k running at 1024 Hz for the RTC
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK3, 32, ClockSource::OSC32K, true)
        .unwrap();
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let timer = rtc::Rtc::clock_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
    let pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_push_pull_output();

    red_led.set_high().unwrap();
    delay.delay_ms(500_u32);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(hal::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    red_led.set_low().unwrap();
    delay.delay_ms(500_u32);

    // Now work on the SD peripherals. Slow SPI speed required on init
    let spi = hal::spi_master(
        &mut clocks,
        400_u32.khz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sclk,
        pins.mosi,
        pins.miso,
    );

    red_led.set_high().unwrap();
    delay.delay_ms(500_u32);

    let sd_cd = pins.sd_cd.into_pull_up_input();
    let mut sd_cs = pins.sd_cs.into_push_pull_output();
    sd_cs.set_high().unwrap();

    red_led.set_low().unwrap();
    delay.delay_ms(500_u32);

    while USB_DATA_RECEIVED.load(atomic::Ordering::Relaxed) == false {
        delay.delay_ms(250_u32);
        red_led.toggle().unwrap();
    }

    if sd_cd.is_low().unwrap() {
        usbserial_write!("No card detected. Waiting...\r\n");
        while sd_cd.is_low().unwrap() {
            delay.delay_ms(250_u32);
        }
    }
    usbserial_write!("Card inserted!\r\n");
    delay.delay_ms(250_u32);

    let mut controller = Controller::new(SdMmcSpi::new(spi, sd_cs), timer);

    match controller.device().init() {
        Ok(_) => {
            usbserial_write!("OK!\r\nCard size...\r\n");
            match controller.device().card_size_bytes() {
                Ok(size) => usbserial_write!("{} bytes\r\n", size),
                Err(e) => usbserial_write!("Err: {:?}\r\n", e),
            }

            for i in 0..=3 {
                let volume = controller.get_volume(VolumeIdx(i));
                usbserial_write!("volume {:?}\r\n", volume);
                if let Ok(volume) = volume {
                    let root_dir = controller.open_root_dir(&volume).unwrap();
                    usbserial_write!("Listing root directory:\r\n");
                    controller
                        .iterate_dir(&volume, &root_dir, |x| {
                            usbserial_write!("\tFound: {:?}\r\n", x);
                        })
                        .unwrap();
                }
            }
        }
        Err(e) => usbserial_write!("Init err: {:?}!\r\n", e),
    }

    usbserial_write!("Done!\r\n");
    loop {
        delay.delay_ms(1_000_u32);
        red_led.toggle().unwrap();
    }
}

/// Writes the given message out over USB serial.
#[macro_export]
macro_rules! usbserial_write {
    ($($tt:tt)*) => {{
        let mut s: String<U1024> = String::new();
        write!(s, $($tt)*).unwrap();
        let message_bytes = s.as_bytes();
        let mut total_written = 0;
        while total_written < message_bytes.len() {
            let bytes_written = disable_interrupts(|_| unsafe {
                match USB_SERIAL.as_mut().unwrap().write(
                    &message_bytes[total_written..]
                ) {
                    Ok(count) => count,
                    Err(_) => 0,
                }
            });
            total_written += bytes_written;
        }
    }};
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static USB_DATA_RECEIVED: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[interrupt]
fn USB() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 16];
                if let Ok(count) = serial.read(&mut buf) {
                    if count > 0 {
                        USB_DATA_RECEIVED.store(true, atomic::Ordering::Relaxed);
                    }
                }
            });
        });
    };
}
