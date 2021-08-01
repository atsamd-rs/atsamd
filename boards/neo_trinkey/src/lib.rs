#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use embedded_hal as ehal;
pub use hal::pac;

#[cfg(feature = "usb")]
pub use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_pins!(
    PA03 {
        name: touch1,
        aliases: {
            AlternateB: Touch1
        }
    },

    PA05 {
        name: neo_pixel,
        aliases: {
            AlternateB: NeoPixel
        }
    },

    PA07 {
        name: touch2,
        aliases: { AlternateB: Touch2 }
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
