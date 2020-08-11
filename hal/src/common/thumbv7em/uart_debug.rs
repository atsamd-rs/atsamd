use crate::gpio;
use crate::sercom::*;
use cortex_m::interrupt::free as disable_interrupts;

pub static mut WRITER: DbgWriter = DbgWriter { uart: None };

pub struct DbgWriter {
    uart: Option<
        UART3<Sercom3Pad1<gpio::Pa16<gpio::PfD>>, Sercom3Pad0<gpio::Pa17<gpio::PfD>>, (), ()>,
    >,
}

impl ::core::fmt::Write for DbgWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        match &mut self.uart {
            Some(uart) => uart.write_str(s),
            None => Ok(()),
        }
    }
}

pub fn wire_uart(
    uart: UART3<Sercom3Pad1<gpio::Pa16<gpio::PfD>>, Sercom3Pad0<gpio::Pa17<gpio::PfD>>, (), ()>,
) {
    disable_interrupts(|_| unsafe {
        WRITER = DbgWriter { uart: Some(uart) };
    });
}
