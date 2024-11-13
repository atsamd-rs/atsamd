#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use bsp::{hal, pac, periph_alias, pin_alias};
use hal::fugit::Hertz;
use hal::fugit::MillisDuration;
use hal::{
    clock::GenericClockController,
    dmac::{Ch0, Ch1, DmaController, PriorityLevel},
    prelude::*,
    sercom::{
        uart::{Config, UartFutureRxDuplexDma, UartFutureTxDuplexDma},
        Sercom3,
    },
};
use metro_m4 as bsp;
use rtic_monotonics::Monotonic;

rtic_monotonics::systick_monotonic!(Mono, 10000);

atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
    DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
});

atsamd_hal::bind_multiple_interrupts!(struct UartIrqs {
    SERCOM3: [SERCOM3_0, SERCOM3_1, SERCOM3_2, SERCOM3_3, SERCOM3_OTHER] => atsamd_hal::sercom::uart::InterruptHandler<Sercom3>;
});

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let freq: Hertz<u32> = clocks.gclk0().into();
    Mono::start(_core.SYST, freq.to_Hz());

    let pins = bsp::Pins::new(peripherals.port);

    // Take rx and tx pins
    let (uart_rx, uart_tx) = (pin_alias!(pins.d0), pin_alias!(pins.d1));
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    // Initialize DMA Controller
    let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);
    // Turn dmac into an async controller
    let mut dmac = dmac.into_future(DmacIrqs);
    // Get individual handles to DMA channels
    let channels = dmac.split();

    // Initialize DMA Channels 0 and 1
    let channel0 = channels.0.init(PriorityLevel::Lvl0);
    let channel1 = channels.1.init(PriorityLevel::Lvl0);

    let (uart_rx, uart_tx) = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    )
    .into_future(UartIrqs)
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
        Mono::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
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
