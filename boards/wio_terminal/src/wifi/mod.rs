use atsamd_hal::{
    clock::GenericClockController,
    delay::Delay,
    ehal::digital::OutputPin,
    pac::Mclk,
    prelude::*,
    sercom::{uart, IoSet2, Sercom0},
};
#[cfg(not(feature = "wifi-dma"))]
use bbqueue::BBBuffer;

use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::NVIC;

#[cfg(not(feature = "wifi-dma"))]
mod irq;
#[cfg(not(feature = "wifi-dma"))]
use irq::Transport;
#[cfg(feature = "wifi-dma")]
mod dma;
#[cfg(feature = "wifi-dma")]
use dma::Transport;
#[cfg(feature = "wifi-dma")]
pub use dma::{_handle_dma_rx_wrap, WifiRxChannel, WifiTxChannel, RX_RING_LEN};

pub use erpc::rpcs;
use seeed_erpc as erpc;

use super::pins::aliases::*;

use crate::WIFI_UART_BAUD;

/// The set of pins which are connected to the RTL8720 in some way
pub struct WifiPins {
    pub pwr: WifiPwrReset,
    pub rxd: WifiRxdReset,
    pub txd: WifiTxdReset,
    pub mosi: WifiTxReset,
    pub clk: WifiClkReset,
    pub miso: WifiRxReset,
    pub cs: WifiCsReset,
    pub ready: WifiReadyReset,
    pub dir: WifiDirReset,
}

/// eRPC-based protocol to the RTL8720 chip
pub struct Wifi {
    _pwr: WifiPwr,
    transport: Transport,
    sequence: u32,
}

/// Errors reported by a wifi transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    /// The UART reported an overflow or framing error, or received bytes had
    /// to be dropped because the receive buffer was full.
    Rx,
    /// The request could not be sent.
    Tx,
    /// Nothing arrived within the read budget (see [`REPLY_WAIT_BUDGET`] and
    /// [`INTER_BYTE_BUDGET`]).
    Timeout,
}

// Read timeouts, counted in polling iterations because the transport owns no
// timer. One iteration is roughly 50 CPU cycles (estimate, not calibrated),
// so at 120 MHz:

/// Wait for the first byte of a reply: about 10 s. Long enough for the
/// slowest RPC (`WifiConnect` blocks on the RTL8720 until it has associated),
/// short enough that a silent RTL8720 cannot hang the caller forever.
pub const REPLY_WAIT_BUDGET: u32 = 24_000_000;

/// Wait between bytes once a reply has started: about 10 ms. The RTL8720
/// sends a frame back to back (16 us per byte), so a longer gap means bytes
/// were lost and the frame will never complete.
pub const INTER_BYTE_BUDGET: u32 = 24_000;

/// UART pads for the labelled RX & TX pins
pub type WifiUartPads = uart::Pads<Sercom0, IoSet2, WifiRx, WifiTx>;

/// UART device for the labelled RX & TX pins
pub type WifiUart = uart::Uart<uart::Config<WifiUartPads>, uart::Duplex>;

impl Wifi {
    /// Builds the UART and power-cycles the RTL8720.
    fn start_uart(
        pins: WifiPins,
        sercom0: Sercom0,
        clocks: &mut GenericClockController,
        mclk: &mut Mclk,
        delay: &mut Delay,
    ) -> (WifiPwr, WifiUart) {
        let gclk0 = clocks.gclk0();

        let pads = uart::Pads::default().rx(pins.miso).tx(pins.mosi);
        let uart = uart::Config::new(
            mclk,
            sercom0,
            pads,
            clocks.sercom0_core(&gclk0).unwrap().freq(),
        )
        .baud(
            WIFI_UART_BAUD.Hz(),
            uart::BaudMode::Fractional(uart::Oversampling::Bits16),
        );

        delay.delay_ms(10u8);

        // Reset the RTL8720 MCU.
        let mut pwr: WifiPwr = pins.pwr.into();
        OutputPin::set_low(&mut pwr).ok();
        delay.delay_ms(100u8);
        OutputPin::set_high(&mut pwr).ok();
        delay.delay_ms(200u8);

        (pwr, uart.enable())
    }

    #[cfg(not(feature = "wifi-dma"))]
    pub fn init(
        pins: WifiPins,
        sercom0: Sercom0,
        clocks: &mut GenericClockController,
        mclk: &mut Mclk,
        delay: &mut Delay,
        rx_buff: &'static BBBuffer<512>,
        tx_buff: &'static BBBuffer<128>,
    ) -> Wifi {
        let (pwr, uart) = Self::start_uart(pins, sercom0, clocks, mclk, delay);
        Wifi {
            _pwr: pwr,
            transport: Transport::new(uart, rx_buff, tx_buff),
            sequence: 0,
        }
    }

    #[cfg(feature = "wifi-dma")]
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        pins: WifiPins,
        sercom0: Sercom0,
        clocks: &mut GenericClockController,
        mclk: &mut Mclk,
        delay: &mut Delay,
        rx_channel: WifiRxChannel,
        tx_channel: WifiTxChannel,
        rx_ring: &'static mut [u8; RX_RING_LEN],
    ) -> Wifi {
        let (pwr, uart) = Self::start_uart(pins, sercom0, clocks, mclk, delay);
        Wifi {
            _pwr: pwr,
            transport: Transport::new(uart, rx_channel, tx_channel, rx_ring),
            sequence: 0,
        }
    }

    /// Turns on internal interrupts. Call this after you have finished
    /// initializing the rest of your peripherals but before you start
    /// issuing RPCs against the wifi chip.
    pub fn enable(&mut self, _cs: &CriticalSection, nvic: &mut NVIC) {
        self.transport.enable(nvic);
    }

    /// Convenience function to connection an access point with the given
    /// network name and security parameters, and request an IP via DHCP.
    pub fn connect_to_ap<S: TryInto<heapless::String<64>>, P: TryInto<heapless::String<64>>>(
        &mut self,
        delay: &mut Delay,
        ssid: S,
        pw: P,
        security: erpc::Security,
    ) -> Result<erpc::IPInfo, erpc::Err<()>> {
        self.blocking_rpc(rpcs::AdapterInit {})?;
        self.blocking_rpc(rpcs::DHCPClientStop {
            interface: erpc::L3Interface::Station,
        })?;
        self.blocking_rpc(rpcs::WifiOff {})?;

        delay.delay_ms(35u8);

        self.blocking_rpc(rpcs::WifiOn {
            mode: erpc::WifiMode::Station,
        })?;

        self.blocking_rpc(rpcs::WifiConnect {
            ssid: ssid.try_into().map_err(|_| erpc::Err::Unknown)?,
            password: pw.try_into().map_err(|_| erpc::Err::Unknown)?,
            security: security,
            semaphore: 0,
        })?;

        self.blocking_rpc(rpcs::DHCPClientStart {
            interface: erpc::L3Interface::Station,
        })?;

        delay.delay_ms(25u8);
        self.blocking_rpc(rpcs::GetIPInfo {
            interface: erpc::L3Interface::Station,
        })
        .map_err(|_| erpc::Err::RPCErr(()))
    }

    /// Called from ISR: Handles the signal that the UART has recieved
    /// a byte that needs to be read.
    #[cfg(not(feature = "wifi-dma"))]
    pub fn _handle_rx(&mut self) {
        self.transport.handle_rx();
    }

    /// Called from ISR: Handles the signal that the outgoing UART buffer
    /// has room for the next byte.
    #[cfg(not(feature = "wifi-dma"))]
    pub fn _handle_data_empty(&mut self) {
        self.transport.handle_data_empty();
    }

    /// Issues an RPC, blocking till a response is recieved.
    pub fn blocking_rpc<'a, RPC: erpc::RPC>(
        &mut self,
        mut rpc: RPC,
    ) -> Result<RPC::ReturnValue, erpc::Err<RPC::Error>> {
        // Transmit the request.
        let mut tx_buff = heapless::Vec::new();
        tx_buff
            .extend_from_slice(&rpc.header(self.next_seq()).as_bytes())
            .map_err(|_| erpc::Err::TXErr)?;
        rpc.args(&mut tx_buff);
        self.write_frame(&tx_buff).map_err(|_| erpc::Err::TXErr)?;

        loop {
            let result = self.recieve_rpc_response(&mut rpc);
            if let Err(erpc::Err::NotOurs) = result {
                continue;
            };
            break result;
        }
    }

    fn recieve_rpc_response<'a, RPC: erpc::RPC>(
        &mut self,
        rpc: &mut RPC,
    ) -> Result<RPC::ReturnValue, erpc::Err<RPC::Error>> {
        // Read the frame header
        let fh = match self.recieve_frame_header(rpc) {
            Ok(fh) => fh,
            Err(e) => {
                self.transport.discard_pending();
                return Err(e);
            }
        };

        // Read the payload, check CRC, hand off to underlying trait to decode
        let mut buffer = [0u8; 2048];
        let sz = fh.msg_length as usize;
        if sz > buffer.len() {
            // A corrupt or oversized header; don't index out of range.
            self.transport.discard_pending();
            return Err(erpc::Err::ResponseOverrun);
        }
        if !self.read_ok(&mut buffer[..sz], INTER_BYTE_BUDGET) {
            self.transport.discard_pending();
            return Err(erpc::Err::Unknown);
        }

        let crc = fh.check_crc(&buffer[..sz]);
        if crc.is_err() {
            self.transport.discard_pending();
        }
        crc?;
        rpc.parse(&buffer[..sz])
    }

    fn recieve_frame_header<'a, RPC: erpc::RPC>(
        &mut self,
        _rpc: &mut RPC,
    ) -> Result<erpc::FrameHeader, erpc::Err<RPC::Error>> {
        let mut buffer = [0u8; 4];
        if !self.read_ok(&mut buffer, REPLY_WAIT_BUDGET) {
            self.transport.discard_pending();
            return Err(erpc::Err::Unknown);
        }

        match erpc::FrameHeader::parse(&buffer[..]) {
            Err(e) => Err(erpc::Err::Parsing(e)),
            Ok(fh) => Ok(fh.1),
        }
    }

    fn write_frame(&mut self, msg: &heapless::Vec<u8, 64>) -> Result<(), TransportError> {
        let header = erpc::FrameHeader::new_from_msg(msg);
        self.transport.write(&header.as_bytes())?;
        self.transport.write(msg)
    }

    /// Non-generic so the error path is not duplicated per RPC type.
    fn read_ok(&mut self, buffer: &mut [u8], first_byte_budget: u32) -> bool {
        self.transport.read_exact(buffer, first_byte_budget).is_ok()
    }

    fn next_seq(&mut self) -> u32 {
        self.sequence += 1;
        self.sequence
    }
}

/// Transport instrumentation, used by the `wifi_bench` example to compare the
/// interrupt-driven and DMA-driven UART backends.
///
/// Requires the DWT cycle counter to be running (see `wifi_bench`).
#[cfg(feature = "wifi-stats")]
pub mod stats {
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    use cortex_m::peripheral::DWT;

    static ISR_COUNT: AtomicU32 = AtomicU32::new(0);
    static ISR_CYCLES: AtomicU32 = AtomicU32::new(0);

    /// Counters accumulated since the last [`reset`].
    #[derive(Clone, Copy, Debug)]
    pub struct Snapshot {
        /// Number of wifi transport interrupt handler invocations.
        pub isr_count: u32,
        /// CPU cycles spent inside the handler bodies. Excludes exception
        /// entry/exit, which costs roughly another 25-30 cycles per invocation.
        pub isr_cycles: u32,
    }

    pub fn reset() {
        ISR_COUNT.store(0, Relaxed);
        ISR_CYCLES.store(0, Relaxed);
    }

    pub fn snapshot() -> Snapshot {
        Snapshot {
            isr_count: ISR_COUNT.load(Relaxed),
            isr_cycles: ISR_CYCLES.load(Relaxed),
        }
    }

    /// Accounts for one interrupt handler invocation; create it on entry to
    /// the handler and let it drop on exit.
    pub struct IsrGuard(u32);

    impl IsrGuard {
        #[inline(always)]
        pub fn enter() -> Self {
            Self(DWT::cycle_count())
        }
    }

    impl Drop for IsrGuard {
        #[inline(always)]
        fn drop(&mut self) {
            ISR_CYCLES.fetch_add(DWT::cycle_count().wrapping_sub(self.0), Relaxed);
            ISR_COUNT.fetch_add(1, Relaxed);
        }
    }
}

/// Imports necessary for using `wifi_singleton`.
pub mod wifi_prelude {
    pub use crate::wifi::*;
    pub use atsamd_hal::pac::Sercom0;
    pub use atsamd_hal::pac::{interrupt, Mclk};
    pub use bbqueue::{BBBuffer, Producer};

    pub use cortex_m::interrupt::CriticalSection;
}

/// Declares static globals for the wifi controller, and wires up interrupts.
#[cfg(not(feature = "wifi-dma"))]
#[macro_export]
macro_rules! wifi_singleton {
    ($global_name:ident) => {
        static mut $global_name: Option<Wifi> = None;
        static WIFI_RX: BBBuffer<512> = BBBuffer::new();
        static WIFI_TX: BBBuffer<128> = BBBuffer::new();

        /// Initializes the wifi controller from within an interrupt-free context.
        unsafe fn wifi_init(
            _cs: &CriticalSection,
            pins: WifiPins,
            sercom0: Sercom0,
            clocks: &mut GenericClockController,
            mclk: &mut Mclk,
            delay: &mut Delay,
        ) {
            unsafe {
                $global_name = Some(Wifi::init(
                    pins, sercom0, clocks, mclk, delay, &WIFI_RX, &WIFI_TX,
                ));
            }
        }

        #[interrupt]
        fn SERCOM0_0() {
            // Data Register Empty interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    wifi._handle_data_empty();
                });
            }
        }

        #[interrupt]
        fn SERCOM0_2() {
            // Recieve Complete interrupt.
            unsafe {
                $global_name.as_mut().map(|wifi| {
                    wifi._handle_rx();
                });
            }
        }
    };
}

/// Declares static globals for the wifi controller (DMA transport). Defines the
/// `DMAC_0` handler, which counts receive-ring wraps; no UART interrupts.
#[cfg(feature = "wifi-dma")]
#[macro_export]
macro_rules! wifi_singleton {
    ($global_name:ident) => {
        static mut $global_name: Option<Wifi> = None;
        static mut WIFI_RX_RING: [u8; RX_RING_LEN] = [0; RX_RING_LEN];

        /// Initializes the wifi controller from within an interrupt-free context.
        #[allow(clippy::too_many_arguments)]
        unsafe fn wifi_init(
            _cs: &CriticalSection,
            pins: WifiPins,
            sercom0: Sercom0,
            clocks: &mut GenericClockController,
            mclk: &mut Mclk,
            delay: &mut Delay,
            rx_channel: WifiRxChannel,
            tx_channel: WifiTxChannel,
        ) {
            unsafe {
                $global_name = Some(Wifi::init(
                    pins,
                    sercom0,
                    clocks,
                    mclk,
                    delay,
                    rx_channel,
                    tx_channel,
                    &mut *core::ptr::addr_of_mut!(WIFI_RX_RING),
                ));
            }
        }

        #[interrupt]
        fn DMAC_0() {
            // Receive ring wrapped (DMA channel 0 transfer complete).
            _handle_dma_rx_wrap();
        }
    };
}
