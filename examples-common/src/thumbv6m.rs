use crate::{bsp, Board, UsbBus, UsbBusAllocator};

pub struct OurBoard {}

/// List of USB interrupts to enable/disable when needed
pub const USB_INTERRUPTS: [bsp::pac::interrupt; 1] = [bsp::pac::interrupt::USB];

impl Board for OurBoard {
    fn init(
        mut peripherals: bsp::pac::Peripherals,
    ) -> (
        bsp::hal::clock::GenericClockController,
        Option<bsp::RedLed>,
        Option<UsbBusAllocator<UsbBus>>,
    ) {
        let mut clocks = bsp::hal::clock::GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);

        let led = if enable_led {
            Some(pins.d13.into())
        } else {
            None
        };

        let usb = if enable_usb {
            Some(bsp::usb_allocator(
                peripherals.USB,
                &mut clocks,
                &mut peripherals.PM,
                pins.usb_dm,
                pins.usb_dp,
            ))
        } else {
            None
        };

        (clocks, led, usb)
    }
}
