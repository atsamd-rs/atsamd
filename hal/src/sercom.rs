// Note: section 7.2.3 shows which pins support I2C Hs mode
use atsamd21g18a::sercom0::I2CM;
use atsamd21g18a::{SERCOM0, SERCOM1, SERCOM2, SERCOM3, SERCOM4, SERCOM5, PM};
use clock::wait_for_gclk_sync;
use clock::Clocks;
use gpio::{self, IntoFunction, Port};
use hal::blocking::i2c::{Read, Write, WriteRead};
use time::Hertz;

/// The PadPin trait makes it more ergonomic to convert a
/// pin into a Sercom pad.  You should not implement this
/// trait for yourself; only the implementations in the
/// sercom module make sense.
pub trait PadPin<T> {
    fn into_pad(self, port: &mut Port) -> T;
}

/// The pad macro helps to define enums for pads and makes it
/// a little more convenient to initialize them.
macro_rules! pad {
    ($(pub enum $PadType:ident {
        $( $PinType:ident ($new:ident, $Pf:ident),)+
    })+
    ) => {
$(
pub enum $PadType {
    $(
        $PinType(gpio::$PinType<gpio::$Pf>),
    )+
}

impl $PadType {
    $(
    /// Construct pad from the appropriate pin in any mode
    pub fn $new<MODE>(pin: gpio::$PinType<MODE>, port: &mut Port) -> Self {
        $PadType::$PinType(pin.into_function(port))
    }

    )+
}

$(
impl<MODE> PadPin<$PadType> for gpio::$PinType<MODE> {
    fn into_pad(self, port: &mut Port) -> $PadType {
        $PadType::$new(self, port)
    }
}
)+

)+
    };
}

pad!(
// sercom0[0]:  PA04:D   PA08:C
// sercom0[1]:  PA05:D   PA09:C
// sercom0[2]:  PA06:D   PA10:C
// sercom0[3]:  PA07:D   PA11:C

pub enum Sercom0Pad0 {
    Pa4(pa4, PfD),
    Pa8(pa8, PfC),
}

pub enum Sercom0Pad1 {
    Pa5(pa5, PfD),
    Pa9(pa9, PfC),
}

pub enum Sercom0Pad2 {
    Pa6(pa6, PfD),
    Pa10(pa10, PfC),
}

pub enum Sercom0Pad3 {
    Pa7(pa7, PfD),
    Pa11(pa11, PfC),
}

// sercom1[0]:  PA16:C   PA00:D
// sercom1[1]:  PA17:C   PA01:D
// sercom1[2]:  PA18:C   PA30:D
// sercom1[3]:  PA19:C   PA31:D

pub enum Sercom1Pad0 {
    Pa0(pa0, PfD),
    Pa16(pa16, PfC),
}

pub enum Sercom1Pad1 {
    Pa1(pa1, PfD),
    Pa17(pa17, PfC),
}

pub enum Sercom1Pad2 {
    Pa18(pa18, PfC),
    Pa30(pa30, PfD),
}

pub enum Sercom1Pad3 {
    Pa19(pa19, PfC),
    Pa31(pa31, PfD),
}

// sercom2[0]:  PA12:C   PA08:D
// sercom2[1]:  PA13:C   PA09:D
// sercom2[2]:  PA14:C   PA10:D
// sercom2[3]:  PA15:C   PA11:D

pub enum Sercom2Pad0 {
    Pa8(pa8, PfD),
    Pa12(pa12, PfC),
}

pub enum Sercom2Pad1 {
    Pa9(pa9, PfD),
    Pa13(pa13, PfC),
}

pub enum Sercom2Pad2 {
    Pa10(pa10, PfD),
    Pa14(pa14, PfC),
}

pub enum Sercom2Pad3 {
    Pa11(pa11, PfD),
    Pa15(pa15, PfC),
}

// sercom3[0]:  PA16:D   PA22:C
// sercom3[1]:  PA17:D   PA23:C
// sercom3[2]:  PA18:D   PA24:C   PA20:D
// sercom3[3]:  PA19:D   PA25:C   PA21:D

pub enum Sercom3Pad0 {
    Pa16(pa16, PfD),
    Pa22(pa22, PfC),
}
pub enum Sercom3Pad1 {
    Pa17(pa17, PfD),
    Pa23(pa23, PfC),
}
pub enum Sercom3Pad2 {
    Pa18(pa18, PfD),
    Pa20(pa20, PfD),
    Pa24(pa24, PfC),
}
pub enum Sercom3Pad3 {
    Pa19(pa19, PfD),
    Pa21(pa21, PfD),
    Pa25(pa25, PfC),
}

// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

pub enum Sercom4Pad0 {
    Pa12(pa12, PfD),
    Pb8(pb8, PfD),
    Pb12(pb12, PfC),
}

pub enum Sercom4Pad1 {
    Pa13(pa13, PfD),
    Pb9(pb9, PfD),
    Pb13(pb13, PfC),
}

pub enum Sercom4Pad2 {
    Pa14(pa14, PfD),
    Pb10(pb10, PfD),
    Pb14(pb14, PfC),
}

pub enum Sercom4Pad3 {
    Pa15(pa15, PfD),
    Pb11(pb11, PfD),
    Pb15(pb15, PfC),
}

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

pub enum Sercom5Pad0 {
    Pa22(pa22, PfD),
    Pb2(pb2, PfD),
    Pb16(pb16, PfC),
    Pb30(pb30, PfD),
}

pub enum Sercom5Pad1 {
    Pa23(pa23, PfD),
    Pb3(pb3, PfD),
    Pb17(pb17, PfC),
    Pb31(pb31, PfD),
}

pub enum Sercom5Pad2 {
    Pa24(pa24, PfD),
    Pb0(pb0, PfD),
    Pa20(pa20, PfC),
    Pb22(pb22, PfD),
}

pub enum Sercom5Pad3 {
    Pa25(pa25, PfD),
    Pb1(pb1, PfD),
    Pa21(pa21, PfC),
    Pb23(pb23, PfD),
}
);

const BUS_STATE_IDLE: u8 = 1;
const BUS_STATE_OWNED: u8 = 2;

const MASTER_ACT_READ: u8 = 2;
const MASTER_ACT_STOP: u8 = 3;

macro_rules! i2c {
    ([
        $($Type:ident: ($pad0:ident, $pad1:ident, $SERCOM:ident, $powermask:ident, $clock:ident),)+
    ]) => {
        $(
pub struct $Type {
    sda: $pad0,
    scl: $pad1,
    sercom: $SERCOM,
}

impl $Type {
    pub fn new<F: Into<Hertz>>(
        clocks: &Clocks,
        freq: F,
        sercom: $SERCOM,
        pm: &mut PM,
        sda: $pad0,
        scl: $pad1,
    ) -> Self {
        // Power up the peripheral bus clock.
        // safe because we're exclusively owning SERCOM
        pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

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
            let gclk: Hertz = clocks.$clock().unwrap().generator().into();
            let baud = (gclk.0 / (2 * freq.into().0) - 1) as u8;
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

    pub fn free(self) -> ($pad0, $pad1, $SERCOM) {
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
impl Write for $Type {
    type Error = I2CError;

    /// Sends bytes to slave with address `addr`
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let res = self.do_write(addr, bytes);
        self.cmd_stop();
        res
    }
}

impl Read for $Type {
    type Error = I2CError;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_read(addr, buffer);
        self.cmd_stop();
        res
    }
}

impl WriteRead for $Type {
    type Error = I2CError;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_write_read(addr, bytes, buffer);
        self.cmd_stop();
        res
    }
}
        )+
    };
}

i2c!([
    I2CMaster0: (Sercom0Pad0, Sercom0Pad1, SERCOM0, sercom0_, sercom0_core),
    I2CMaster1: (Sercom1Pad0, Sercom1Pad1, SERCOM1, sercom1_, sercom1_core),
    I2CMaster2: (Sercom2Pad0, Sercom2Pad1, SERCOM2, sercom2_, sercom2_core),
    I2CMaster3: (Sercom3Pad0, Sercom3Pad1, SERCOM3, sercom3_, sercom3_core),
    I2CMaster4: (Sercom4Pad0, Sercom4Pad1, SERCOM4, sercom4_, sercom4_core),
    I2CMaster5: (Sercom5Pad0, Sercom5Pad1, SERCOM5, sercom5_, sercom5_core),
]);

#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    AddressError,
    BusError,
    Timeout,
    Nack,
}
