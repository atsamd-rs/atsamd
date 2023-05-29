//! MCAN example
//! Assumed bus:
//! - 375 kb/s nominal bitrate
//! - 750 kb/s data bitrate if sending/receiving CAN FD frames with bitrate
//!   switching
//!
//! 1. Sends a message over CAN on SW0 button press and prints out transmit
//! event queue content, protocol status register and error counter register in
//! RTT terminal.
//!
//! 2. Sets up an interrupt line and message filters
//! - messages with standard IDs will end up in RxFifo0
//! - messages with extended IDs will end up in RxFifo1
//! - messages content will be printed out in RTT terminal upon arrival
//!
//! 3. LED0 will blink to indicate activity (sending & receiving)

#![no_std]
#![no_main]

use atsame54_xpro as bsp;
use bsp::hal;
use hal::clock::v2 as clock;
use hal::eic::pin::*;
use hal::gpio::{Interrupt as GpioInterrupt, *};
use hal::prelude::*;

use dwt_systick_monotonic::{DwtSystick, ExtU32};

use mcan::embedded_can as ecan;
use mcan::generic_array::typenum::consts::*;
use mcan::interrupt::{Interrupt, InterruptLine, OwnedInterruptSet};
use mcan::message::rx;
use mcan::message::tx;
use mcan::messageram::SharedMemory;
use mcan::prelude::*;
use mcan::rx_fifo::Fifo0;
use mcan::rx_fifo::Fifo1;
use mcan::rx_fifo::RxFifo;
use mcan::{
    config::{BitTiming, Mode},
    filter::{Action, ExtFilter, Filter},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

pub struct Capacities;

impl mcan::messageram::Capacities for Capacities {
    type StandardFilters = U1;
    type ExtendedFilters = U1;
    type RxBufferMessage = rx::Message<64>;
    type DedicatedRxBuffers = U0;
    type RxFifo0Message = rx::Message<64>;
    type RxFifo0 = U64;
    type RxFifo1Message = rx::Message<64>;
    type RxFifo1 = U64;
    type TxMessage = tx::Message<64>;
    type TxBuffers = U32;
    type DedicatedTxBuffers = U0;
    type TxEventFifo = U32;
}

type RxFifo0 = RxFifo<
    'static,
    Fifo0,
    clock::types::Can1,
    <Capacities as mcan::messageram::Capacities>::RxFifo0Message,
>;
type RxFifo1 = RxFifo<
    'static,
    Fifo1,
    clock::types::Can1,
    <Capacities as mcan::messageram::Capacities>::RxFifo1Message,
>;
type Tx = mcan::tx_buffers::Tx<'static, clock::types::Can1, Capacities>;
type TxEventFifo = mcan::tx_event_fifo::TxEventFifo<'static, clock::types::Can1>;
type Aux = mcan::bus::Aux<
    'static,
    clock::types::Can1,
    hal::can::Dependencies<
        clock::types::Can1,
        clock::gclk::Gclk0Id,
        bsp::Ata6561Rx,
        bsp::Ata6561Tx,
        bsp::pac::CAN1,
    >,
>;
type Button = ExtInt15<Pin<PB31, GpioInterrupt<PullUp>>>;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [FREQM])]
mod app {
    use super::*;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<48_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        button: Button,
        led: bsp::Led,
        line_interrupts: OwnedInterruptSet<clock::types::Can1>,
        rx_fifo_0: RxFifo0,
        rx_fifo_1: RxFifo1,
        tx: Tx,
        tx_event_fifo: TxEventFifo,
        aux: Aux,
    }

    #[init(local = [
        #[link_section = ".can"]
        can_memory: SharedMemory<Capacities> = SharedMemory::new()
    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Application up!");

        let (_buses, clocks, tokens) = clock::clock_system_at_reset(
            ctx.device.OSCCTRL,
            ctx.device.OSC32KCTRL,
            ctx.device.GCLK,
            ctx.device.MCLK,
            &mut ctx.device.NVMCTRL,
        );

        let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            clocks.gclk0.freq().to_Hz(),
        );

        let pins = bsp::Pins::new(ctx.device.PORT);

        let (pclk_eic, gclk0) = clock::pclk::Pclk::enable(tokens.pclks.eic, clocks.gclk0);

        let mut eic = hal::eic::init_with_ulp32k(&mut mclk, pclk_eic.into(), ctx.device.EIC);
        let mut button = bsp::pin_alias!(pins.button).into_pull_up_ei();
        eic.button_debounce_pins(&[button.id()]);
        button.sense(&mut eic, Sense::FALL);
        button.enable_interrupt(&mut eic);
        eic.finalize();

        let can1_rx = bsp::pin_alias!(pins.ata6561_rx).into_mode();
        let can1_tx = bsp::pin_alias!(pins.ata6561_tx).into_mode();
        let mut can1_standby = bsp::pin_alias!(pins.ata6561_standby).into_push_pull_output();

        let _ = can1_standby.set_low();

        let (pclk_can1, gclk0) = clock::pclk::Pclk::enable(tokens.pclks.can1, gclk0);

        let (dependencies, _gclk0) = hal::can::Dependencies::new(
            gclk0,
            pclk_can1,
            clocks.ahbs.can1,
            can1_rx,
            can1_tx,
            ctx.device.CAN1,
        );

        let mut can =
            mcan::bus::CanConfigurable::new(375.kHz(), dependencies, ctx.local.can_memory).unwrap();

        can.config().mode = Mode::Fd {
            allow_bit_rate_switching: true,
            data_phase_timing: BitTiming::new(750.kHz()),
        };

        let line_interrupts = can
            .interrupts()
            .enable(
                [
                    Interrupt::RxFifo0NewMessage,
                    Interrupt::RxFifo0Full,
                    Interrupt::RxFifo0MessageLost,
                    Interrupt::RxFifo1NewMessage,
                    Interrupt::RxFifo1Full,
                    Interrupt::RxFifo1MessageLost,
                ]
                .into_iter()
                .collect(),
                InterruptLine::Line0,
                // ATSAMD chips do not expose separate NVIC lines to MCAN
                // InterruptLine::Line0 and InterruptLine::Line1 are wired
                // together in the hardware.
            )
            .unwrap();

        can.filters_standard()
            .push(Filter::Classic {
                action: Action::StoreFifo0,
                filter: ecan::StandardId::MAX,
                mask: ecan::StandardId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Standard filter application failed"));

        can.filters_extended()
            .push(ExtFilter::Classic {
                action: Action::StoreFifo1,
                filter: ecan::ExtendedId::MAX,
                mask: ecan::ExtendedId::ZERO,
            })
            .unwrap_or_else(|_| panic!("Extended filter application failed"));

        let can = can.finalize().unwrap();

        let rx_fifo_0 = can.rx_fifo_0;
        let rx_fifo_1 = can.rx_fifo_1;
        let tx = can.tx;
        let tx_event_fifo = can.tx_event_fifo;
        let aux = can.aux;

        let led = bsp::pin_alias!(pins.led).into();

        bump_activity_led();

        (
            Shared {},
            Local {
                button,
                led,
                line_interrupts,
                rx_fifo_0,
                rx_fifo_1,
                tx,
                tx_event_fifo,
                aux,
            },
            init::Monotonics(mono),
        )
    }

    #[task(binds = EIC_EXTINT_15, local = [counter: u16 = 0, button, tx_event_fifo, aux, tx])]
    fn button(ctx: button::Context) {
        ctx.local.button.clear_interrupt();
        bump_activity_led();
        rprintln!("Button pressed! Status:");
        while let Some(e) = ctx.local.tx_event_fifo.pop() {
            rprintln!("TxEvent: {:0X?}", e);
        }
        rprintln!("{:?}", ctx.local.aux.protocol_status());
        rprintln!("{:?}", ctx.local.aux.error_counters());

        let counter = *ctx.local.counter;
        let wrapped_counter = (counter % u8::MAX as u16) as u8;
        let mut payload = [0_u8; 64];
        payload.fill(wrapped_counter);

        ctx.local
            .tx
            .transmit_queued(
                tx::MessageBuilder {
                    id: ecan::Id::Extended(ecan::ExtendedId::new(counter as _).unwrap()),
                    frame_type: tx::FrameType::FlexibleDatarate {
                        payload: &payload,
                        bit_rate_switching: true,
                        force_error_state_indicator: false,
                    },
                    store_tx_event: Some(wrapped_counter),
                }
                .build()
                .unwrap(),
            )
            .unwrap();
        rprintln!("Message {:0X} sent!", counter);
        *ctx.local.counter += 1;
    }

    #[task(priority = 2, binds = CAN1, local = [line_interrupts, rx_fifo_0, rx_fifo_1])]
    fn can1(mut ctx: can1::Context) {
        bump_activity_led();
        let line_interrupts = ctx.local.line_interrupts;
        for interrupt in line_interrupts.iter_flagged() {
            match interrupt {
                Interrupt::RxFifo0NewMessage => {
                    for message in &mut ctx.local.rx_fifo_0 {
                        log("RxFifo0", &message);
                    }
                }
                Interrupt::RxFifo1NewMessage => {
                    for message in &mut ctx.local.rx_fifo_1 {
                        log("RxFifo1", &message);
                    }
                }
                i => rprintln!("{:?} interrupt triggered", i),
            }
        }
    }

    #[task(local = [led])]
    fn activity_led(ctx: activity_led::Context, led_on: bool) {
        let _ = ctx.local.led.set_state((!led_on).into());
        if led_on {
            let _ = activity_led::spawn_after(100.millis(), false);
        }
    }

    fn bump_activity_led() {
        let _ = activity_led::spawn(true);
    }

    fn log(fifo: &str, message: &impl mcan::message::Raw) {
        rprintln!("New message received ({})", fifo);
        rprintln!("id:                 {:0X?}", message.id());
        rprintln!("decoded_dlc:        {:?}", message.decoded_dlc());
        rprintln!("fd_format:          {:?}", message.fd_format());
        rprintln!("bit_rate_switching: {:?}", message.bit_rate_switching());
        rprintln!("is_remote_frame:    {:?}", message.is_remote_frame());
        rprintln!("data:               {:0X?}", message.data());
    }
}
