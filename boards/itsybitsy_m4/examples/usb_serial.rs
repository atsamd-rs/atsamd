#![no_std]
#![no_main]

use bsp::hal;
/// Makes the itsybitsy_m4 appear as a USB serial port. The color of the
/// dotstar LED can be changed by sending bytes to the serial port.
///
/// Sending the characters R, G, and O set the LED red, green, and off
/// respectively. For example:
/// $> sudo stty -F /dev/ttyACM0 115200 raw -echo
/// $> sudo bash -c "echo 'R' > /dev/ttyACM0"
/// $> sudo bash -c "echo 'G' > /dev/ttyACM0"
/// $> sudo bash -c "echo 'O' > /dev/ttyACM0"
use itsybitsy_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use hal::{
    clock::GenericClockController,
    dbgprint,
    ehal::timer::CountDown,
    pac::{interrupt, CorePeripherals, Peripherals},
    time::MegaHertz,
    timer::TimerCounter,
    usb::UsbBus,
};
use smart_leds::{hsv::RGB8, SmartLedsWrite};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let gclk0 = clocks.gclk0();
    let tc2_3 = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&tc2_3, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(MegaHertz(4));
    let mut rgb = bsp::dotstar_bitbang(
        pins.dotstar_miso.into(),
        pins.dotstar_mosi.into(),
        pins.dotstar_sck.into(),
        timer,
    );
    rgb.write([RGB8 { r: 0, g: 0, b: 0 }].iter().cloned())
        .unwrap();

    dbgprint!(
        "\n\n\n\n~========== STARTING {:?} ==========~\n",
        hal::serial_number()
    );
    dbgprint!(
        "Last reset was from {:?}\n",
        hal::reset_cause(&peripherals.RSTC)
    );

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.MCLK,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    loop {
        let pending = disable_interrupts(|_| unsafe {
            let pending = PENDING_COLOR;
            PENDING_COLOR = None;
            pending
        });
        if let Some(color) = pending {
            rgb.write(color.iter().cloned()).unwrap();
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut PENDING_COLOR: Option<[RGB8; 1]> = None;

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        match *c as char {
                            'R' => {
                                PENDING_COLOR = Some([RGB8 { r: 120, g: 0, b: 0 }]);
                            }
                            'G' => {
                                PENDING_COLOR = Some([RGB8 { r: 0, g: 120, b: 0 }]);
                            }
                            'O' => {
                                PENDING_COLOR = Some([RGB8 { r: 0, g: 0, b: 0 }]);
                            }
                            _ => {}
                        }
                    }
                };
            };
        };
    };
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
