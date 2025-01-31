#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::{hal, pac, periph_alias, pin_alias};
use feather_m0 as bsp;
use hal::{
    clock::GenericClockController,
    dmac::{Ch0, Ch1, DmaController, PriorityLevel},
    fugit::MillisDuration,
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

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);

    // Take rx and tx pins
    let (uart_rx, uart_tx) = (pin_alias!(pins.uart_rx), pin_alias!(pins.uart_tx));
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);
    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(Irqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channels 0 and 1
    let channel0 = channels.0.init(PriorityLevel::Lvl0);
    let channel1 = channels.1.init(PriorityLevel::Lvl0);

    let (uart_rx, uart_tx) = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut peripherals.pm,
        uart_rx,
        uart_tx,
    )
    .into_future(Irqs)
    .with_rx_dma_channel(channel0)
    .with_tx_dma_channel(channel1)
    .split();

    // For embassy-executor, spawning multiple tasks on the same executor requires
    // either:
    // * Tuning the task arena size either via a Cargo feature or the
    //   `EMBASSY_EXECUTOR_TASK_ARENA_SIZE` environment variable
    // * Using the `nightly` Cargo feature along with
    //   #![feature(type_alias_impl_trait)]
    spawner.spawn(send_bytes(uart_tx)).unwrap();
    spawner.spawn(receive_bytes(uart_rx)).unwrap();
}

#[embassy_executor::task]
async fn send_bytes(mut uart_tx: UartFutureTxDuplexDma<Config<bsp::UartPads>, Ch1>) {
    loop {
        uart_tx.write(b"Hello, world!").await.unwrap();
        defmt::info!("Sent 10 bytes");
        Systick::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
    }
}

#[embassy_executor::task]
async fn receive_bytes(mut uart_rx: UartFutureRxDuplexDma<Config<bsp::UartPads>, Ch0>) {
    uart_rx.as_mut().flush_rx_buffer();

    loop {
        let mut buf = [0x00; 10];
        match uart_rx.read(&mut buf).await {
            Ok(()) => defmt::info!("read {:#x}", &buf),
            Err(_) => {
                defmt::error!("UART Error.");
                // Flusing the RX buffer may drop a few bytes.
                uart_rx.as_mut().flush_rx_buffer();
            }
        }
    }
}
