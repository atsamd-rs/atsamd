//! USB Device support

use calibration::{usb_transn_cal, usb_transp_cal, usb_trim_cal};
use clock;
use gpio;
use target_device::usb::DEVICE;
use target_device::{PM, USB};

mod devicedesc;
use self::devicedesc::Descriptors;

/// Emit SOF at 1Khz on this pin when configured as function G
pub type SofPad = gpio::Pa23<gpio::PfG>;

/// USB D- is connected here
pub type DmPad = gpio::Pa24<gpio::PfG>;

/// USB D+ is connected here
pub type DpPad = gpio::Pa25<gpio::PfG>;

pub struct UsbDevice<'a> {
    desc: &'a mut Descriptors,
    dm_pad: DmPad,
    dp_pad: DpPad,
    usb: USB,
}

impl<'a> UsbDevice<'a> {
    pub fn new(
        _clock: &clock::UsbClock,
        pm: &mut PM,
        dm_pad: DmPad,
        dp_pad: DpPad,
        usb: USB,
        desc: &'a mut Descriptors,
    ) -> Self {
        pm.apbbmask.modify(|_, w| w.usb_().set_bit());
        Self {
            dm_pad,
            dp_pad,
            usb,
            desc,
        }
    }

    fn usb(&self) -> &DEVICE {
        unsafe { &self.usb.device }
    }

    /// Reset the USB hardware.
    pub fn reset(&mut self) {
        let addr = self.desc as *mut _ as u32;
        let usb = self.usb();
        usb.ctrla.modify(|_, w| w.swrst().set_bit());
        while usb.syncbusy.read().swrst().bit_is_set() {}

        usb.descadd.write(|w| unsafe { w.descadd().bits(addr) });
        usb.padcal.modify(|_, w| unsafe {
            w.transn().bits(usb_transn_cal());
            w.transp().bits(usb_transp_cal());
            w.trim().bits(usb_trim_cal())
        });
        usb.ctrla.modify(|_, w| {
            w.mode().device();
            w.runstdby().set_bit()
        });
        // full speed
        usb.ctrlb.modify(|_, w| w.spdconf().fs());

        usb.ctrla.modify(|_, w| w.enable().set_bit());
    }

    pub fn free(self) -> (DmPad, DpPad, USB) {
        (self.dm_pad, self.dp_pad, self.usb)
    }
}
