#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::{asm::delay as asm_delay, peripheral::NVIC};
use hal::{clock::GenericClockController, prelude::*, usb::UsbBus};
use pac::{interrupt, CorePeripherals, Peripherals};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use bsp::{entry, hal, pac, Led0, Led1};
use xiao_m0 as bsp;

struct UsbSerial {
    usb_bus: UsbBusAllocator<UsbBus>,
    serial: SerialPort<'static, UsbBus>,
    led_data: Led1,
}

struct Environment {
    usb_serial: Option<UsbSerial>,
    led0: Led0,
    peripherals: Peripherals,
    core: CorePeripherals,
    clocks: GenericClockController,
    pins: bsp::Pins,
    bus_allocator: UsbBusAllocator<UsbBus>,
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut LED_DATA: Option<Led1> = None;

fn init() -> Environment {
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

    let usb_serial = unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0xdead, 0xbeef))
                .manufacturer("Agilistas!")
                .product("xiao_usb_morse")
                .serial_number("1")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
        LED_DATA = Some(pins.led1.into_mode());
        UsbSerial {
            usb_bus: USB_ALLOCATOR.as_ref().unwrap(),
            serial: USB_SERIAL.as_ref().unwrap(),
            led_data: LED_DATA.as_ref().unwrap(),
        }
    };

    Environment {
        usb_serial: Some(usb_serial),
        led0: pins.led0.into_push_pull_output(),
        peripherals,
        core,
        clocks,
        pins,
        bus_allocator,
    }
}

#[entry]
fn main() -> ! {
    let mut environment: Environment = init();
    let mut led0: Led0 = environment.pins.led0.into_push_pull_output();
    let mut delay = hal::delay::Delay::new(environment.core.SYST, &mut environment.clocks);

    loop {
        delay.delay_ms(200u8);
        led0.toggle().unwrap();
        delay.delay_ms(600u16);
        led0.toggle().unwrap();
    }
}
