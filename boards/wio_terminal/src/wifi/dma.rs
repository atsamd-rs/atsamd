//! DMA-driven UART transport: the receiver runs forever in a circular DMA
//! transfer into a static ring, and the transmitter is a blocking DMA write.
//! No UART interrupts are used.

use atsamd_hal::dmac::{
    BufferPair, Busy, Ch0, Ch1, ChId, Channel, InterruptFlags, Ready, Transfer, TriggerAction,
};
use atsamd_hal::embedded_io::Write;
use atsamd_hal::pac;
use atsamd_hal::sercom::uart::{Config, RxDuplex, TxDuplex, Uart};
use atsamd_hal::sercom::{Sercom, Sercom0};
use atsamd_hal::typelevel::NoneT;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::NVIC;

use super::{TransportError, WifiUart, WifiUartPads, INTER_BYTE_BUDGET};

pub const RX_RING_LEN: usize = 2048;
pub type WifiRxChannel = Channel<Ch0, Ready>;
pub type WifiTxChannel = Channel<Ch1, Ready>;
type RxHalf = Uart<Config<WifiUartPads>, RxDuplex>;
type TxHalf = Uart<Config<WifiUartPads>, TxDuplex, NoneT, WifiTxChannel>;
type RxTransfer = Transfer<Channel<Ch0, Busy>, BufferPair<RxHalf, &'static mut [u8; RX_RING_LEN]>>;
const RX_CHANNEL: usize = <Ch0 as ChId>::USIZE;

/// Makes every completed block of `channel`'s first descriptor set the
/// transfer-complete flag (`BTCTRL.BLOCKACT = INT`). Call after the transfer
/// is built and before it is started.
///
/// The HAL leaves `BLOCKACT = NOACT`. With that, `CHINTFLAG.TCMPL` is set only
/// when a whole transaction ends, which a circular transfer never does, so
/// ring wraps would go uncounted (observed on hardware: no flag at a wrap,
/// with or without the interrupt enabled). `INT` sets the flag at the end of
/// every block; it raises an interrupt only if the channel's TCMPL interrupt
/// is enabled, and it does not stop the channel while the descriptor links to
/// itself.
pub(super) fn set_block_flag(channel: usize) {
    let dmac = unsafe { pac::Dmac::steal() };
    // Descriptor section: 16 bytes per channel, BTCTRL is the first half-word.
    let base = dmac.baseaddr().read().bits() as *mut u16;
    unsafe {
        let btctrl = base.add(channel * 8);
        let value = core::ptr::read_volatile(btctrl);
        // BLOCKACT is bits 4:3; INT = 1.
        core::ptr::write_volatile(btctrl, (value & !(0b11 << 3)) | (1 << 3));
    }
}

/// Number of times the receive ring has wrapped, counted by the `DMAC_0`
/// interrupt (RX channel transfer complete).
static RX_WRAPS: AtomicU32 = AtomicU32::new(0);

/// Called from the `DMAC_0` interrupt (see `wifi_singleton!`).
#[doc(hidden)]
pub fn _handle_dma_rx_wrap() {
    let dmac = unsafe { pac::Dmac::steal() };
    let flags = dmac.channel(RX_CHANNEL).chintflag();
    if flags.read().tcmpl().bit_is_set() {
        flags.write(|w| w.tcmpl().set_bit());
        RX_WRAPS.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct Transport {
    _rx: RxTransfer,
    tx: TxHalf,
    ring: *const u8,
    /// Bytes consumed since start, modulo 2^32 (`RX_RING_LEN` divides 2^32,
    /// so the ring index is `read_total % RX_RING_LEN`).
    read_total: u32,
}

// The raw pointer refers to a `'static` buffer that is only written by DMA.
unsafe impl Send for Transport {}

/// Index in the ring where the DMA will write the next byte.
///
/// The DMAC only writes a channel's descriptor back to memory when another
/// channel takes over, so the write-back BTCNT can be stale while this channel
/// was the last one to run. `ACTIVE` holds the live count for the most recent
/// channel even after `ABUSY` clears, so it is trusted whenever its ID is the
/// RX channel. The two registers are not read atomically, so retry until
/// `ACTIVE` is unchanged around the read.
fn write_index() -> usize {
    let dmac = unsafe { pac::Dmac::steal() };
    let remaining = loop {
        let before = dmac.active().read();
        let remaining = if before.id().bits() as usize == RX_CHANNEL {
            before.btcnt().bits()
        } else {
            let writeback = dmac.wrbaddr().read().bits() as *const u8;
            unsafe { core::ptr::read_volatile(writeback.add(RX_CHANNEL * 16 + 2) as *const u16) }
        };
        if dmac.active().read().bits() == before.bits() {
            break remaining as usize;
        }
    };
    // Before the first byte the write-back entry is still zero.
    if remaining == 0 || remaining > RX_RING_LEN {
        0
    } else {
        RX_RING_LEN - remaining
    }
}

/// Bytes written by the DMA since start, modulo 2^32.
///
/// Combines the wrap count with the ring index. A wrap can happen between the
/// reads, and the interrupt that counts it may not have run yet, so the
/// pending transfer-complete flag counts as a wrap too, and the read is
/// retried until the count and the flag are unchanged around it.
fn written_total() -> u32 {
    let dmac = unsafe { pac::Dmac::steal() };
    let flags = dmac.channel(RX_CHANNEL).chintflag();
    loop {
        let wraps = RX_WRAPS.load(Ordering::Relaxed);
        let pending = flags.read().tcmpl().bit_is_set();
        let index = write_index();
        if flags.read().tcmpl().bit_is_set() == pending && RX_WRAPS.load(Ordering::Relaxed) == wraps
        {
            return (wraps + pending as u32)
                .wrapping_mul(RX_RING_LEN as u32)
                .wrapping_add(index as u32);
        }
    }
}

/// Returns whether the UART receiver reported an overflow or framing error,
/// and clears the flags.
fn take_rx_error() -> bool {
    let usart = unsafe { pac::Sercom0::steal() };
    let status = usart.usart_int().status().read();
    let failed = status.bufovf().bit_is_set() || status.ferr().bit_is_set();
    if failed {
        usart
            .usart_int()
            .status()
            .write(|w| w.bufovf().set_bit().ferr().set_bit());
    }
    failed
}

impl Transport {
    pub fn new(
        uart: WifiUart,
        mut rx_channel: WifiRxChannel,
        tx_channel: WifiTxChannel,
        ring: &'static mut [u8; RX_RING_LEN],
    ) -> Self {
        let (rx, tx) = uart.split();
        let tx: TxHalf = tx.with_tx_channel(tx_channel);
        let ring_ptr: *const u8 = ring.as_ptr();
        // Count ring wraps (see `written_total`); the NVIC line is unmasked in
        // `enable`.
        rx_channel.enable_interrupts(InterruptFlags::new().with_tcmpl(true));
        let rx = Transfer::new(rx_channel, rx, ring, true).unwrap();
        set_block_flag(RX_CHANNEL);
        let rx: RxTransfer = rx.begin(<Sercom0 as Sercom>::DMA_RX_TRIGGER, TriggerAction::Burst);

        Transport {
            _rx: rx,
            tx,
            ring: ring_ptr,
            read_total: 0,
        }
    }

    /// Unmasks the `DMAC_0` interrupt that counts receive-ring wraps.
    pub fn enable(&mut self, nvic: &mut NVIC) {
        unsafe {
            nvic.set_priority(pac::Interrupt::DMAC_0, 1);
            NVIC::unmask(pac::Interrupt::DMAC_0);
        }
    }

    pub fn write(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        self.tx.write_all(frame).map_err(|_| TransportError::Tx)
    }

    /// Blocks until `buffer` has been filled from the ring. Fails if the
    /// UART reports an overflow or framing error, if the DMA has overwritten
    /// bytes not yet read (more than `RX_RING_LEN` unread), or if the first
    /// byte does not arrive within `first_byte_budget` polls, or a later one
    /// within [`INTER_BYTE_BUDGET`].
    pub fn read_exact(
        &mut self,
        buffer: &mut [u8],
        first_byte_budget: u32,
    ) -> Result<(), TransportError> {
        let mut budget = first_byte_budget;
        for out in buffer.iter_mut() {
            loop {
                // Signed: a wrap caught at the exact moment the block completes
                // can read one ring length behind for a poll; that means
                // "nothing new yet", not an overrun.
                let unread = written_total().wrapping_sub(self.read_total) as i32;
                if unread > RX_RING_LEN as i32 {
                    return Err(TransportError::Rx);
                }
                if unread > 0 {
                    break;
                }
                if take_rx_error() {
                    return Err(TransportError::Rx);
                }
                budget -= 1;
                if budget == 0 {
                    return Err(TransportError::Timeout);
                }
            }
            let index = self.read_total as usize % RX_RING_LEN;
            *out = unsafe { core::ptr::read_volatile(self.ring.add(index)) };
            self.read_total = self.read_total.wrapping_add(1);
            budget = INTER_BYTE_BUDGET;
        }
        Ok(())
    }

    /// Drops everything received so far, to resynchronise after an error.
    pub fn discard_pending(&mut self) {
        self.read_total = written_total();
    }
}
