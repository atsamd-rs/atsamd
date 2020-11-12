use crate::{
    gpio::{Floating, Input, Pa10, Pa11, Pa8, Pa9, Pb10, Pb11, PfH, Port},
    target_device::qspi::instrframe,
    target_device::{MCLK, QSPI},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// The command you selected cannot be performed by this function
    CommandFunctionMismatch,
}

pub struct Qspi {
    qspi: QSPI,
    _sck: Pb10<PfH>,
    _cs: Pb11<PfH>,
    _io0: Pa8<PfH>,
    _io1: Pa9<PfH>,
    _io2: Pa10<PfH>,
    _io3: Pa11<PfH>,
}

impl Qspi {
    /// Enable the clocks for the qspi peripheral in single data rate mode
    /// assuming 120mhz system clock, for 4mhz spi mode 0 operation.
    pub fn new(
        mclk: &mut MCLK,
        port: &mut Port,
        qspi: QSPI,
        _sck: Pb10<Input<Floating>>,
        _cs: Pb11<Input<Floating>>,
        _io0: Pa8<Input<Floating>>,
        _io1: Pa9<Input<Floating>>,
        _io2: Pa10<Input<Floating>>,
        _io3: Pa11<Input<Floating>>,
    ) -> Qspi {
        mclk.apbcmask.modify(|_, w| w.qspi_().set_bit());
        // Enable the clocks for the qspi peripheral in single data rate mode.
        mclk.ahbmask.modify(|_, w| {
            w.qspi_().set_bit();
            w.qspi_2x_().clear_bit()
        });

        let _sck = _sck.into_function_h(port);
        let _cs = _cs.into_function_h(port);
        let _io0 = _io0.into_function_h(port);
        let _io1 = _io1.into_function_h(port);
        let _io2 = _io2.into_function_h(port);
        let _io3 = _io3.into_function_h(port);

        qspi.ctrla.write(|w| w.swrst().set_bit());
        qspi.baud.write(|w| unsafe {
            // TODO get system clock value instead of hardcoding
            //(120_000_000u32 / 4_000_000u32) = 30 = BAUD + 1
            // BAUD = 29
            w.baud().bits(29); // 4Mhz
                               // SPI MODE 0
            w.cpol().clear_bit();
            w.cpha().clear_bit()
        });

        qspi.ctrlb.write(|w| {
            w.mode().memory();
            w.csmode().noreload();
            w.csmode().lastxfer();
            w.datalen()._8bits()
        });

        qspi.ctrla.modify(|_, w| w.enable().set_bit());

        Qspi {
            qspi,
            _sck,
            _cs,
            _io0,
            _io1,
            _io2,
            _io3,
        }
    }

    unsafe fn run_write_instruction(
        &self,
        command: Command,
        tfm: TransferMode,
        addr: u32,
        buf: &[u8],
    ) {
        if command == Command::EraseSector || command == Command::EraseBlock {
            self.qspi.instraddr.write(|w| w.addr().bits(addr));
        }
        self.qspi
            .instrctrl
            .modify(|_, w| w.instr().bits(command.bits()));
        self.qspi.instrframe.write(|w| {
            tfm.instrframe(
                w,
                if command == Command::QuadPageProgram {
                    instrframe::TFRTYPE_A::WRITEMEMORY
                } else {
                    instrframe::TFRTYPE_A::WRITE
                },
            )
        });
        self.qspi.instrframe.read().bits();

        if buf.len() > 0 {
            core::ptr::copy(buf.as_ptr(), (QSPI_AHB + addr) as *mut u8, buf.len());
        }

        self.qspi.ctrla.write(|w| {
            w.enable().set_bit();
            w.lastxfer().set_bit()
        });

        while self.qspi.intflag.read().instrend().bit_is_clear() {}
        self.qspi.intflag.write(|w| w.instrend().set_bit());
        while self.qspi.intflag.read().csrise().bit_is_clear() {}
        self.qspi.intflag.write(|w| w.csrise().set_bit());
    }

    unsafe fn run_read_instruction(
        &self,
        command: Command,
        tfm: TransferMode,
        addr: u32,
        buf: &mut [u8],
    ) {
        self.qspi
            .instrctrl
            .modify(|_, w| w.instr().bits(command.bits()));
        self.qspi.instrframe.write(|w| {
            tfm.instrframe(
                w,
                if command == Command::QuadRead {
                    instrframe::TFRTYPE_A::READMEMORY
                } else {
                    instrframe::TFRTYPE_A::READ
                },
            )
        });
        self.qspi.instrframe.read().bits();

        if buf.len() > 0 {
            core::ptr::copy((QSPI_AHB + addr) as *mut u8, buf.as_mut_ptr(), buf.len());
        }
        self.qspi.ctrla.write(|w| {
            w.enable().set_bit();
            w.lastxfer().set_bit()
        });

        while self.qspi.intflag.read().instrend().bit_is_clear() {}
        self.qspi.intflag.write(|w| w.instrend().set_bit());
        while self.qspi.intflag.read().csrise().bit_is_clear() {}
        self.qspi.intflag.write(|w| w.csrise().set_bit());
    }

    /// Run a generic command that neither takes nor receives data
    pub fn run_command(&self, command: Command) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::WriteEnable
            | Command::WriteDisable
            | Command::Reset
            | Command::EnableReset => (),
            _ => return Err(Error::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_read_instruction(command, tfm, 0, &mut []);
        }
        Ok(())
    }

    /// Run one of the read commands
    pub fn read_command(&self, command: Command, response: &mut [u8]) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::Read
            | Command::QuadRead
            | Command::ReadId
            | Command::ReadStatus
            | Command::ReadStatus2 => (),
            _ => return Err(Error::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            data_enable: true,
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_read_instruction(command, tfm, 0, response);
        }
        Ok(())
    }

    /// Run one of the write commands
    pub fn write_command(&self, command: Command, data: &[u8]) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::PageProgram
            | Command::QuadPageProgram
            | Command::WriteStatus
            | Command::WriteStatus2 => (),
            _ => return Err(Error::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            data_enable: data.len() > 0,
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_write_instruction(command, tfm, 0, data);
        }
        Ok(())
    }

    /// Run one of the erase commands
    pub fn erase_command(&self, command: Command, address: u32) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::EraseSector | Command::EraseBlock => {
                let tfm = TransferMode {
                    address_enable: true,
                    instruction_enable: true,
                    ..TransferMode::default()
                };
                unsafe {
                    self.run_write_instruction(command, tfm, address, &[]);
                }
            }
            Command::EraseChip => {
                let tfm = TransferMode {
                    instruction_enable: true,
                    ..TransferMode::default()
                };
                unsafe {
                    self.run_read_instruction(command, tfm, 0, &mut []);
                }
            }
            _ => return Err(Error::CommandFunctionMismatch),
        }

        Ok(())
    }

    /// Quad Fast Read a sequential block of memory to buf
    /// Note hardcodes 8 dummy cycles
    pub fn read_memory(&mut self, addr: u32, buf: &mut [u8]) {
        let tfm = TransferMode {
            quad_width: true,
            address_enable: true,
            data_enable: true,
            instruction_enable: true,
            dummy_cycles: 8,
            ..TransferMode::default()
        };
        unsafe { self.run_read_instruction(Command::QuadRead, tfm, addr, buf) };
    }

    /// Page Program a sequential block of memory to addr.
    ///
    /// Note more than page size bytes are sent to the device, some bytes will
    /// be discarded. Check your device for specific handling.
    pub fn write_memory(&mut self, addr: u32, buf: &[u8]) {
        let tfm = TransferMode {
            quad_width: true,
            address_enable: true,
            data_enable: true,
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe { self.run_write_instruction(Command::QuadPageProgram, tfm, addr, buf) };
    }

    /// Set the clock divider, relative to the main clock
    ///
    /// This fn safely subtracts 1 from your input value as the underlying fn is
    /// SCK Baud = MCKL / (value + 1)
    ///
    /// ex if MCLK is 120mhz
    /// value  0 is reduced to  0 results in 120mhz clock
    /// value  1 is reduced to  0 results in 120mhz clock
    /// value  2 is reduced to  1 results in  60mhz clock
    pub fn set_clk_divider(&mut self, value: u8) {
        // The baud register is divisor - 1
        self.qspi
            .baud
            .write(|w| unsafe { w.baud().bits(value.saturating_sub(1)) });
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct TransferMode {
    quad_width: bool,
    data_enable: bool,
    opcode_enable: bool,
    address_enable: bool,
    instruction_enable: bool,
    dummy_cycles: u8,
}

impl TransferMode {
    unsafe fn instrframe(
        self,
        instrframe: &mut instrframe::W,
        tfrtype: instrframe::TFRTYPE_A,
    ) -> &mut instrframe::W {
        if self.quad_width {
            instrframe.width().quad_output();
        } else {
            instrframe.width().single_bit_spi();
        }

        if self.data_enable {
            instrframe.dataen().set_bit();
        }
        if self.opcode_enable {
            instrframe.dataen().set_bit();
        }
        if self.address_enable {
            instrframe.addren().set_bit();
        }
        if self.instruction_enable {
            instrframe.instren().set_bit();
        }

        if self.dummy_cycles > 0 {
            instrframe.dummylen().bits(self.dummy_cycles);
        }
        instrframe.addrlen()._24bits();
        instrframe.optcodeen().clear_bit();
        instrframe.tfrtype().variant(tfrtype);
        instrframe
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    Read = 0x03,
    QuadRead = 0x6B,
    ReadId = 0x9F,
    PageProgram = 0x02,
    QuadPageProgram = 0x32,
    ReadStatus = 0x05,
    ReadStatus2 = 0x35,
    WriteStatus = 0x01,
    WriteStatus2 = 0x31,
    EnableReset = 0x66,
    Reset = 0x99,
    WriteEnable = 0x06,
    WriteDisable = 0x04,
    EraseSector = 0x20,
    EraseBlock = 0xD8,
    EraseChip = 0xC7,
}

impl Command {
    fn bits(self) -> u8 {
        self as u8
    }
}

const QSPI_AHB: u32 = 0x04000000;
