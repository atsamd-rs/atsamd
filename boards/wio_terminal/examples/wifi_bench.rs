//! Benchmark for the wifi transport (eRPC over UART to the RTL8720).
//!
//! Build and flash it once per transport backend and compare the numbers
//! shown on the LCD:
//!
//! ```text
//! cargo build --release --example wifi_bench --features wifi-stats
//! cargo build --release --example wifi_bench --features wifi-stats,wifi-dma
//! ```
//!
//! Three things are measured:
//!
//! 1. `GetVersion` round trips: a small request and a small reply. This is
//!    dominated by the RTL8720's turnaround time and shows whether the
//!    transport adds latency to ordinary command exchange.
//! 2. `ScanGetAP` round trips: a small request and the largest reply the
//!    current RPC set produces (about 1 kB), i.e. a sustained RX stream.
//! 3. Stall tolerance: both workloads are repeated while a 1 kHz timer
//!    interrupt, which the wifi interrupts cannot preempt, keeps the CPU busy
//!    for a growing number of microseconds. This emulates other interrupt
//!    handlers or critical sections in an application. Each step prints its
//!    label before it runs and `ok` after it; a transport that drops bytes
//!    either reports errors or panics, which leaves the last label on screen
//!    without a result.
//!
//! For 1 and 2 the screen shows min/avg/max round trip time, the number of
//! transport interrupts per RPC and the share of CPU time spent in the
//! transport's interrupt handlers (handler bodies only; add roughly 25-30
//! cycles of exception entry/exit per interrupt).

#![no_std]
#![no_main]
#![allow(static_mut_refs)]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::fugit::ExtU32;
use wio::hal::timer::TimerCounter;
use wio::pac::{CorePeripherals, Peripherals, Tc3};
use wio::prelude::*;
use wio::wifi_prelude::*;
use wio::wifi_rpcs as rpc;
use wio::wifi_stats;
use wio::{entry, wifi_singleton};

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::{DWT, NVIC};

use eg::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::{Baseline, Text};

use generic_array::typenum::consts::U16;
use heapless::String;

const BACKEND: &str = if cfg!(feature = "wifi-dma") {
    "dma"
} else {
    "irq"
};

/// Round trips per workload in the latency tests.
const SMALL_RPCS: u32 = 200;
const LARGE_RPCS: u32 = 20;

/// Time the 1 kHz interrupt keeps the CPU busy in each stall test step.
const STALL_STEPS_US: [u32; 8] = [0, 10, 20, 30, 50, 100, 200, 400];
/// Round trips per workload in each stall test step.
const STALL_SMALL_RPCS: u32 = 50;
const STALL_LARGE_RPCS: u32 = 10;

/// How long the TC3 handler spins, in CPU cycles.
static STALL_CYCLES: AtomicU32 = AtomicU32::new(0);

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let sets = wio::Pins::new(peripherals.port).split();

    // All timing is done with the free-running CPU cycle counter.
    core.DCB.enable_trace();
    core.DWT.enable_cycle_counter();

    let (display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.sercom7,
            &mut peripherals.mclk,
            58.MHz(),
            &mut delay,
        )
        .unwrap();
    let mut console = Console::new(display);

    // The stall timer runs off GCLK0, which is also the CPU clock.
    let gclk0 = clocks.gclk0();
    let tc_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let cpu_hz = tc_clock.freq().to_Hz();
    let mut stall_timer = TimerCounter::tc3_(&tc_clock, peripherals.tc3, &mut peripherals.mclk);

    // Initialize the wifi peripheral. This block is the only part of the
    // benchmark that depends on the transport backend.
    #[cfg(feature = "wifi-dma")]
    let mut dmac = wio::hal::dmac::DmaController::init(peripherals.dmac, &mut peripherals.pm);
    #[cfg(feature = "wifi-dma")]
    let channels = dmac.split();
    #[cfg(feature = "wifi-dma")]
    let rx_channel = channels.0.init(wio::hal::dmac::PriorityLevel::Lvl3);
    #[cfg(feature = "wifi-dma")]
    let tx_channel = channels.1.init(wio::hal::dmac::PriorityLevel::Lvl0);
    let nvic = &mut core.NVIC;
    disable_interrupts(|cs| unsafe {
        #[cfg(not(feature = "wifi-dma"))]
        wifi_init(
            cs,
            sets.wifi,
            peripherals.sercom0,
            &mut clocks,
            &mut peripherals.mclk,
            &mut delay,
        );
        #[cfg(feature = "wifi-dma")]
        wifi_init(
            cs,
            sets.wifi,
            peripherals.sercom0,
            &mut clocks,
            &mut peripherals.mclk,
            &mut delay,
            rx_channel,
            tx_channel,
        );
        if let Some(wifi) = WIFI.as_mut() {
            wifi.enable(cs, nvic);
        }
    });
    let wifi = unsafe { WIFI.as_mut().unwrap() };

    let version = wifi.blocking_rpc(rpc::GetVersion {}).unwrap();
    console.line(format_args!(
        "wifi_bench [{}] fw {} {} baud",
        BACKEND,
        version,
        wio::WIFI_UART_BAUD
    ));

    // 1. Small request, small reply.
    let small = run(SMALL_RPCS, || wifi.blocking_rpc(rpc::GetVersion {}).is_ok());
    console.report("GetVersion", &small, cpu_hz);

    // 2. Small request, large reply. Scan once so there is something to fetch.
    wifi.blocking_rpc(rpc::ScanStart {}).unwrap();
    while wifi.blocking_rpc(rpc::IsScanning {}).unwrap() {
        delay.delay_ms(50u8);
    }
    let large = run(LARGE_RPCS, || {
        wifi.blocking_rpc(rpc::ScanGetAP::<U16>::new()).is_ok()
    });
    console.report("ScanGetAP", &large, cpu_hz);

    // 3. Stall tolerance. The SAMD51 implements the top 3 priority bits only,
    // so the wifi interrupts' priority of 1 is really 0, the highest. TC3 at
    // 0 is therefore not preempted by them and delays them while it spins.
    console.line(format_args!("1 kHz stall, us per ms:"));
    InterruptDrivenTimer::start(&mut stall_timer, 1.millis());
    stall_timer.enable_interrupt();
    unsafe {
        core.NVIC.set_priority(wio::pac::Interrupt::TC3, 0);
        NVIC::unmask(wio::pac::Interrupt::TC3);
    }
    for stall_us in STALL_STEPS_US {
        console.partial(format_args!("{}..", stall_us));
        STALL_CYCLES.store((cpu_hz / 1_000_000) * stall_us, Relaxed);

        let small = run(STALL_SMALL_RPCS, || {
            wifi.blocking_rpc(rpc::GetVersion {}).is_ok()
        });
        let large = run(STALL_LARGE_RPCS, || {
            wifi.blocking_rpc(rpc::ScanGetAP::<U16>::new()).is_ok()
        });

        match small.errors + large.errors {
            0 => console.partial(format_args!("ok ")),
            n => console.partial(format_args!("E{} ", n)),
        }
    }
    STALL_CYCLES.store(0, Relaxed);
    console.newline();
    console.line(format_args!("done"));

    loop {
        delay.delay_ms(200u8);
    }
}

wifi_singleton!(WIFI);

#[interrupt]
fn TC3() {
    // Writing a 1 clears the overflow flag.
    unsafe { Tc3::steal() }
        .count16()
        .intflag()
        .write(|w| w.ovf().set_bit());

    ensure_cycle_counter();
    let start = DWT::cycle_count();
    let stall = STALL_CYCLES.load(Relaxed);
    while DWT::cycle_count().wrapping_sub(start) < stall {
        // Never spin forever if a probe stops the counter mid-stall.
        ensure_cycle_counter();
    }
}

/// Ending a debug probe session (e.g. `probe-rs` exiting) clears
/// `DEMCR.TRCENA`, which stops the cycle counter. All timing here, and the
/// TC3 stall loop, depend on it, so turn it back on if it was switched off.
fn ensure_cycle_counter() {
    const DEMCR: *mut u32 = 0xE000_EDFC as *mut u32;
    const TRCENA: u32 = 1 << 24;
    const DWT_CTRL: *mut u32 = 0xE000_1000 as *mut u32;
    const CYCCNTENA: u32 = 1;
    unsafe {
        let demcr = core::ptr::read_volatile(DEMCR);
        if demcr & TRCENA == 0 {
            core::ptr::write_volatile(DEMCR, demcr | TRCENA);
        }
        let ctrl = core::ptr::read_volatile(DWT_CTRL);
        if ctrl & CYCCNTENA == 0 {
            core::ptr::write_volatile(DWT_CTRL, ctrl | CYCCNTENA);
        }
    }
}

/// Results of running one workload a number of times.
struct Run {
    count: u32,
    errors: u32,
    min_cycles: u32,
    max_cycles: u32,
    total_cycles: u64,
    isr_count: u32,
    isr_cycles: u32,
}

/// Calls `rpc` `count` times, timing each call. `rpc` returns whether the
/// round trip succeeded.
fn run(count: u32, mut rpc: impl FnMut() -> bool) -> Run {
    let mut result = Run {
        count,
        errors: 0,
        min_cycles: u32::MAX,
        max_cycles: 0,
        total_cycles: 0,
        isr_count: 0,
        isr_cycles: 0,
    };

    ensure_cycle_counter();
    wifi_stats::reset();
    for _ in 0..count {
        let start = DWT::cycle_count();
        let ok = rpc();
        let cycles = DWT::cycle_count().wrapping_sub(start);

        if !ok {
            result.errors += 1;
        }
        result.min_cycles = result.min_cycles.min(cycles);
        result.max_cycles = result.max_cycles.max(cycles);
        result.total_cycles += cycles as u64;
    }
    let stats = wifi_stats::snapshot();
    result.isr_count = stats.isr_count;
    result.isr_cycles = stats.isr_cycles;

    result
}

/// Line-oriented text output on the LCD.
struct Console {
    display: wio::LCD,
    row: i32,
    column: i32,
}

impl Console {
    const ROW_HEIGHT: i32 = 12;
    const COLUMN_WIDTH: i32 = 6;

    fn new(mut display: wio::LCD) -> Self {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240))
            .into_styled(style)
            .draw(&mut display)
            .ok();

        Self {
            display,
            row: 0,
            column: 0,
        }
    }

    /// Writes text at the cursor, without moving to the next row.
    fn partial(&mut self, args: core::fmt::Arguments) {
        let mut text = String::<64>::new();
        write!(text, "{}", args).ok();
        // Wrap instead of drawing past the right edge of the 320 px screen.
        if self.column > 0 && (3 + (self.column + text.len() as i32) * Self::COLUMN_WIDTH) > 320 {
            self.newline();
        }

        let position = Point::new(
            3 + self.column * Self::COLUMN_WIDTH,
            3 + self.row * Self::ROW_HEIGHT,
        );
        Text::with_baseline(
            text.as_str(),
            position,
            MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
            Baseline::Top,
        )
        .draw(&mut self.display)
        .ok();

        self.column += text.len() as i32;
    }

    fn newline(&mut self) {
        self.row += 1;
        self.column = 0;
    }

    fn line(&mut self, args: core::fmt::Arguments) {
        self.partial(args);
        self.newline();
    }

    fn report(&mut self, name: &str, run: &Run, cpu_hz: u32) {
        let us = |cycles: u64| cycles * 1_000_000 / cpu_hz as u64;
        let total = run.total_cycles.max(1);
        // Share of the wall time spent in transport interrupt handlers, in
        // tenths of a percent.
        let isr_permille = run.isr_cycles as u64 * 1000 / total;

        self.line(format_args!(
            "{} x{}, {} errors",
            name, run.count, run.errors
        ));
        self.line(format_args!(
            " us min/avg/max {}/{}/{}",
            us(run.min_cycles as u64),
            us(run.total_cycles / run.count.max(1) as u64),
            us(run.max_cycles as u64),
        ));
        self.line(format_args!(
            " isr/rpc {}  isr cpu {}.{}%",
            run.isr_count / run.count.max(1),
            isr_permille / 10,
            isr_permille % 10,
        ));
    }
}
