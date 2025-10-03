#![no_std]
#![no_main]
#![allow(static_mut_refs)]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use core::cell::OnceCell;
use core::sync::atomic;

use cortex_m::peripheral::NVIC;
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{SdCard, VolumeIdx, VolumeManager};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::hal;
use feather_m0 as bsp;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::{ClockGenId, ClockSource, GenericClockController};
use hal::delay::Delay;
use hal::fugit::RateExtU32;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::rtc;
use hal::timer::TimerCounter;
use hal::usb::UsbBus;

#[entry]
fn main() -> ! {
    // setup basic peripherals
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // configure the peripherals we'll need
    // get the internal 32k running at 1024 Hz for the RTC
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::Gclk3, 32, ClockSource::Osc32k, true)
        .unwrap();

    // Setup timer clock from
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();

    // Setup a timer for the SPI driver delay source
    let timer_spi = TimerCounter::tc4_(tc45, peripherals.tc4, &mut peripherals.pm);

    // Setup a timer for the SPI driver delay source
    let timer_sdcard = TimerCounter::tc5_(tc45, peripherals.tc5, &mut peripherals.pm);

    // Setup the RTC for the SD card volume time source
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let rtc = rtc::Rtc::clock_mode(peripherals.rtc, rtc_clock.freq(), &mut peripherals.pm);

    // Setup red led
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    red_led.set_high().unwrap();
    delay.delay_ms(500_u32);

    let bus_allocator = unsafe {
        let _ = USB_ALLOCATOR.set(bsp::usb_allocator(
            peripherals.usb,
            &mut clocks,
            &mut peripherals.pm,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.get().unwrap()
    };

    unsafe {
        let _ = USB_SERIAL.set(SerialPort::new(bus_allocator));
        let _ = USB_BUS.set(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")])
                .expect("Failed to set strings")
                .device_class(USB_CLASS_CDC)
                .build(),
        );

        // Enable USB interrupt
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    red_led.set_low().unwrap();
    delay.delay_ms(500_u32);

    // Now work on the SD peripherals. Slow SPI speed required on init
    let spi_sercom = periph_alias!(peripherals.spi_sercom);
    let spi_bus = bsp::spi_master(
        &mut clocks,
        4.MHz(),
        spi_sercom,
        &mut peripherals.pm,
        pins.sclk,
        pins.mosi,
        pins.miso,
    );

    red_led.set_high().unwrap();
    delay.delay_ms(500_u32);

    let sd_cd: bsp::SdCd = pins.sd_cd.into();
    let mut sd_cs: bsp::SdCs = pins.sd_cs.into();
    sd_cs.set_high().unwrap();

    red_led.set_low().unwrap();
    delay.delay_ms(500_u32);

    while !USB_DATA_RECEIVED.load(atomic::Ordering::Relaxed) {
        delay.delay_ms(250_u32);
        red_led.toggle().unwrap();
    }

    if sd_cd.is_low().unwrap() {
        serial_writeln!("No card detected. Waiting...");
        while sd_cd.is_low().unwrap() {
            delay.delay_ms(250_u32);
        }
    }
    serial_writeln!("Card inserted!");
    delay.delay_ms(250_u32);

    let spi_device = ExclusiveDevice::new(spi_bus, sd_cs, timer_spi).unwrap();
    let sdcard = SdCard::new(spi_device, timer_sdcard);

    match sdcard.num_bytes() {
        Ok(num_bytes) => {
            serial_writeln!("OK!\r\nCard size...");
            serial_writeln!("{} bytes", num_bytes);

            let volume_manager = VolumeManager::new(sdcard, rtc);

            for i in 0..=3 {
                let volume = volume_manager.open_volume(VolumeIdx(i));
                serial_writeln!("volume {:?}", volume);
                if let Ok(volume) = volume {
                    match volume.open_root_dir() {
                        Ok(root_dir) => {
                            serial_writeln!("Listing root directory:");
                            root_dir
                                .iterate_dir(|x| {
                                    serial_writeln!("\tFound: {:?}", x);
                                })
                                .unwrap();
                        }
                        Err(e) => serial_writeln!("Directory err: {:?}!", e),
                    }
                }
            }
        }
        Err(e) => serial_writeln!("Init err: {:?}!", e),
    }

    serial_writeln!("Done!");
    loop {
        delay.delay_ms(1_000_u32);
        red_led.toggle().unwrap();
    }
}

static mut USB_ALLOCATOR: OnceCell<UsbBusAllocator<UsbBus>> = OnceCell::new();
static mut USB_BUS: OnceCell<UsbDevice<UsbBus>> = OnceCell::new();
static mut USB_SERIAL: OnceCell<SerialPort<UsbBus>> = OnceCell::new();
static USB_DATA_RECEIVED: atomic::AtomicBool = atomic::AtomicBool::new(false);

/// Borrows the global singleton `UsbSerial` for a brief period with interrupts
/// disabled
///
/// # Arguments
/// `borrower`: The closure that gets run borrowing the global `UsbSerial`
///
/// # Safety
/// the global singleton `UsbSerial` can be safely borrowed because we disable
/// interrupts while it is being borrowed, guaranteeing that interrupt handlers
/// like `USB` cannot mutate `UsbSerial` while we are as well.
///
/// # Panic
/// If `init` has not been called and we haven't initialized our global
/// singleton `UsbSerial`, we will panic.
fn usbserial_get<T, R>(borrower: T) -> R
where
    T: Fn(&mut SerialPort<UsbBus>) -> R,
{
    usb_free(|_| unsafe {
        let usb_serial = USB_SERIAL.get_mut().expect("UsbSerial not initialized");
        borrower(usb_serial)
    })
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
#[inline]
fn usb_free<F, R>(f: F) -> R
where
    F: FnOnce(&cortex_m::interrupt::CriticalSection) -> R,
{
    NVIC::mask(interrupt::USB);

    let r = f(&unsafe { cortex_m::interrupt::CriticalSection::new() });

    unsafe {
        NVIC::unmask(interrupt::USB);
    };

    r
}

/// Writes the given message out over USB serial.
///
/// # Arguments
/// * println args: variable arguments passed along to `core::write!`
///
/// # Warning
/// as this function deals with a static mut, and it is also accessed in the
/// USB interrupt handler, we both have unsafe code for unwrapping a static mut
/// as well as disabling of interrupts while we do so.
///
/// # Safety
/// the only time the static mut is used, we have interrupts disabled so we know
/// we have sole access
#[macro_export]
macro_rules! serial_writeln {
    ($($tt:tt)+) => {{
        use core::fmt::Write;

        let mut s: heapless::String<256> = heapless::String::new();
        core::write!(&mut s, $($tt)*).unwrap();
        usbserial_get(|usbserial| {
            usbserial.write(s.as_bytes()).ok();
            usbserial.write("\r\n".as_bytes()).ok();
        });
    }};
}

#[interrupt]
fn USB() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.get_mut() {
            if let Some(serial) = USB_SERIAL.get_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];
                if let Ok(count) = serial.read(&mut buf) {
                    if count > 0 {
                        USB_DATA_RECEIVED.store(true, atomic::Ordering::Relaxed);
                    }
                }
            };
        };
    };
}
