#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use bsp::{hal, periph_alias, pin_alias};
use feather_m0 as bsp;
use fugit::MillisDuration;
use hal::{
    clock::GenericClockController,
    dmac::{Ch0, Ch1, DmaController, PriorityLevel},
    prelude::*,
    sercom::{
        uart::{Config, UartFutureRxDuplexDma, UartFutureTxDuplexDma},
        Sercom0,
    },
};
use rtic_monotonics::systick::Systick;

atsamd_hal::bind_interrupts!(struct Irqs {
    SERCOM0 => atsamd_hal::sercom::uart::InterruptHandler<Sercom0>;
    DMAC => atsamd_hal::dmac::InterruptHandler;
});

#[rtic::app(device = bsp::pac, dispatchers = [I2S, AC])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        uart_rx: UartFutureRxDuplexDma<Config<bsp::UartPads>, Ch0>,
        uart_tx: UartFutureTxDuplexDma<Config<bsp::UartPads>, Ch1>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut peripherals = cx.device;
        let _core = cx.core;

        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = bsp::Pins::new(peripherals.PORT);

        // Take rx and tx pins
        let (uart_rx, uart_tx) = (pin_alias!(pins.uart_rx), pin_alias!(pins.uart_tx));
        let uart_sercom = periph_alias!(peripherals.uart_sercom);

        // Initialize DMA Controller
        let dmac = DmaController::init(peripherals.DMAC, &mut peripherals.PM);
        // Turn dmac into an async controller
        let mut dmac = dmac.into_future(Irqs);
        // Get individual handles to DMA channels
        let channels = dmac.split();

        // Initialize DMA Channels 0 and 1
        let channel0 = channels.0.init(PriorityLevel::LVL0);
        let channel1 = channels.1.init(PriorityLevel::LVL0);

        let (uart_rx, uart_tx) = bsp::uart(
            &mut clocks,
            9600.Hz(),
            uart_sercom,
            &mut peripherals.PM,
            uart_rx,
            uart_tx,
        )
        .into_future(Irqs)
        .with_rx_dma_channel(channel0)
        .with_tx_dma_channel(channel1)
        .split();

        send_bytes::spawn().ok();
        receive_bytes::spawn().ok();

        (Shared {}, Local { uart_rx, uart_tx })
    }

    #[task(local = [uart_tx], priority = 1)]
    async fn send_bytes(cx: send_bytes::Context) {
        let uart = cx.local.uart_tx;

        loop {
            uart.write(&[0x00; 10]).await;
            defmt::info!("Sent 10 bytes");
            Systick::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
        }
    }

    #[task(local = [uart_rx], priority = 2)]
    async fn receive_bytes(cx: receive_bytes::Context) {
        let uart = cx.local.uart_rx;
        uart.as_mut().flush_rx_buffer();

        loop {
            let mut buf = [0x00; 10];
            match uart.read(&mut buf).await {
                Ok(()) => defmt::info!("read {}", &buf),
                Err(_) => {
                    defmt::error!("UART Error.");
                    // Flusing the RX buffer may drop a few bytes.
                    uart.as_mut().flush_rx_buffer();
                }
            }
        }
    }
}
