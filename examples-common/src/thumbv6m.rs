use crate::{bsp, Board, UsbBus, UsbBusAllocator};

pub struct OurBoard {}

/// List of USB interrupts to enable/disable when needed
pub const USB_INTERRUPTS: [bsp::pac::interrupt; 1] = [bsp::pac::interrupt::USB];

impl Board for OurBoard {
    fn init(
        mut peripherals: bsp::pac::Peripherals,
    ) -> (
        bsp::hal::clock::GenericClockController,
        bsp::RedLed,
        UsbBusAllocator<UsbBus>,
    ) {
        let mut clocks = bsp::hal::clock::GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);

        let led = pins.d13.into();

        let usb = bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        );

        (clocks, led, usb)
    }
}
