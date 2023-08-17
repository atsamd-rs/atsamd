use crate::hal;
use crate::pins::{UsbN, UsbP};
use hal::pac;

use hal::clock::GenericClockController;
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: UsbN,
    dp: UsbP,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}
