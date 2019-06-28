#![no_std]
#![no_main]

extern crate metro_m0 as hal;
extern crate panic_rtt;

use cortex_m_rt::{exception, ExceptionFrame};
use hal::clock::GenericClockController;
use hal::hal::usb::usb_device::bus::UsbBusWrapper;
use hal::hal::usb::usb_device::prelude::*;
use hal::prelude::*;
use hal::{dbgprint, entry};
use hal::{interrupt, CorePeripherals, Peripherals};

static mut USB_BUS: Option<UsbBusWrapper<hal::UsbBus>> = None;
static mut USB_DEV: Option<UsbDevice<hal::UsbBus>> = None;
static mut USB_SERIAL: Option<cdc_acm::SerialPort<hal::UsbBus>> = None;

// Minimal CDC-ACM implementation
mod cdc_acm {
    use core::cmp::min;
    use hal::hal::usb::usb_device::class_prelude::*;
    use hal::hal::usb::usb_device::utils::AtomicMutex;
    use hal::hal::usb::usb_device::Result;

    pub const USB_CLASS_CDC: u8 = 0x02;
    const USB_CLASS_DATA: u8 = 0x0a;
    const CDC_SUBCLASS_ACM: u8 = 0x02;
    const CDC_PROTOCOL_AT: u8 = 0x01;

    const CS_INTERFACE: u8 = 0x24;
    const CDC_TYPE_HEADER: u8 = 0x00;
    const CDC_TYPE_CALL_MANAGEMENT: u8 = 0x01;
    const CDC_TYPE_ACM: u8 = 0x02;
    const CDC_TYPE_UNION: u8 = 0x06;

    const REQ_SET_LINE_CODING: u8 = 0x20;
    const REQ_SET_CONTROL_LINE_STATE: u8 = 0x22;

    const BUFSIZ: usize = 16;

    struct Buf {
        buf: [u8; BUFSIZ],
        len: usize,
    }

    pub struct SerialPort<'a, B: 'a + UsbBus + Sync> {
        comm_if: InterfaceNumber,
        comm_ep: EndpointIn<'a, B>,
        data_if: InterfaceNumber,
        read_ep: EndpointOut<'a, B>,
        write_ep: EndpointIn<'a, B>,

        read_buf: AtomicMutex<Buf>,
    }

    impl<'a, B: UsbBus + Sync> SerialPort<'a, B> {
        pub fn new(bus: &'a UsbBusWrapper<B>) -> SerialPort<'a, B> {
            SerialPort {
                comm_if: bus.interface(),
                comm_ep: bus.interrupt(8, 255),
                data_if: bus.interface(),
                read_ep: bus.bulk(BUFSIZ as u16),
                write_ep: bus.bulk(BUFSIZ as u16),
                read_buf: AtomicMutex::new(Buf {
                    buf: [0; BUFSIZ],
                    len: 0,
                }),
            }
        }

        pub fn write(&self, data: &[u8]) -> Result<usize> {
            match self.write_ep.write(data) {
                Ok(count) => Ok(count),
                Err(UsbError::Busy) => Ok(0),
                e => e,
            }
        }

        pub fn read(&self, data: &mut [u8]) -> Result<usize> {
            let mut guard = self.read_buf.try_lock();

            let buf = match guard {
                Some(ref mut buf) => buf,
                None => return Ok(0),
            };

            // Terrible buffering implementation for brevity's sake

            if buf.len == 0 {
                buf.len = match self.read_ep.read(&mut buf.buf) {
                    Ok(count) => count,
                    Err(UsbError::NoData) => return Ok(0),
                    e => return e,
                };
            }

            if buf.len == 0 {
                return Ok(0);
            }

            let count = min(data.len(), buf.len);

            &data[..count].copy_from_slice(&buf.buf[0..count]);

            buf.buf.rotate_left(count);
            buf.len -= count;

            Ok(count)
        }
    }

    impl<'a, B: UsbBus + Sync> UsbClass for SerialPort<'a, B> {
        fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
            // TODO: make a better DescriptorWriter to make it harder to make invalid
            // descriptors
            writer.interface(
                self.comm_if,
                1,
                USB_CLASS_CDC,
                CDC_SUBCLASS_ACM,
                CDC_PROTOCOL_AT,
            )?;

            writer.write(CS_INTERFACE, &[CDC_TYPE_HEADER, 0x10, 0x01])?;

            writer.write(
                CS_INTERFACE,
                &[CDC_TYPE_CALL_MANAGEMENT, 0x00, self.data_if.into()],
            )?;

            writer.write(CS_INTERFACE, &[CDC_TYPE_ACM, 0x00])?;

            writer.write(
                CS_INTERFACE,
                &[CDC_TYPE_UNION, self.comm_if.into(), self.data_if.into()],
            )?;

            writer.endpoint(&self.comm_ep)?;

            writer.interface(self.data_if, 2, USB_CLASS_DATA, 0x00, 0x00)?;

            writer.endpoint(&self.write_ep)?;
            writer.endpoint(&self.read_ep)?;

            Ok(())
        }

        fn control_out(&self, req: &control::Request, buf: &[u8]) -> ControlOutResult {
            let _ = buf;

            if req.request_type == control::RequestType::Class
                && req.recipient == control::Recipient::Interface
            {
                return match req.request {
                    REQ_SET_LINE_CODING => ControlOutResult::Ok,
                    REQ_SET_CONTROL_LINE_STATE => ControlOutResult::Ok,
                    _ => ControlOutResult::Ignore,
                };
            }

            ControlOutResult::Ignore
        }
    }
}

#[entry]
fn main() -> ! {
    dbgprint!("main entered");
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut core = CorePeripherals::take().unwrap();
    /* let mut delay = Delay::new(core.SYST, &mut clocks);
     */
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_high().unwrap();

    dbgprint!("make usb_bus");

    let bus = unsafe {
        USB_BUS = Some(hal::usb_bus(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
            &mut pins.port,
        ));
        USB_BUS.as_ref().unwrap()
    };

    dbgprint!("make serial");
    let serial = unsafe {
        USB_SERIAL = Some(cdc_acm::SerialPort::new(bus));
        USB_SERIAL.as_ref().unwrap()
    };

    dbgprint!("make dev");
    unsafe {
        USB_DEV = Some(
            UsbDevice::new(bus, UsbVidPid(0x5824, 0x27dd))
                .manufacturer("Wez Furlong")
                .product("Serial port")
                .serial_number("RUST")
                .device_class(cdc_acm::USB_CLASS_CDC)
                .build(&[serial]),
        );
    }

    unsafe {
        core.NVIC.set_priority(hal::Interrupt::USB, 0);
    }
    core.NVIC.enable(hal::Interrupt::USB);

    dbgprint!("do loop");
    loop {}
}

fn poll_usb() {
    unsafe { USB_DEV.as_ref() }.map(|dev| {
        dev.poll();

        if dev.state() == UsbDeviceState::Configured {
            let mut buf = [0u8; 8];
            let serial = unsafe { USB_SERIAL.as_ref().unwrap() };

            match serial.read(&mut buf) {
                Ok(count) if count > 0 => {
                    // Echo back in upper case
                    for c in buf[0..count].iter_mut() {
                        if 0x61 <= *c && *c <= 0x7a {
                            *c &= !0x20;
                        }
                    }

                    serial.write(&buf[0..count]).ok();
                }
                _ => {}
            }
        }
    });
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    dbgprint!("hard_fault");
    panic!("{:#?}", ef);
}

#[interrupt]
fn USB() {
    poll_usb();
}

#[exception]
fn DefaultHandler(irqn: i16) {
    dbgprint!("default_handler");
    panic!("Unhandled exception (IRQn = {})", irqn);
}
