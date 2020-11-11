use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{Floating, Input, IntoFunction, Pa24, Pa25, Pb26, Pb27, PfC, Port};
use atsamd_hal::sercom::{PadPin, Sercom2Pad0, Sercom2Pad1, UART2};
use atsamd_hal::target_device::{self, MCLK, SERCOM2};
use atsamd_hal::time::Hertz;

#[cfg(feature = "usb")]
use atsamd_hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};
#[cfg(feature = "usb")]
use target_device::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

/// UART pins (uses `SERCOM2`)
pub struct UART {
    /// UART transmit pin
    pub tx: Pb26<Input<Floating>>,

    /// UART receive pin
    pub rx: Pb27<Input<Floating>>,
}

impl UART {
    /// Set up the labelled TX/RX pins to operate as a UART device at the
    /// specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom2: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()> {
        let gclk0 = clocks.gclk0();
        UART2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            baud.into(),
            sercom2,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// USB pins
pub struct USB {
    /// USB data-minus pin
    pub dm: Pa24<Input<Floating>>,

    /// USB data-plus pin
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    /// Create a USB allocator.
    pub fn usb_allocator(
        self,
        usb: target_device::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            mclk,
            self.dm.into_function(port),
            self.dp.into_function(port),
            usb,
        ))
    }
}
