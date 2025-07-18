//! QSPI
//!
//! This module provides an interface to the QSPI peripheral,
//! which is mainly used for communicating with external flash chips.
//!
//! The QSPI peripheral supports 2 different modes:
//! * Oneshot (Default) - In this mode, the QSPI peripheral simply
//!   sends and receives data to and from the flash chip. Quad-SPI
//!   communication is only used for reading and writing data to the
//!   flash chip. Other commands (Like erase, ID) are done in Single-SPI
//!   mode.
//! * XIP (eXecute In Place) - In this mode, the processor can execute
//!   code directly off the flash chip. Quad-SPI communication is forced
//!
//! This module does not come with an API for easily communicating with
//! flash chips, due to the varying differences in timings and commands.
//!
//! For an example of actually communicating (R/W) with a QSPI flash chip,
//! look at the pygamer BSP QSPI example.
//!
//! ## IMPORTANT
//! When using QSPI commands, the QSPI peripheral will STALL the CPU.
//! Therefore, you should select the highest SPI communication frequency
//! that the flash chip supports.
//!
//! Erasing a chip can take sometimes up to 1-2 minutes, in which time
//! the CPU is stalled. Bare this in mind when issuing long-running commands
//! with the watchdog enabled!
//!
//! ## Using the QSPI peripheral
//! ```
//! let pins = Pins::new(pac_peripherals.port);
//! let ahb_qspi = clocks.ahbs.qspi;
//! let apb_qspi = clocks.apbs.qspi;
//! let (qspi, gclk0) = QspiBuilder::new(
//!     pins.sck,
//!     pins.cs,
//!     pins.data0,
//!     pins.data1,
//!     pins.data2,
//!     pins.data3,
//! )
//! .with_freq(50_000_000)
//! .with_mode(atsamd_hal::qspi::QspiMode::_0)
//! .build(pac_peripherals.qspi, ahb_qspi, apb_qspi, gclk0)
//! .unwrap();
//! // QSPI is now in oneshot mode.
//! // ...
//! // Switch qspi to XIP mode if required
//! let qspi_xip = qspi.into_xip();
//! ```

use crate::{
    clock::v2::{
        Enabled, Source,
        ahb::AhbClk,
        apb::ApbClk,
        gclk::{EnabledGclk0, Gclk, Gclk0Id, Gclk0Io, GclkSourceId},
        types::Qspi as QspiClock,
    },
    gpio::{AlternateH, PA08, PA09, PA10, PA11, PB10, PB11, Pin},
    pac::{self, qspi::instrframe},
    typelevel::{Decrement, Increment, PrivateDecrement, PrivateIncrement},
};
use core::marker::PhantomData;

/// Qspi used for read/write of fixed-size octet buffers
pub struct OneShot;
/// Qspi is memory-mapped as read/execute
pub struct XIP;

pub struct Qspi<MODE> {
    qspi: pac::Qspi,
    _ahb: AhbClk<QspiClock>,
    _apb: ApbClk<QspiClock>,
    gclk0_freq: u32,
    _sck: Pin<PB10, AlternateH>,
    _cs: Pin<PB11, AlternateH>,
    _io0: Pin<PA08, AlternateH>,
    _io1: Pin<PA09, AlternateH>,
    _io2: Pin<PA10, AlternateH>,
    _io3: Pin<PA11, AlternateH>,
    _mode: PhantomData<MODE>,
}

/// QSPI signal operating modes
pub enum QspiMode {
    /// * Shift SCK Edge: Falling,
    /// * Capture SCK Edge: Falling
    /// * SCK inactive level: Low
    _0,
    /// * Shift SCK Edge: Rising,
    /// * Capture SCK Edge: Rising
    /// * SCK inactive level: Low
    _1,
    /// * Shift SCK Edge: Rising,
    /// * Capture SCK Edge: Rising
    /// * SCK inactive level: High
    _2,
    /// * Shift SCK Edge: Falling,
    /// * Capture SCK Edge: Falling
    /// * SCK inactive level: High
    _3,
}

pub struct QspiBuilder {
    sck: Pin<PB10, AlternateH>,
    cs: Pin<PB11, AlternateH>,
    io0: Pin<PA08, AlternateH>,
    io1: Pin<PA09, AlternateH>,
    io2: Pin<PA10, AlternateH>,
    io3: Pin<PA11, AlternateH>,
    freq: Option<u32>,
    mode: Option<QspiMode>,
    scramble_mode: Option<(u32, bool)>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QspiError {
    /// No SPI Frequency provided
    NoFreq,
    /// No QSPI signal mode provided
    NoMode,
    /// Target SPI frequency could
    /// not be achieved with the provided
    /// CPU speed
    SpiFreqNotValid,
    /// The command you selected cannot be performed by this function
    CommandFunctionMismatch,
}

/// # QSPI Configuration Builder
///
/// This structure contains methods that configures the QSPI module.
///
/// Setting frequency [`Self::with_freq`] and SPI mode [`Self::with_mode`]
/// are required to get QSPI running, without calling these, the [`Self::build`]
/// function will return a [`QspiError`]. Other configuration options are
/// optional, so are not required
impl QspiBuilder {
    pub fn new(
        sck: impl Into<Pin<PB10, AlternateH>>,
        cs: impl Into<Pin<PB11, AlternateH>>,
        io0: impl Into<Pin<PA08, AlternateH>>,
        io1: impl Into<Pin<PA09, AlternateH>>,
        io2: impl Into<Pin<PA10, AlternateH>>,
        io3: impl Into<Pin<PA11, AlternateH>>,
    ) -> Self {
        Self {
            sck: sck.into(),
            cs: cs.into(),
            io0: io0.into(),
            io1: io1.into(),
            io2: io2.into(),
            io3: io3.into(),
            freq: None,
            mode: None,
            scramble_mode: None,
        }
    }

    /// Sets the target frequency of the SPI communication
    pub fn with_freq(mut self, freq: u32) -> Self {
        self.freq = Some(freq);
        self
    }

    /// Sets the SPI operation mode
    pub fn with_mode(mut self, mode: QspiMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Enables the optional scramble feature of QSPI
    ///
    /// * Key - 32 bit key to use for the scramble
    /// * Random - Enable if the hardware based extra key should be
    /// applied - This means that a QSPI chip will appear scrambled,
    /// even to another processor. Disabling the random mode ensures
    /// that whilst the QSPI chip itself is scrambled, it is only using
    /// the user provided key - Thus allowing other processors with the
    /// same key to read the QSPI chip
    pub fn with_scramble(mut self, key: u32, random: bool) -> Self {
        self.scramble_mode = Some((key, random));
        self
    }

    /// Initialize the QSPI peripheral, and start communication
    /// in regular SPI mode
    pub fn build<I: GclkSourceId, S: Increment>(
        self,
        qspi: pac::Qspi,
        ahb: AhbClk<QspiClock>,
        apb: ApbClk<QspiClock>,
        gclk0: EnabledGclk0<I, S>,
    ) -> Result<(Qspi<OneShot>, EnabledGclk0<I, S::Inc>), QspiError> {
        Qspi::new(qspi, ahb, apb, gclk0, self)
    }
}

impl Qspi<OneShot> {
    pub(crate) fn new<I: GclkSourceId, S: Increment>(
        qspi: pac::Qspi,
        ahb: AhbClk<QspiClock>,
        apb: ApbClk<QspiClock>,
        gclk0: Enabled<Gclk<Gclk0Id, I>, S>,
        builder: QspiBuilder,
    ) -> Result<(Qspi<OneShot>, EnabledGclk0<I, S::Inc>), QspiError> {
        let targ_freq = builder.freq.ok_or(QspiError::NoFreq)?;
        let mode = builder.mode.ok_or(QspiError::NoMode)?;
        let gclk0_freq = gclk0.freq().to_Hz();
        // Ensure that the target SPI Freq can be achieved
        if gclk0_freq % targ_freq != 0 {
            return Err(QspiError::SpiFreqNotValid);
        }
        // Divider must be 0-254
        let div = gclk0_freq / targ_freq;
        if div > 254 || div == 0 {
            return Err(QspiError::SpiFreqNotValid);
        }
        qspi.ctrla().write(|w| w.swrst().set_bit());
        qspi.baud().write(|w| unsafe {
            w.baud().bits(div as u8 - 1);
            let cpol = match mode {
                QspiMode::_0 | QspiMode::_1 => false,
                QspiMode::_2 | QspiMode::_3 => true,
            };
            let cpha = match mode {
                QspiMode::_0 | QspiMode::_2 => false,
                QspiMode::_1 | QspiMode::_3 => true,
            };
            w.cpol().bit(cpol);
            w.cpha().bit(cpha)
        });

        qspi.ctrlb().write(|w| {
            w.mode().memory();
            w.csmode().noreload();
            w.csmode().lastxfer();
            w.datalen()._8bits()
        });

        // Enable scrambling if the user requested it
        if let Some((key, random_en)) = builder.scramble_mode {
            qspi.scrambctrl().write(|w| {
                w.enable().set_bit();
                w.randomdis().bit(!random_en)
            });
            qspi.scrambkey().write(|w| unsafe { w.key().bits(key) });
        }

        qspi.ctrla().modify(|_, w| w.enable().set_bit());

        Ok((
            Self {
                qspi,
                _ahb: ahb,
                _apb: apb,
                gclk0_freq,
                _sck: builder.sck,
                _cs: builder.cs,
                _io0: builder.io0,
                _io1: builder.io1,
                _io2: builder.io2,
                _io3: builder.io3,
                _mode: PhantomData,
            },
            gclk0.inc(),
        ))
    }

    /// Run a generic command that neither takes nor receives data
    pub fn run_command(&self, command: Command) -> Result<(), QspiError> {
        match command {
            //TODO verify this list of commands
            Command::WriteEnable
            | Command::WriteDisable
            | Command::Reset
            | Command::EnableReset => (),
            _ => return Err(QspiError::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_read_instruction(command, tfm, 0, &mut [], true);
        }
        Ok(())
    }

    /// Run one of the read commands
    pub fn read_command(&self, command: Command, response: &mut [u8]) -> Result<(), QspiError> {
        match command {
            //TODO verify this list of commands
            Command::Read
            | Command::QuadRead
            | Command::ReadId
            | Command::ReadStatus
            | Command::ReadStatus2 => (),
            _ => return Err(QspiError::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            data_enable: true,
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_read_instruction(command, tfm, 0, response, true);
        }
        Ok(())
    }

    /// Run one of the write commands
    pub fn write_command(&self, command: Command, data: &[u8]) -> Result<(), QspiError> {
        match command {
            //TODO verify this list of commands
            Command::PageProgram
            | Command::QuadPageProgram
            | Command::WriteStatus
            | Command::WriteStatus2 => (),
            _ => return Err(QspiError::CommandFunctionMismatch),
        }

        let tfm = TransferMode {
            data_enable: !data.is_empty(),
            instruction_enable: true,
            ..TransferMode::default()
        };
        unsafe {
            self.run_write_instruction(command, tfm, 0, data);
        }
        Ok(())
    }

    /// Run one of the erase commands
    pub fn erase_command(&self, command: Command, address: u32) -> Result<(), QspiError> {
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
                    self.run_read_instruction(command, tfm, 0, &mut [], true);
                }
            }
            _ => return Err(QspiError::CommandFunctionMismatch),
        }

        Ok(())
    }

    /// Quad Fast Read a sequential block of memory to buf
    /// Note: Hardcodes 8 dummy cycles
    pub fn read_memory(&mut self, addr: u32, buf: &mut [u8]) {
        let tfm = TransferMode {
            quad_width: true,
            address_enable: true,
            data_enable: true,
            instruction_enable: true,
            dummy_cycles: 8,
            ..TransferMode::default()
        };
        unsafe { self.run_read_instruction(Command::QuadRead, tfm, addr, buf, true) };
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

    /// Latches the peripheral in a read/execute state, so it can be used to
    /// read or execute directly from flash.
    ///
    /// Note: Hardcodes 8 dummy cycles.
    pub fn into_xip(self) -> Qspi<XIP> {
        let tfm = TransferMode {
            quad_width: true,
            address_enable: true,
            data_enable: true,
            instruction_enable: true,
            dummy_cycles: 8,
            ..TransferMode::default()
        };
        unsafe {
            self.run_read_instruction(Command::QuadRead, tfm, 0, &mut [], false);
        }

        Qspi::<XIP> {
            qspi: self.qspi,
            _ahb: self._ahb,
            _apb: self._apb,
            gclk0_freq: self.gclk0_freq,
            _sck: self._sck,
            _cs: self._cs,
            _io0: self._io0,
            _io1: self._io1,
            _io2: self._io2,
            _io3: self._io3,
            _mode: PhantomData,
        }
    }

    /// Return the consumed pins and the Qspi peripheral
    ///
    /// Order: `(qspi, apb, ahb, gclk0, sck, cs, io0, io1, io2, io3)`
    #[allow(clippy::type_complexity)]
    pub fn free<I: Gclk0Io, S: Source + Decrement>(
        self,
        gclk0: EnabledGclk0<I, S>,
    ) -> (
        pac::Qspi,
        AhbClk<QspiClock>,
        ApbClk<QspiClock>,
        EnabledGclk0<I, S::Dec>,
        Pin<PB10, AlternateH>,
        Pin<PB11, AlternateH>,
        Pin<PA08, AlternateH>,
        Pin<PA09, AlternateH>,
        Pin<PA10, AlternateH>,
        Pin<PA11, AlternateH>,
    ) {
        (
            self.qspi,
            self._ahb,
            self._apb,
            gclk0.dec(),
            self._sck,
            self._cs,
            self._io0,
            self._io1,
            self._io2,
            self._io3,
        )
    }
}

/// Operations available in XIP mode
impl Qspi<XIP> {
    /// Latches the peripheral in a read/execute state, so it can be used to
    /// read or execute directly from flash.
    pub fn into_oneshot(self) -> Qspi<OneShot> {
        unsafe { self.finalize() };

        Qspi::<OneShot> {
            qspi: self.qspi,
            _ahb: self._ahb,
            _apb: self._apb,
            gclk0_freq: self.gclk0_freq,
            _sck: self._sck,
            _cs: self._cs,
            _io0: self._io0,
            _io1: self._io1,
            _io2: self._io2,
            _io3: self._io3,
            _mode: PhantomData,
        }
    }
}

// (Mostly internal) methods available in any mode.
impl<MODE> Qspi<MODE> {
    unsafe fn finalize(&self) {
        self.qspi.ctrla().write(|w| {
            w.enable().set_bit();
            w.lastxfer().set_bit()
        });

        while self.qspi.intflag().read().instrend().bit_is_clear() {}
        self.qspi.intflag().write(|w| w.instrend().set_bit());
        while self.qspi.intflag().read().csrise().bit_is_clear() {}
        self.qspi.intflag().write(|w| w.csrise().set_bit());
    }

    unsafe fn run_write_instruction(
        &self,
        command: Command,
        tfm: TransferMode,
        addr: u32,
        buf: &[u8],
    ) {
        unsafe {
            if command == Command::EraseSector || command == Command::EraseBlock {
                self.qspi.instraddr().write(|w| w.addr().bits(addr));
            }
            self.qspi
                .instrctrl()
                .modify(|_, w| w.instr().bits(command.bits()));
            self.qspi.instrframe().write(|w| {
                tfm.instrframe(
                    w,
                    if command == Command::QuadPageProgram {
                        instrframe::Tfrtypeselect::Writememory
                    } else {
                        instrframe::Tfrtypeselect::Write
                    },
                )
            });
            self.qspi.instrframe().read().bits();

            if !buf.is_empty() {
                core::ptr::copy(buf.as_ptr(), (QSPI_AHB + addr) as *mut u8, buf.len());
            }

            self.finalize();
        }
    }

    unsafe fn run_read_instruction(
        &self,
        command: Command,
        tfm: TransferMode,
        addr: u32,
        buf: &mut [u8],
        finalize: bool,
    ) {
        unsafe {
            self.qspi
                .instrctrl()
                .modify(|_, w| w.instr().bits(command.bits()));
            self.qspi.instrframe().write(|w| {
                tfm.instrframe(
                    w,
                    if command == Command::QuadRead {
                        instrframe::Tfrtypeselect::Readmemory
                    } else {
                        instrframe::Tfrtypeselect::Read
                    },
                )
            });
            self.qspi.instrframe().read().bits();

            if !buf.is_empty() {
                core::ptr::copy((QSPI_AHB + addr) as *mut u8, buf.as_mut_ptr(), buf.len());
            }

            if finalize {
                self.finalize();
            }
        }
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
        tfrtype: instrframe::Tfrtypeselect,
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
            unsafe {
                instrframe.dummylen().bits(self.dummy_cycles);
            }
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
