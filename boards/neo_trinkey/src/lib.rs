#![no_std]

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::ehal;
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

/// Convenience method for getting the USB Bus Allocator.
///
/// Basic usage would look like the below:
/// ```no_run
/// use neo_trinkey::hal::clock::GenericClockController;
/// use neo_trinkey::pac::Peripherals;
///
/// let mut peripherals = Peripherals::take().unwrap();
/// let mut clocks = GenericClockController::with_internal_32kosc(
///     peripherals.GCLK,
///     &mut peripherals.PM,
///     &mut peripherals.SYSCTRL,
///     &mut peripherals.NVMCTRL,
/// );
/// let pins = bsp::Pins::new(peripherals.PORT);
///
/// let bus_allocator = bsp::usb_allocator(
///     peripherals.USB,
///     &mut clocks,
///     &mut peripherals.PM,
///     pins.usb_dm,
///     pins.usb_dp,
/// );
/// ```
/// However to take advantage of USB interrupts you will need, to do some unsafe
/// rust. See the USB code examples in the `examples/` directory of the project.
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
