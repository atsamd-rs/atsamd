#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::pac;

#[cfg(feature = "usb")]
pub use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA15 {
        name: neo_pixel,
        aliases: {
            AlternateB: NeoPixel
        }
    },

    PA28 {
        name: button,
        aliases: {
            AlternateB: Buttons
        }
    },

    PA24 {
        /// The USB D- pad
        name: usb_dm
        aliases: {
            AlternateG: UsbDm
        }
    },

    PA25 {
        /// The USB D+ pad
        name: usb_dp
        aliases: {
            AlternateG: UsbDp
        }
    }
);

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut hal::clock::GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}
