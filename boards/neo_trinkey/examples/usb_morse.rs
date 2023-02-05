#![no_std]
#![no_main]
use panic_halt as _;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use neo_trinkey as bsp;

use bsp::entry;
use bsp::hal;
use bsp::pac;

use hal::clock::GenericClockController;
use hal::usb::UsbBus;

// Added
use hal::delay::Delay;
// use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

use smart_leds::{hsv::RGB8, SmartLedsWrite};
use ws2812_timer_delay::Ws2812;
// End Added
use pac::{interrupt, CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
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
                .manufacturer("Agilistas!")
                .product("Serial port")
                .serial_number("TRINKEY_MORSE")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());
    let neo_pixel = pins.neo_pixel.into_push_pull_output();
    let mut ws2812 = Ws2812::new(timer, neo_pixel);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 4;
    let off = [RGB8::default(); NUM_LEDS];
    let on = [
        RGB8::new(5, 5, 0),
        RGB8::new(0, 5, 5),
        RGB8::new(5, 0, 5),
        RGB8::new(2, 2, 2),
    ];

    loop {
        let letter = pop();
        emit_morse_letter(letter);

        ws2812.write(off.iter().cloned()).unwrap();
        delay.delay_ms(500u16);

        unsafe {
            let colors: [RGB8; 4] = [
                RGB8::new(MORSE_QUEUE[0], 5, 0),
                RGB8::new(MORSE_QUEUE[1], 5, 0),
                RGB8::new(MORSE_QUEUE[2], 5, 0),
                RGB8::new(MORSE_QUEUE[3], 5, 0),
            ];
            ws2812.write(colors.iter().cloned()).unwrap();
        }
        delay.delay_ms(500u16);

        cycle_delay(15 * 1024 * 1024);
    }
}

fn pop() -> char {
    return 's';
}

static mut MORSE_QUEUE: [u8; 4] = [1, 2, 3, 4];
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write("Received: ".as_bytes()).ok();
                        serial.write(&[c.clone()]).ok();
                        serial.write("\r\n".as_bytes()).ok();
                        emit_morse_letter('e');
                        // emit_morse_letter(&[c.clone()].as_char());
                    }
                };
            });
        });
    };
}

fn print_to_serial(message: &str) {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                serial.write(message.as_bytes()).ok();
            });
        });
    };
}

fn emit_morse_letter(letter: char) {
    let downcased_letter = letter.to_ascii_lowercase(); // Add support for Latin 1 later.
    match downcased_letter {
        'a' => {
            emit_morse_dot();
            emit_morse_dash();
        }
        'b' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dot();
        }
        'c' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dot();
        }
        'd' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dot();
        }
        'e' => emit_morse_dot(),
        'f' => {
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dot();
        }
        'g' => {
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dot();
        }
        'h' => {
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dot();
        }
        'i' => {
            emit_morse_dot();
            emit_morse_dot();
        }
        'j' => {
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dash();
        }
        'k' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dash();
        }
        'l' => {
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dot();
        }
        'm' => {
            emit_morse_dash();
            emit_morse_dash();
        }
        'n' => {
            emit_morse_dash();
            emit_morse_dash();
        }
        'o' => {
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dash();
        }
        'p' => {
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dot();
        }
        'q' => {
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dash();
        }
        'r' => {
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dot();
        }
        's' => {
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dot();
        }
        't' => emit_morse_dash(),
        'u' => {
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dash();
        }
        'v' => {
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dash();
        }
        'w' => {
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dash();
        }
        'x' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dot();
            emit_morse_dash();
        }
        'y' => {
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dash();
            emit_morse_dash();
        }
        'z' => {
            emit_morse_dash();
            emit_morse_dash();
            emit_morse_dot();
            emit_morse_dot();
        }
        _ => {
            emit_morse_space();
        }
    }
}

const INTERVAL: u16 = 500u16;

fn emit_morse_dot() {
    print_to_serial(".");
    // let neo_pixel = pins.neo_pixel.into_push_pull_output();
    // let mut ws2812 = Ws2812::new(timer, neo_pixel);
    // ws2812.write(on.iter().cloned()).unwrap();
    // delay.delay_ms(INTERVAL);
    // ws2812.write(off.iter().cloned()).unwrap();
    // delay.delay_ms(INTERVAL);
    unsafe {
        MORSE_QUEUE = [2, 3, 4, 1];
    }
}
fn emit_morse_dash() {
    print_to_serial("_");
    unsafe {
        MORSE_QUEUE = [8, 7, 6, 5];
    }
    // let neo_pixel = pins.neo_pixel.into_push_pull_output();
    // let mut ws2812 = Ws2812::new(timer, neo_pixel);
    // ws2812.write(on.iter().cloned()).unwrap();
    // delay.delay_ms(3 * INTERVAL);
    // ws2812.write(off.iter().cloned()).unwrap();
    // delay.delay_ms(INTERVAL);
}
fn emit_morse_space() {
    // let neo_pixel = pins.neo_pixel.into_push_pull_output();
    // let mut ws2812 = Ws2812::new(timer, neo_pixel);
    // ws2812.write(off.iter().cloned()).unwrap();
    // delay.delay_ms(7 * INTERVAL);
}

#[interrupt]
fn USB() {
    // Note: USB is the name of the interrupt, you can not attach the #[interrupt]
    // tag to poll_usb. Although you could add the contents of poll_usb into
    // this function, separating them allows you to add more functions to run on
    // the USB interrupt in the future.
    poll_usb();
}
