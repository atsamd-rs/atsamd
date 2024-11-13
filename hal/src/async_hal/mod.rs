//! # Asynchronous HAL APIs
//!
//! This HAL provides a comprehensive and efficient way to interact with
//! the underlying peripherals in an asynchronous fashion, enabling concurrent
//! and non-blocking programming through the use of `async`/`await` APIs.
//!
//! This module provides the basis for interacting with peripherals through
//! `async` APIs. Notably, in order to function correctly and wake an `await`ing
//! [`Future`](core::future::Future), peripherals must be able to signal when
//! their respective interrupts fire. Traditionally, the user manually writes
//! their own interrupt handlers. When using `async` APIs, the peripherals
//! effectively take control of their own interrupt handlers in order to wake
//! tasks at the appropriate time.
//!
//! ## Using the `async` APIs
//!
//! To use the asynchronous APIs provided by the HAL, enable the `async` Cargo
//! feature. Note that this uses a certain amount of static RAM in order to
//! initialize wakers for each peripheral.
//!
//! ## Supported peripherals
//!
//! * [`UART`](crate::sercom::uart)
//! * [`SPI`](crate::sercom::spi)
//! * [`I2C`](crate::sercom::i2c)
//! * [`DMAC`](crate::dmac)
//! * [`EIC`](crate::eic) (external GPIO interrupts)
//! * [`Timers`](crate::timer)
//!
//!  **Note**: The asynchronous APIs for the individual peripherals are provided
//! in their respective modules. This module only deals with the generalities of
//! using `async` constructs throughout the HAL.
//!
//! ## Declaring interrupt bindings
//!
//! In order for the peripherals to wake their respective tasks, the interrupt
//! sources must be bound to their handler at compile time. A struct that
//! implements [`Binding`](self::interrupts::Binding) must be passed to an async
//! peripheral in order to prove to the compiler that the correct interrupt
//! handlers have been bound.
//!
//! This module provides convenient macros that generate the interrupt bindings.
//! Use [`bind_interrupts`](crate::bind_interrupts) to bind single interrupt
//! sources to handlers. See also [Declaring multiple interrupt source
//! bindings](#declaring-multiple-interrupt-source-bindings).
//!
//! ## Declaring multiple interrupt source bindings
//!
//! For some `thumbv7em` peripherals, there are multiple interrupt sources used
//! by a single peripheral. In these cases, we must provide a binding to an
//! interrupt handler for each of these interrupt sources in order for the
//! peripheral driver to function properly. The HAL defines only one interrupt
//! "source" per peripheral. Your job is to tell it where to find all the
//! relevant interrupts it must use to operate the peripheral properly. Use
//! [`bind_multiple_interrupts`](crate::bind_multiple_interrupts) to bind
//! multiple interrupts to a single handler.
//!
//! Currently, the supported peripherals which have multiple interrupts per
//! peripheral (**thumbv7em targets only**):
//!
//! * `SERCOMx: [SERCOMx_0, SERCOMx_1, SERCOMx_2, SERCOMx_OTHER]`
//! * `DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER]`
//!
//! ## Complete example using the `feather_m0` BSP and the [Embassy executor](https://crates.io/crates/embassy-executor)
//! ```no_run
//! #![no_std]
//! #![no_main]
//!
//! use defmt_rtt as _;
//! use panic_probe as _;
//!
//! use bsp::hal;
//! use bsp::pac;
//! use feather_m0 as bsp;
//! use fugit::MillisDuration;
//! use hal::{
//!     clock::GenericClockController,
//!     dmac::{DmaController, PriorityLevel},
//!     prelude::*,
//!     sercom::Sercom4,
//! };
//! use rtic_monotonics::systick::Systick;
//!
//! atsamd_hal::bind_interrupts!(struct Irqs {
//!     SERCOM4 => atsamd_hal::sercom::spi::InterruptHandler<Sercom4>;
//!     DMAC => atsamd_hal::dmac::InterruptHandler;
//! });
//!
//! #[embassy_executor::main]
//! async fn main(_s: embassy_executor::Spawner) {
//!     let mut peripherals = pac::Peripherals::take().unwrap();
//!     let _core = pac::CorePeripherals::take().unwrap();
//!
//!     let mut clocks = GenericClockController::with_external_32kosc(
//!         peripherals.gclk,
//!         &mut peripherals.pm,
//!         &mut peripherals.sysctrl,
//!         &mut peripherals.nvmctrl,
//!     );
//!
//!     let pins = bsp::Pins::new(peripherals.port);
//!
//!     // Take SPI pins
//!     let (miso, mosi, sclk) = (pins.miso, pins.mosi, pins.sclk);
//!
//!     // Initialize DMA Controller
//!     let dmac = DmaController::init(peripherals.dmac, &mut peripherals.pm);
//!
//!     // Turn dmac into an async controller
//!     let mut dmac = dmac.into_future(Irqs);
//!     // Get individual handles to DMA channels
//!     let channels = dmac.split();
//!
//!     // Initialize DMA Channels 0 and 1
//!     let channel0 = channels.0.init(PriorityLevel::Lvl0);
//!     let channel1 = channels.1.init(PriorityLevel::Lvl0);
//!
//!     let mut spi = bsp::spi_master(
//!         &mut clocks,
//!         100.kHz(),
//!         peripherals.sercom4,
//!         &mut peripherals.pm,
//!         sclk,
//!         mosi,
//!         miso,
//!     )
//!     .into_future(Irqs)
//!     .with_dma_channels(channel0, channel1);
//!
//!     loop {
//!         defmt::info!("Sending 0x00 to SPI device...");
//!         spi.write(&[0x00]).await.unwrap();
//!
//!         defmt::info!("Sent 0x00.");
//!
//!         let mut buffer = [0xff; 4];
//!         spi.read(&mut buffer).await.unwrap();
//!         defmt::info!("Read buffer: {:#x}", buffer);
//!         Systick::delay(MillisDuration::<u32>::from_ticks(500).convert()).await;
//!     }
//! }
//! ```

pub mod interrupts;

/// Bind interrupt sources to a single handler each.
///
/// This defines the right interrupt handlers, creates a unit struct (like
/// `struct Irqs;`) and implements the right
/// [`Binding`](crate::async_hal::interrupts::Binding)s for it. You can pass
/// this struct to drivers to prove at compile-time that the right interrupts
/// have been bound.
///
/// Refer to the module-level documentation for details on when to use
/// [`bind_interrupts`](crate::bind_interrupts) vs
/// [`bind_multiple_interrupts`](crate::bind_multiple_interrupts).
///
/// ## Example
/// ```
/// use atsamd_hal::{dmac, sercom, bind_interrupts};
///
/// bind_interrupts!(struct Irqs {
///     SERCOM0 => sercom::i2c::InterruptHandler;
///     DMAC => dmac::InterruptHandler;
/// });
/// ```
#[macro_export]
macro_rules! bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $($handler:ty),*;)* }) => {
        #[derive(Copy, Clone)]
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe extern "C" fn $irq() {
                $(
                    <$handler as $crate::async_hal::interrupts::Handler<$crate::async_hal::interrupts::$irq>>::on_interrupt();
                )*
            }

            $(
                unsafe impl $crate::async_hal::interrupts::Binding<$crate::async_hal::interrupts::$irq, $handler> for $name
                    where $crate::async_hal::interrupts::$irq: $crate::async_hal::interrupts::SingleInterruptSource
                    {}
            )*
        )*
    };
}

/// Bind multiple interrupt sources to the same interrupt handler.
///
/// This defines the right interrupt handlers, creates a unit struct (like
/// `struct Irqs;`) and implements the right
/// [`Binding`](crate::async_hal::interrupts::Binding)s for it. You can pass
/// this struct to drivers to prove at compile-time that the right interrupts
/// have been bound.
///
/// Due to limitations in the macro's implementation, only one binding per macro
/// call is supported. Call [`bind_multiple_interrupts`] as many times as you
/// need multiple-interrupt bindings.
///
/// Refer to the module-level documentation for details on when to use
/// [`bind_interrupts`] vs
/// [`bind_multiple_interrupts`].
///
/// ## Example
/// ```
/// atsamd_hal::bind_multiple_interrupts!(struct DmacIrqs {
///     DMAC: [DMAC_0, DMAC_1, DMAC_2, DMAC_OTHER] => atsamd_hal::dmac::InterruptHandler;
/// });
///
/// atsamd_hal::bind_multiple_interrupts!(struct SpiIrqs {
///     SERCOM0: [SERCOM0_0, SERCOM0_1, SERCOM0_2, SERCOM0_OTHER] => atsamd_hal::sercom::spi::InterruptHandler;
/// });
/// ```
///
/// [`bind_interrupts`]: crate::bind_interrupts
/// [`bind_multiple_interrupts`]: crate::bind_multiple_interrupts
#[macro_export]
macro_rules! bind_multiple_interrupts {
        ($vis:vis struct $name:ident { $int_source:ident: [ $($irq:ident),+ ] => $handler:ty; }) => {
        #[derive(Copy, Clone)]
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe extern "C" fn $irq() {
                <$handler as $crate::async_hal::interrupts::Handler<$crate::async_hal::interrupts::$int_source>>::on_interrupt();
            }
        )+

        unsafe impl $crate::async_hal::interrupts::Binding<$crate::async_hal::interrupts::$int_source, $handler> for $name
            where $crate::async_hal::interrupts::$int_source: $crate::async_hal::interrupts::MultipleInterruptSources
            {}
    };
}
