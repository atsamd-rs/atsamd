// Note: section 7.2.3 shows which pins support I2C Hs mode
use atsamd21g18a::sercom0::I2CM;
use atsamd21g18a::{SERCOM3, GCLK, PM};
use clock::wait_for_gclk_sync;
use clock::Clocks;
use gpio;
use hal::blocking::i2c::{Read, Write, WriteRead};
use time::Hertz;

// sercom0[0]:  PA04:D   PA08:C
// sercom0[1]:  PA05:D   PA09:C
// sercom0[2]:  PA06:D   PA10:C
// sercom0[3]:  PA07:D   PA11:C

pub enum Sercom0Pad0 {
    Pa4(gpio::Pa4<gpio::PfD>),
    Pa8(gpio::Pa8<gpio::PfC>),
}

pub enum Sercom0Pad1 {
    Pa5(gpio::Pa5<gpio::PfD>),
    Pa9(gpio::Pa9<gpio::PfC>),
}

pub enum Sercom0Pad2 {
    Pa6(gpio::Pa6<gpio::PfD>),
    Pa10(gpio::Pa10<gpio::PfC>),
}

pub enum Sercom0Pad3 {
    Pa7(gpio::Pa7<gpio::PfD>),
    Pa11(gpio::Pa11<gpio::PfC>),
}

// sercom1[0]:  PA16:C   PA00:D
// sercom1[1]:  PA17:C   PA01:D
// sercom1[2]:  PA18:C   PA30:D
// sercom1[3]:  PA19:C   PA31:D

pub enum Sercom1Pad0 {
    Pa0(gpio::Pa0<gpio::PfD>),
    Pa16(gpio::Pa16<gpio::PfC>),
}

pub enum Sercom1Pad1 {
    Pa1(gpio::Pa1<gpio::PfD>),
    Pa17(gpio::Pa17<gpio::PfC>),
}

pub enum Sercom1Pad2 {
    Pa18(gpio::Pa18<gpio::PfC>),
    Pa30(gpio::Pa30<gpio::PfD>),
}

pub enum Sercom1Pad3 {
    Pa19(gpio::Pa19<gpio::PfC>),
    Pa31(gpio::Pa31<gpio::PfD>),
}

// sercom2[0]:  PA12:C   PA08:D
// sercom2[1]:  PA13:C   PA09:D
// sercom2[2]:  PA14:C   PA10:D
// sercom2[3]:  PA15:C   PA11:D

pub enum Sercom2Pad0 {
    Pa8(gpio::Pa8<gpio::PfD>),
    Pa12(gpio::Pa12<gpio::PfC>),
}

pub enum Sercom2Pad1 {
    Pa9(gpio::Pa9<gpio::PfD>),
    Pa13(gpio::Pa13<gpio::PfC>),
}

pub enum Sercom2Pad2 {
    Pa10(gpio::Pa10<gpio::PfD>),
    Pa14(gpio::Pa14<gpio::PfC>),
}

pub enum Sercom2Pad3 {
    Pa11(gpio::Pa11<gpio::PfD>),
    Pa15(gpio::Pa15<gpio::PfC>),
}

// sercom3[0]:  PA16:D   PA22:C
// sercom3[1]:  PA17:D   PA23:C
// sercom3[2]:  PA18:D   PA24:C   PA20:D
// sercom3[3]:  PA19:D   PA25:C   PA21:D

pub enum Sercom3Pad0 {
    Pa16(gpio::Pa16<gpio::PfD>),
    Pa22(gpio::Pa22<gpio::PfC>),
}

pub enum Sercom3Pad1 {
    Pa17(gpio::Pa17<gpio::PfD>),
    Pa23(gpio::Pa23<gpio::PfC>),
}

pub enum Sercom3Pad2 {
    Pa18(gpio::Pa18<gpio::PfD>),
    Pa20(gpio::Pa20<gpio::PfD>),
    Pa24(gpio::Pa24<gpio::PfC>),
}

pub enum Sercom3Pad3 {
    Pa19(gpio::Pa19<gpio::PfD>),
    Pa21(gpio::Pa21<gpio::PfD>),
    Pa25(gpio::Pa25<gpio::PfC>),
}

// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

pub enum Sercom4Pad0 {
    Pa12(gpio::Pa12<gpio::PfD>),
    Pb8(gpio::Pb8<gpio::PfD>),
    Pb12(gpio::Pb12<gpio::PfC>),
}

pub enum Sercom4Pad1 {
    Pa13(gpio::Pa13<gpio::PfD>),
    Pb9(gpio::Pb9<gpio::PfD>),
    Pb13(gpio::Pb13<gpio::PfC>),
}

pub enum Sercom4Pad2 {
    Pa14(gpio::Pa14<gpio::PfD>),
    Pb10(gpio::Pb10<gpio::PfD>),
    Pb14(gpio::Pb14<gpio::PfC>),
}

pub enum Sercom4Pad3 {
    Pa15(gpio::Pa15<gpio::PfD>),
    Pb11(gpio::Pb11<gpio::PfD>),
    Pb15(gpio::Pb15<gpio::PfC>),
}

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

pub enum Sercom5Pad0 {
    Pa22(gpio::Pa22<gpio::PfD>),
    Pb2(gpio::Pb2<gpio::PfD>),
    Pb16(gpio::Pb16<gpio::PfC>),
    Pb30(gpio::Pb30<gpio::PfD>),
}

pub enum Sercom5Pad1 {
    Pa23(gpio::Pa23<gpio::PfD>),
    Pb3(gpio::Pb3<gpio::PfD>),
    Pb17(gpio::Pb17<gpio::PfC>),
    Pb31(gpio::Pb31<gpio::PfD>),
}

pub enum Sercom5Pad2 {
    Pa24(gpio::Pa24<gpio::PfD>),
    Pb0(gpio::Pb0<gpio::PfD>),
    Pa20(gpio::Pa20<gpio::PfC>),
    Pb22(gpio::Pb22<gpio::PfD>),
}

pub enum Sercom5Pad3 {
    Pa25(gpio::Pa25<gpio::PfD>),
    Pb1(gpio::Pb1<gpio::PfD>),
    Pa21(gpio::Pa21<gpio::PfC>),
    Pb23(gpio::Pb23<gpio::PfD>),
}

pub struct I2CMaster3 {
    sda: Sercom3Pad0,
    scl: Sercom3Pad1,
    sercom: SERCOM3,
}

const BUS_STATE_IDLE: u8 = 1;
const BUS_STATE_OWNED: u8 = 2;

const MASTER_ACT_READ: u8 = 2;
const MASTER_ACT_STOP: u8 = 3;

impl I2CMaster3 {
    pub fn new<F: Into<Hertz>>(
        clocks: &Clocks,
        freq: F,
        sercom: SERCOM3,
        pm: &mut PM,
        gclk: &mut GCLK,
        sda: Sercom3Pad0,
        scl: Sercom3Pad1,
    ) -> Self {
        // Power up the peripheral bus clock.
        // safe because we're exclusively owning SERCOM
        pm.apbcmask.modify(|_, w| w.sercom3_().set_bit());

        // Configure clock
        gclk.clkctrl.write(|w| {
            w.id().sercom3_core();
            w.gen().gclk0();
            w.clken().set_bit()
        });
        wait_for_gclk_sync(gclk);

        unsafe {
            // reset the sercom instance
            sercom.i2cm.ctrla.modify(|_, w| w.swrst().set_bit());
            // wait for reset to complete
            while sercom.i2cm.syncbusy.read().swrst().bit_is_set()
                || sercom.i2cm.ctrla.read().swrst().bit_is_set()
            {}

            // Put the hardware into i2c master mode
            sercom.i2cm.ctrla.modify(|_, w| w.mode().i2c_master());
            // wait for configuration to take effect
            while sercom.i2cm.syncbusy.read().enable().bit_is_set() {}

            // set the baud rate
            let baud = (clocks.sysclock().0 / (2 * freq.into().0) - 1) as u8;
            sercom.i2cm.baud.modify(|_, w| w.baud().bits(baud));

            sercom.i2cm.ctrla.modify(|_, w| w.enable().set_bit());
            // wait for configuration to take effect
            while sercom.i2cm.syncbusy.read().enable().bit_is_set() {}

            // set the bus idle
            sercom
                .i2cm
                .status
                .modify(|_, w| w.busstate().bits(BUS_STATE_IDLE));
            // wait for it to take effect
            while sercom.i2cm.syncbusy.read().sysop().bit_is_set() {}
        }

        Self { sda, scl, sercom }
    }

    pub fn free(self) -> (Sercom3Pad0, Sercom3Pad1, SERCOM3) {
        (self.sda, self.scl, self.sercom)
    }

    fn start_tx_write(&mut self, addr: u8) -> Result<(), I2CError> {
        loop {
            match self.i2cm().status.read().busstate().bits() {
                BUS_STATE_IDLE | BUS_STATE_OWNED => break,
                _ => continue,
            }
        }

        // Signal start and transmit encoded address.
        unsafe {
            self.i2cm()
                .addr
                .write(|w| w.addr().bits((addr as u16) << 1));
        }

        // wait for transmission to complete
        while !self.i2cm().intflag.read().mb().bit_is_set() {}

        self.status_to_err()
    }

    fn status_to_err(&mut self) -> Result<(), I2CError> {
        let status = self.i2cm().status.read();
        if status.arblost().bit_is_set() {
            return Err(I2CError::ArbitrationLost);
        }
        if status.buserr().bit_is_set() {
            return Err(I2CError::BusError);
        }
        if status.rxnack().bit_is_set() {
            return Err(I2CError::Nack);
        }
        if status.lowtout().bit_is_set() || status.sexttout().bit_is_set()
            || status.mexttout().bit_is_set()
        {
            return Err(I2CError::Timeout);
        }

        Ok(())
    }

    fn start_tx_read(&mut self, addr: u8) -> Result<(), I2CError> {
        loop {
            match self.i2cm().status.read().busstate().bits() {
                BUS_STATE_IDLE | BUS_STATE_OWNED => break,
                _ => continue,
            }
        }

        self.i2cm().intflag.modify(|_, w| w.error().clear_bit());

        // Signal start (or rep start if appropriate)
        // and transmit encoded address.
        unsafe {
            self.i2cm()
                .addr
                .write(|w| w.addr().bits(((addr as u16) << 1) | 1));
        }

        // wait for transmission to complete
        loop {
            let intflag = self.i2cm().intflag.read();
            // If arbitration was lost, it will be signalled via the mb bit
            if intflag.mb().bit_is_set() {
                return Err(I2CError::ArbitrationLost);
            }
            if intflag.sb().bit_is_set() || intflag.error().bit_is_set() {
                break;
            }
        }

        self.status_to_err()
    }

    fn wait_sync(&mut self) {
        while self.i2cm().syncbusy.read().sysop().bit_is_set() {}
    }

    fn cmd(&mut self, cmd: u8) {
        unsafe {
            self.i2cm().ctrlb.modify(|_, w| w.cmd().bits(cmd));
        }
        self.wait_sync();
    }

    fn cmd_stop(&mut self) {
        self.cmd(MASTER_ACT_STOP)
    }

    fn cmd_read(&mut self) {
        unsafe {
            self.i2cm().ctrlb.modify(|_, w| {
                // clear bit means send ack
                w.ackact().clear_bit();
                w.cmd().bits(MASTER_ACT_READ)
            });
        }
        self.wait_sync();
    }

    fn i2cm(&mut self) -> &I2CM {
        unsafe { &self.sercom.i2cm }
    }

    fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), I2CError> {
        for b in bytes {
            unsafe {
                self.i2cm().data.write(|w| w.bits(*b));
            }

            loop {
                let intflag = self.i2cm().intflag.read();
                if intflag.mb().bit_is_set() || intflag.error().bit_is_set() {
                    break;
                }
            }
            self.status_to_err()?;
        }
        Ok(())
    }

    fn read_one(&mut self) -> u8 {
        while !self.i2cm().intflag.read().sb().bit_is_set() {}
        self.i2cm().data.read().bits()
    }

    fn fill_buffer(&mut self, buffer: &mut [u8]) -> Result<(), I2CError> {
        // Some manual iterator gumph because we need to ack bytes after the first.
        let mut iter = buffer.iter_mut();
        *iter.next().expect("buffer len is at least 1") = self.read_one();

        loop {
            match iter.next() {
                None => break,
                Some(dest) => {
                    // Ack the last byte so that we can receive another one
                    self.cmd_read();
                    *dest = self.read_one();
                }
            }
        }

        // arrange to send nack on next command to
        // stop slave from transmitting more data
        self.i2cm().ctrlb.modify(|_, w| w.ackact().set_bit());

        Ok(())
    }

    fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), I2CError> {
        self.start_tx_write(addr)?;
        self.send_bytes(bytes)
    }

    fn do_read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2CError> {
        self.start_tx_read(addr)?;
        self.fill_buffer(buffer)
    }

    fn do_write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I2CError> {
        self.start_tx_write(addr)?;
        self.send_bytes(bytes)?;
        self.start_tx_read(addr)?;
        self.fill_buffer(buffer)
    }
}

#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    AddressError,
    BusError,
    Timeout,
    Nack,
}

impl Write for I2CMaster3 {
    type Error = I2CError;

    /// Sends bytes to slave with address `addr`
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let res = self.do_write(addr, bytes);
        self.cmd_stop();
        res
    }
}

impl Read for I2CMaster3 {
    type Error = I2CError;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_read(addr, buffer);
        self.cmd_stop();
        res
    }
}

impl WriteRead for I2CMaster3 {
    type Error = I2CError;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_write_read(addr, bytes, buffer);
        self.cmd_stop();
        res
    }
}
