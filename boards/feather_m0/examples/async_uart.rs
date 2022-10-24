#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = bsp::pac, dispatchers = [I2S, AC])]
mod app {
    use bsp::{hal, pac, periph_alias, pin_alias};
    use feather_m0 as bsp;
    use fugit::MillisDuration;
    use hal::{
        clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController},
        prelude::*,
        rtc::{Count32Mode, Rtc},
        sercom::uart::{Config, RxDuplex, TxDuplex, UartFuture},
    };

    #[monotonic(binds = RTC, default = true)]
    type Monotonic = Rtc<Count32Mode>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        uart_rx: UartFuture<Config<bsp::UartPads>, RxDuplex, bsp::pac::Interrupt>,
        uart_tx: UartFuture<Config<bsp::UartPads>, TxDuplex, bsp::pac::Interrupt>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
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

        let sercom0_irq = cortex_m_interrupt::take_nvic_interrupt!(pac::Interrupt::SERCOM0, 4);

        enable_internal_32kosc(&mut peripherals.SYSCTRL);
        let timer_clock = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::OSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);

        // Setup RTC monotonic
        let rtc_clock = clocks.rtc(&timer_clock).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        let (uart_rx, uart_tx) = bsp::uart(
            &mut clocks,
            9600.hz(),
            uart_sercom,
            &mut peripherals.PM,
            uart_rx,
            uart_tx,
        )
        .into_future(sercom0_irq)
        .split();

        send_bytes::spawn().ok();
        receive_bytes::spawn().ok();

        (Shared {}, Local { uart_rx, uart_tx }, init::Monotonics(rtc))
    }

    #[task(local = [uart_tx], priority = 2)]
    async fn send_bytes(cx: send_bytes::Context) {
        let uart = cx.local.uart_tx;

        loop {
            uart.write(&[0x00; 4]).await;
            crate::app::monotonics::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
        }
    }

    #[task(local = [uart_rx], priority = 3)]
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
