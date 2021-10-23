use crate::{bsp, Board, UsbBus, UsbBusAllocator};

pub struct OurBoard {}

/// List of USB interrupts to enable/disable when needed
pub const USB_INTERRUPTS: [bsp::pac::interrupt; 3] = [
    bsp::pac::interrupt::USB_OTHER,
    bsp::pac::interrupt::USB_TRCPT0,
    bsp::pac::interrupt::USB_TRCPT1,
];

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
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);

        let led = { pins.d13.into() };

        let usb = {
            bsp::usb_allocator(
                pins.usb_dm,
                pins.usb_dp,
                peripherals.USB,
                &mut clocks,
                &mut peripherals.MCLK,
            )
        };

        (clocks, led, usb)
    }
}
