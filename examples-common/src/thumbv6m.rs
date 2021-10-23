use crate::{bsp, Board};

pub struct OurBoard {}

impl Board for OurBoard {
    fn init(
        mut peripherals: bsp::pac::Peripherals,
    ) -> (bsp::Pins, bsp::hal::clock::GenericClockController) {
        let clocks = bsp::hal::clock::GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);

        (pins, clocks)
    }
}
