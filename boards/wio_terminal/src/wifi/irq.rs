//! Interrupt-driven UART transport: one interrupt per byte in each direction,
//! with `bbqueue` buffers between the interrupt handlers and the RPC code.

use atsamd_hal::{pac::interrupt, prelude::*, sercom::uart};
use bbqueue::{BBBuffer, Consumer, Producer};
use cortex_m::peripheral::NVIC;
use nb;

use core::sync::atomic::{AtomicBool, Ordering};

use super::{TransportError, WifiUart, INTER_BYTE_BUDGET};

/// Set by the RX interrupt when a byte was lost (UART error, or the queue was
/// full); taken by `read_exact`, which reports it as `TransportError::Rx`.
static RX_ERROR: AtomicBool = AtomicBool::new(false);

pub struct Transport {
    uart: WifiUart,

    rx_buff_isr: Producer<'static, 512>,
    rx_buff_input: Consumer<'static, 512>,
    tx_buff_isr: Consumer<'static, 128>,
    tx_buff_input: Producer<'static, 128>,
}

impl Transport {
    pub fn new(
        uart: WifiUart,
        rx_buff: &'static BBBuffer<512>,
        tx_buff: &'static BBBuffer<128>,
    ) -> Self {
        let (rx_buff_isr, rx_buff_input) = rx_buff.try_split().unwrap();
        let (tx_buff_input, tx_buff_isr) = tx_buff.try_split().unwrap();

        Transport {
            uart,
            rx_buff_isr,
            rx_buff_input,
            tx_buff_isr,
            tx_buff_input,
        }
    }

    /// Turns on the UART interrupts.
    pub fn enable(&mut self, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(interrupt::SERCOM0_0, 1);
            NVIC::unmask(interrupt::SERCOM0_0);
            nvic.set_priority(interrupt::SERCOM0_2, 1);
            NVIC::unmask(interrupt::SERCOM0_2);
        }

        self.uart.enable_interrupts(uart::Flags::RXC);
    }

    /// Called from ISR: Handles the signal that the UART has recieved
    /// a byte that needs to be read.
    pub fn handle_rx(&mut self) {
        #[cfg(feature = "wifi-stats")]
        let _stats = super::stats::IsrGuard::enter();
        match self.uart.read() {
            Ok(b) => {
                if let Ok(mut wgr) = self.rx_buff_isr.grant_exact(1) {
                    wgr[0] = b;
                    wgr.commit(1);
                } else {
                    // Queue full: the byte is lost.
                    RX_ERROR.store(true, Ordering::Relaxed);
                }
            }
            Err(nb::Error::Other(_e)) => {
                // Overflow, framing or parity error. `read` leaves the error
                // flags and the received data in place, so without this the
                // next `read` fails again and the interrupt never stops.
                self.uart.flush_rx_buffer();
                RX_ERROR.store(true, Ordering::Relaxed);
            }
            // Skip WouldBlock
            Err(nb::Error::WouldBlock) => (),
        };
    }

    /// Called from ISR: Handles the signal that the outgoing UART buffer
    /// has room for the next byte.
    pub fn handle_data_empty(&mut self) {
        #[cfg(feature = "wifi-stats")]
        let _stats = super::stats::IsrGuard::enter();
        if let Ok(rgr) = self.tx_buff_isr.read() {
            let buf = rgr.buf();
            self.uart.write(buf[0]).ok();
            rgr.release(1);
        } else {
            self.uart.disable_interrupts(uart::Flags::DRE);
        }
    }

    /// Queues `frame` for transmission.
    pub fn write(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        for b in frame {
            if let Ok(mut wgr) = self.tx_buff_input.grant_exact(1) {
                wgr[0] = *b;
                wgr.commit(1);
            }
            self.uart.enable_interrupts(uart::Flags::DRE);
        }
        Ok(())
    }

    /// Drops everything received so far, to resynchronise after an error.
    pub fn discard_pending(&mut self) {
        while let Ok(r) = self.rx_buff_input.read() {
            let n = r.buf().len();
            r.release(n);
        }
    }

    /// Blocks until `buffer` has been filled from the receive queue. Fails if
    /// a received byte was lost, or if the first byte does not arrive within
    /// `first_byte_budget` polls, or a later one within [`INTER_BYTE_BUDGET`].
    pub fn read_exact(
        &mut self,
        mut buffer: &mut [u8],
        first_byte_budget: u32,
    ) -> Result<(), TransportError> {
        let mut budget = first_byte_budget;
        while buffer.len() > 0 {
            if RX_ERROR.swap(false, Ordering::Relaxed) {
                return Err(TransportError::Rx);
            }
            let r = match self.rx_buff_input.read() {
                Ok(r) => r,
                Err(_) => {
                    budget -= 1;
                    if budget == 0 {
                        return Err(TransportError::Timeout);
                    }
                    continue;
                }
            };
            budget = INTER_BYTE_BUDGET;
            let b = r.buf();
            let copy_amt = if b.len() < buffer.len() {
                b.len()
            } else {
                buffer.len()
            };

            for (i, b) in b[..copy_amt].iter().enumerate() {
                buffer[i] = *b;
            }
            buffer = &mut buffer[copy_amt..];

            r.release(copy_amt);
        }
        Ok(())
    }
}
