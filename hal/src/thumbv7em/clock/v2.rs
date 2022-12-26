//! # Version 2 of the `clock` module
//!
//! ## Overview
//!
//! This module provides a simple, ergonomic, and most of all **safe** API to
//! create and manage the clock tree in ATSAMD5x and E5x devices. It uses
//! [type-level programming techniques](crate::typelevel) to prevent users from
//! creating invalid or unsound clocking configurations.
//!
//! <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
//! <strong> Note: </strong> Using a type-level API does place some limitations
//! on how the clocks can be configured. The types must be checked at
//! compile-time, which means the state of a given clock must also be known at
//! compile-time. This is exceedingly reasonable for most clocking
//! configurations, because most users set up their clocks once and never change
//! them again. However, if you need to dynamically change the clocking
//! configuration at run-time, you may find this API less ergonomic. A future,
//! fully dynamic API has been discussed, but nothing has been developed so far.
//! </p>
//!
//! The sections that follow provide an explanation of key concepts in the
//! module. We highly recommend users read through them to better understand the
//! `clock` module API. A [complete example](self#getting-started) is also
//! provided.
//!
//! ## Clock safety
//!
//! A clock tree represents dependencies among clocks, where producer clocks
//! feed consumer clocks. Root clocks are the original producers, as they are
//! derived from oscillators or external clocks. Branch clocks are both
//! producers and consumers, since they modify and distribute clocks. And leaf
//! clocks are consumers only; they drive peripherals or external clock outputs
//! but do not feed other clocks.
//!
//! To safely create and use a clock tree, it is critical that producer clocks
//! not be modified or disabled while their consumer clocks are still in active
//! use. Stated differently, if clock `B` consumes clock `A`, then clock `A`
//! **must not** be modified or disabled while clock `B` is still in use.
//!
//! Notice that this requirement mimics the principle of "aliased XOR mutable"
//! underlying the Rust borrow checker. A producer clock can only be modified if
//! it is not "borrowed" (consumed) by any other clocks.
//!
//! The following sections will review the various type-level programming
//! techniques used to enforce this principle in the `clock` module.
//!
//! ## Clock state machines
//!
//! Each available clock is represented in Rust as a unique, singleton object.
//! Users cannot create two instances of the same clock without using `unsafe`.
//!
//! However, a given clock is not always represented with the same **type**.
//! Specifically, each clock has at least two representations, one for the
//! configured and enabled clock, and another for the unconfigured and disabled
//! clock.
//!
//! These states are represented in Rust using distinct types, forming a
//! type-level state machine. Moreover, the disabled state is always represented
//! by a `Token` type. As the name implies, `Token`s have no functionality on
//! their own; they can only be exchanged for a different type representing
//! another state.
//!
//! ## Clock relationships
//!
//! In general, there are two classes of clock in ATSAMD chips. Some clocks map
//! one-to-one (1:1) to a specific bus or peripheral. This is true for the AHB
//! clocks ([`AhbClk`]s), APB clocks ([`ApbClk`]s), GCLK outputs ([`GclkOut`]s),
//! peripheral channel clocks ([`Pclk`]s), and RTC oscillator ([`RtcOsc`]).
//! Other clocks form one-to-many (1:N) relationships, like the external crystal
//! oscillator ([`Xosc`]), the 48 MHz DFLL ([`Dfll`]) or the two DPLLs
//! ([`Dpll`]).
//!
//! The `clock` module uses a distinct approach for each class.
//!
//! ### 1:1 clocks
//!
//! One-to-one relationships are easily modelled in Rust using move semantics.
//! For example, an enabled peripheral channel clock is represented as a
//! [`Pclk`] object. The respective peripheral API can move the `Pclk` and take
//! ownership of it. In that case, the `Pclk` acts as proof that the peripheral
//! clock is enabled, and the transfer of ownership prevents users from
//! modifying or disabling the `Pclk` while it is in use by the peripheral.
//!
//! One-to-one clocks generally have little to no configuration. They are
//! typically converted directly from disabled `Token` types to fully enabled
//! clock types. For example, the `Pclk` type has only two methods,
//! [`Pclk::enable`] and [`Pclk::disable`], which convert [`PclkToken`]s to
//! `Pclk`s and vice versa.
//!
//! ### 1:N clocks
//!
//! One-to-many relationships are more difficult to model in Rust.
//!
//! As discussed above, we are trying to create something akin to "aliased XOR
//! mutable", where producer clocks cannot be modified while used by consumer
//! clocks. A natural approach would be to use the Rust borrow checker directly.
//! In that case, consumer clocks would hold `&Producer` references to the
//! `Producer` clock object. The existence of outstanding shared borrows would
//! naturally prevent users from calling `Producer` methods taking `&mut self`.
//!
//! Unfortunately, while this approach could work, there is a critical problem
//! with disastrous consequences for ergonomics. To satisfy the Rust borrow
//! checker, `Producer` clock objects *could not be moved* while `&Producer`
//! references were still held by consumer clocks.
//!
//! However, this restriction is unnecessary. A `Producer` clock object is
//! merely a semantic object representing the "idea" of a producer clock. And
//! "borrowing" the producer is not meant to protect memory from corruption.
//! Rather, our goal is only to restrict the `Producer` API, to prevent it from
//! being modified or disabled once it has been connected to a consumer. We
//! don't need to permanently hold the `Producer` object in place to do that.
//!
//! It is possible to build a `clock` API based on the borrow checker, but it
//! would be extremely frustrating to use in practice, because of restrictions
//! on the movement of `Producer` objects.
//!
//! Instead, the `clock` module takes a different approach. It uses type-level
//! programming to track, at compile-time, the number of consumer clocks, N,
//! fed by a particular producer clock. With this approach, we can move
//! `Producer` objects while still making them impossible to modify if N > 0.
//!
//! The following sections will describe the implementation of this strategy.
//!
//! ## Tracking N at compile-time for 1:N clocks
//!
//! We have two specific goals. We need to both track the number of consumer
//! clocks, N, that are actively using a given producer clock. And we need to
//! restrict the producer clock API when N > 0.
//!
//! ### A compile-time counter
//!
//! First, we need to develop some way to track the number of consumer clocks,
//! N, within the type system. To accomplish this, we need both a way to
//! represent N in the type system and a way to increase or decrease N when
//! making or breaking connections in the clock tree.
//!
//! To represent N, we can use type-level, [`Unsigned`] integers from the
//! [`typenum`] crate (i.e. [`U0`], [`U1`], etc). And we can use a type
//! *parameter*, `N`, to represent some unknown, type-level number.
//!
//! Next, we need a way to increase or decrease the type parameter `N`. The
//! [`typenum`] crate provides type aliases [`Add1`] and [`Sub1`] that map from
//! each [`Unsigned`] integer to its successor and predecessor types,
//! respectively. We can leverage these to create our own type with a counter
//! that we [`Increment`] or [`Decrement`] at compile-time. These two traits
//! form the foundation for our strategy for handling 1:N clocks in this module.
//!
//! ### The `Enabled` wrapper
//!
//! Our representation of a 1:N producer clock is [`Enabled<T, N>`], which is a
//! wrapper struct that pairs some *enabled* clock type `T` with a type `N`
//! representing a consumer count. The wrapper restricts access to the
//! underlying clock type, `T`, allowing us to selectively define methods when
//! `N = U0`, that is, when there are no consumers of a given producer clock.
//!
//! The [`Enabled`] type itself implements [`Increment`] and [`Decrement`] as
//! well, which allows type-level transformations to increment or decrement the
//! counter, e.g. `Enabled<T, U0>` to `Enabled<T, U1>`. Such transformations can
//! only be performed within the HAL; so users cannot change the consumer count
//! arbitrarily.
//!
//! ### Acting as a clock `Source`
//!
//! Finally, we need to define some generic interface for interacting with 1:N
//! producer clocks. However, when designing this interface, we need to be
//! careful not to lose information during type-level transformations.
//!
//! In particular, the `Enabled` counter type alone is not enough for proper
//! clock safety. If we used consumer `A` to `Increment` producer `P` from
//! `Enabled<P, U0>` to `Enabled<P, U1>`, but then used consumer `B` to
//! `Decrement` the producer back to `Enabled<P, U0>`, we would leave consumer
//! `A` dangling.
//!
//! To solve this problem, we need some way to guarantee that a given consumer
//! can only `Decrement` the same producer it `Increment`ed. Stated differently,
//! we need a way to track the identity of each consumer's clock source.
//!
//! The [`Source`] trait is designed for this purpose. It marks
//! [`Enabled<T, N>`] producer clocks, and it's associated type, [`Id`], is the
//! identity type that should be stored by consumers.
//!
//! Given that all implementers of `Source` are instances of `Enabled<T, N>`,
//! the naïve choice for [`Source::Id`] would be `T`. However, in a moment, we
//! will see why this choice is not ideal.
//!
//! ### `Id` types
//!
//! Many of the clock types in this module have additional type parameters that
//! track the clock's configuration. For instance, [`Xosc0<M>`] represents one
//! of the external crystal oscillators. Here, the type parameter `M` represents
//! the XOSC's [`Mode`](xosc::Mode), which can either be [`CrystalMode`] or
//! [`ClockMode`]. Accordingly, methods to adjust the crystal current, etc. are
//! only available on `Xosc0<CrystalMode>`.
//!
//! While these type parameters are important and necessary for configuration of
//! a given producer clock, they are not relevant to consumer clocks. A consumer
//! clock does not need to know or care which `Mode` the XOSC is using, but
//! it *does* need to track that its clock [`Source`] is XOSC0.
//!
//! From this, we can see that `Enabled<Xosc0<M>, N>` should not implement
//! `Source` with `Source::Id = Xosc0<M>`, because that would require consumers
//! to needlessly track the XOSC `Mode`.
//!
//! Instead, this module defines a series of `Id` types representing the
//! *identity* of a given clock, rather than the clock itself. This is like the
//! distinction between a passport and a person. A passport identifies a person,
//! regardless of changes to their clothes or hair. The `Id` types serve to
//! erase configuration information, representing only the clock's identity.
//!
//! For `Xosc0<M>`, the corresponding `Id` type is [`Xosc0Id`]. Thus,
//! `Enabled<Xosc0<M>, N>` implements `Source` with `Source::Id = Xosc0Id`.
//!
//! ## Notes on memory safety
//!
//! ### Register interfaces
//!
//! Although HAL users see `Token` types as merely opaque objects, internally
//! they serve a dual purpose as the primary register interface to control the
//! corresponding clock. Moreover, they also fundamentally restructure the way
//! registers are accessed relative to the [PAC].
//!
//! Each of the four PAC clocking structs ([`OSCCTRL`], [`OSC32KCTRL`], [`GCLK`]
//! and [`MCLK`]) is a singleton object that controls a set of MMIO registers.
//! It is impossible to create two instances of any PAC object without `unsafe`.
//! However, each object controls a large set of registers that can be further
//! sub-divided into smaller sets for individual clocks. For example, the
//! [`GCLK`] object controls registers for 12 different clock generators and 48
//! peripheral channel clocks.
//!
//! `Token` types serve to break up the large PAC objects into smaller,
//! more-targetted pieces. And in the process, they also remove the PAC objects'
//! [interior mutability]. But this is only possible because each `Token` is
//! *also* a singleton, and because individual clocks are configured through
//! *mutually exclusive* sets of registers.
//!
//! ### Bus clocks
//!
//! Bus clocks are fundamentally different from the other clock types in this
//! module, because they do not use mutually exclusive registers for
//! configuration. For instance, the registers that control [`Dpll0`] are
//! mutually exclusive to those that control [`Dpll1`], but `ApbClk<Sercom0>`
//! and `ApbClk<Sercom1>` share a single register.
//!
//! This presents a challenge for memory safety, because we need some way to
//! guarantee that there are no data races. For example, if both
//! `ApbClk<Sercom0>` and `ApbClk<Sercom1>` tried to modify the `APBAMASK`
//! register from two different execution contexts, a read/modify/write
//! operation could be preempted, leading to memory corruption.
//!
//! To prevent data races when controlling bus clocks, we introduce two new
//! types to mediate access to the shared registers. For [`AhbClk`]s, this is
//! the [`Ahb`] type; and for [`ApbClk`]s, this is the [`Apb`] type. In a sense,
//! the `Ahb` and `Apb` types represent the actual corresponding buses. Thus,
//! enabling an APB clock by converting an [`ApbToken`] into an `ApbClk`
//! requires exclusive access to the `Apb` in the form of `&mut Apb`.
//!
//! ## Getting started
//!
//! To set up a clock tree, start by trading the [PAC](crate::pac)-level
//! clocking structs for their HAL equivalents. Right now, the only way to do so
//! safely is using the [`clock_system_at_reset`] function, which assumes all
//! clocks are in their default state at power-on reset. If this is not the
//! case, because, for example, a bootloader has modified the clocks, then you
//! may need to manually create the matching configuration using `unsafe` code.
//!
//! ```no_run
//! use atsamd_hal::clock::v2::clock_system_at_reset;
//! use atsamd_hal::pac::Peripherals;
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! ```
//!
//! At this point, you may notice that the function returned three different
//! objects, the [`Buses`], [`Clocks`] and [`Tokens`].
//!
//! The [`Buses`] struct contains the [`Ahb`] and [`Apb`] objects, which
//! represent the corresponding AHB and APB buses. See the [notes on memory
//! safety](self#notes-on-memory-safety) for more details on these types.
//!
//! The [`Clocks`] struct contains all of the clocks that are enabled and
//! running at power-on reset, specifically:
//! - All of the [`AhbClks`]
//! - Some of the [`ApbClks`]
//! - The 48 MHz [`Dfll`], running in open-loop mode, represented as as
//!   `Enabled<Dfll, U1>`. `N = U1` here because [`Gclk0`] consumes it. See
//!   [above](self#tracking-n-at-compile-time-for-1n-clocks) for details on
//!   [`Enabled<T, N>`].
//! - [`Gclk0`], sourced by the `Dfll` and represented as
//!   `Enabled<Gclk0<DfllId>, U1>`. Note the use of [`DfllId`] as an [`Id`
//!   type](self#id-types) here. Although `Gclk0` is not consumed by any clock
//!   represented in this module, it *is* consumed by the processor's main
//!   clock. We represent this by setting `N = U1`, which we use to restrict the
//!   available API. Specifically, [`EnabledGclk0`] has special methods not
//!   available to other [`Gclk`]s.
//! - The [`OscUlp32kBase`] clock, which can act as a [`Source`] for the
//!   [`OscUlp1k`] and [`OscUlp32k`] clocks. It has no consumers at power-on
//!   reset, so it is represented as `Enabled<OscUlp32kBase, U0>`. However, it
//!   can never be disabled, so we provide no `.disable()` method.
//!
//! The [`Tokens`] struct contains all of the available `Token`s, which
//! [represent clocks that are disabled](self#clock-state-machines) at power-on
//! reset. Each `Token` can be exchanged for a corresponding clock object.
//!
//! ## Example clock tree
//!
//! Finally, we will walk through the creation of a simple clock tree to
//! illustrate some of the remaining concepts inherent to this module.
//!
//! Starting from the previous snippet, we have the [`Buses`], [`Clocks`] and
//! [`Tokens`] to work with, and our clock tree at power-on reset looks like
//! this.
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Main clock (48 MHz)
//! ```
//!
//! Our goal will be a clock tree that looks like this:
//!
//! ```text
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//!     └── GCLK0 (100 MHz)
//!         ├── Main clock (100 MHz)
//!         ├── SERCOM0 peripheral clock
//!         └── Output to GPIO pin
//! ```
//!
//! We will use an external crystal oscillator running at 8 MHz to feed a DPLL,
//! which will increase the clock frequency to 100 MHz. Then, we will
//! reconfigure GCLK0 to use the 100 MHz DPLL clock instead of the 48 MHz DFLL
//! clock.
//!
//! First, let's import some of the necessary types. We will see what each type
//! represents in turn.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         pclk::Pclk,
//!         xosc::Xosc,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//! ```
//!
//! To create an instance of [`Xosc`], we will first need to identify which of
//! the two XOSC clocks we will use. Suppose an external crystal is attached to
//! pins `PA14` and `PA15`. These pins feed the XOSC0 clock, so we will want to
//! create an instance of [`Xosc0`]. Note that `Xosc0<M>` is merely an alias for
//! `Xosc<Xosc0Id, M>`. Here, [`Xosc0Id`] represents the
//! [*identity*](self#id-types) of the XOSC0 clock, rather than the clock
//! itself, and `M` represents the XOSC [`Mode`](xosc::Mode).
//!
//! Next, we access the [`Tokens`] struct to extract the corresponding
//! [`XoscToken`] for XOSC0, and we trade the PAC `PORT` struct for the
//! [`gpio::Pins`] struct to access the GPIO pins. We can then call
//! [`Xosc::from_crystal`] to trade the token and [`Pin`]s to yield an instance
//! of [`Xosc0`]. In doing so, we also provide the oscillator frequency.
//!
//! Finally, we can chain a call to the [`Xosc::enable`] method to enable the
//! XOSC and return an instance of [`EnabledXosc0<M, N>`], which is simply an
//! alias for `Enabled<Xosc0<M>, N>`. In this case, we get
//! `EnabledXosc0<CrystalMode, U0>`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! let pins = Pins::new(pac.PORT);
//! let xosc0 = Xosc::from_crystal(
//!     tokens.xosc0,
//!     pins.pa14,
//!     pins.pa15,
//!     8.mhz(),
//! ).enable();
//! ```
//!
//! Next, we want to use a DPLL to multiply the 8 MHz crystal clock up to 100
//! MHz. Once again, we need to decide between two instances of a clock, because
//! each chip has two [`Dpll`]s. This time, however, our decision between
//! [`Dpll0`] and [`Dpll1`] is arbitrary.
//!
//! Also note that, like before, `Dpll0<I>` and `Dpll1<I>` are aliases for
//! `Dpll<Dpll0Id, I>` and `Dpll<Dpll1Id, I>`. [`Dpll0Id`] and [`Dpll1Id`]
//! represent the *identity* of the respective DPLL, while `I` represents the
//! [`Id` type](self#id-types) for the [`Source`] driving the DPLL. In this
//! particular case, we aim to create an instance of `Dpll0<Xosc0Id>`.
//!
//! Only certain clocks can drive the DPLL, so `I` is constrained by the
//! [`DpllSourceId`] trait. Specifically, only the [`Xosc0Id`], [`Xosc1Id`],
//! [`Xosc32kId`] and [`GclkId`] types implement this trait.
//!
//! As before, we access the [`Tokens`] struct and use the corresponding
//! [`DpllToken`] when creating an instance of `Dpll`. However, unlike before,
//! we are creating a new clock-tree relationship that must be tracked by the
//! type system. Because DPLL0 will now consume XOSC0, we must [`Increment`]
//! the [`Enabled`] counter for [`EnabledXosc0`].
//!
//! Thus, to create an instance of `Dpll0<XoscId0>`, we must provide the
//! `EnabledXosc0`, so that its `U0` type parameter can be incremented to `U1`.
//! The `Dpll::from_xosc` method takes ownership of the `EnabledXosc0` and
//! returns it with this modified type parameter.
//!
//! This is the essence of clock safety in this module. Once the counter type
//! has been incremeneted to `U1`, the `EnabledXosc0` can no longer be modified
//! or disabled. All further code can guarantee this invariant is upheld. To
//! modify the `EnabledXosc0`, we would first have to use `Dpll::free_source` to
//! disable the DPLL and [`Decrement`] the counter back to `U0`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! ```
//! Next, we set the DPLL pre-divider and loop divider. We must pre-divide
//! the XOSC clock down from 8 MHz to 2 MHz, so that it is within the valid
//! input frequency range for the DPLL. Then, we set the DPLL loop divider, so
//! that it will multiply the 2 MHz clock by 50 for a 100 MHz output. We do not
//! need fractional mutiplication here, so the fractional loop divider is zero.
//! Finally, we can enable the `Dpll`, yielding an instance of
//! `EnabledDpll0<XoscId0>`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! ```
//!
//! So far, our clock tree looks like this
//!
//! ```text
//! DFLL (48 MHz)
//! └── GCLK0 (48 MHz)
//!     └── Main clock (48 MHz)
//!
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//! ```
//!
//! Our next task will be to swap GCLK0 from the 48 MHz DFLL to the 100 MHz
//! DPLL. To do that, we will use the special [`swap_sources`] method on
//! [`EnabledGclk0`] to change the base clock without disabling GCLK0 or the
//! main clock.
//!
//! This time we will be modifying two [`Enabled`] counters simultaneously.
//! We will [`Decrement`] the [`EnabledDfll`] count from `U1` to `U0`, and
//! we will [`Increment`] the [`EnabledDpll0`] count from `U0` to `U1`.
//! Again, we need to provide both the DFLL and DPLL clocks, so that their
//! type parameters can be changed.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! # let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! ```
//!
//! At this point, the DFLL is completely unused, so it can be disbled and
//! deconstructed, leaving only the [`DfllToken`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! # let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! # let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! let dfll_token = dfll.disable().free();
//! ```
//!
//! Our clock tree now looks like this:
//!
//! ```text
//! XOSC0 (8 MHz)
//! └── DPLL0 (100 MHz)
//!     └── GCLK0 (100 MHz)
//!         └── Main clock (100 MHz)
//! ```
//!
//! We have the clocks set up, but we're not using them for anything other than
//! the main clock. Our final steps will create SERCOM APB and peripheral
//! clocks and will output the raw GCLK0 to a GPIO pin.
//!
//! To enable the APB clock for SERCOM0, we must access the [`Apb`] bus struct.
//! We provide an [`ApbToken`] to the [`Apb::enable`] method and receive an
//! [`ApbClk`] in return. APB clocks are [1:1 clocks](self#clock-relationships),
//! so the `ApbClk` is not wrapped with [`Enabled`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! # let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! # let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! # let dfll_token = dfll.disable().free();
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! ```
//!
//! To enable a peripheral channel clock for SERCOM0, we must provide the
//! corresponding [`PclkToken`], as well as the instance of [`EnabledGclk0`], so
//! that its counter can be incremented. The resulting clock has the type
//! `Pclk<Sercom0, Gclk0Id>`.
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         pclk::Pclk,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! # let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! # let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! # let dfll_token = dfll.disable().free();
//! # let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//! ```
//!
//! Like [`Dpll<D, I>`], [`Pclk<P, I>`] also takes two type parameters. The
//! first represents the corresponding peripheral, while the second is again an
//! [`Id` type](self#id-types) representing the [`Source`] driving the [`Pclk`],
//! which is restricted by the [`PclkSourceId`] trait. Because peripheral
//! channel clocks can only be driven by GCLKs, [`PclkSourceId`] is effectively
//! synonymous with the [`GclkId`] trait.
//!
//! Finally, we would like to output GCLK0 to a GPIO pin. Doing so takes a
//! slightly different approach. This time, we provide a GPIO [`Pin`] to the
//! [`Gclk`], which creates a [`GclkOut`] and [`Increment`]s the consumer count
//! for [`EnabledGclk0`].
//!
//! ```no_run
//! # use atsamd_hal::{
//! #     clock::v2::{
//! #         clock_system_at_reset,
//! #         dpll::Dpll,
//! #         pclk::Pclk,
//! #         xosc::Xosc,
//! #     },
//! #     gpio::Pins,
//! #     pac::Peripherals,
//! #     time::U32Ext,
//! # };
//! # let mut pac = Peripherals::take().unwrap();
//! # let (mut buses, clocks, tokens) = clock_system_at_reset(
//! #     pac.OSCCTRL,
//! #     pac.OSC32KCTRL,
//! #     pac.GCLK,
//! #     pac.MCLK,
//! #     &mut pac.NVMCTRL,
//! # );
//! # let pins = Pins::new(pac.PORT);
//! # let xosc0 = Xosc::from_crystal(
//! #     tokens.xosc0,
//! #     pins.pa14,
//! #     pins.pa15,
//! #     8.mhz(),
//! # ).enable();
//! # let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! # let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! # let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! # let dfll_token = dfll.disable().free();
//! # let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! # let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//! let (gclk0, gclk0_out) = gclk0.enable_gclk_out(pins.pb14);
//! ```
//!
//! We have arrived at our final, desired clock tree. Putting the whole example
//! together, we get
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{
//!         clock_system_at_reset,
//!         dpll::Dpll,
//!         pclk::Pclk,
//!         xosc::Xosc,
//!     },
//!     gpio::Pins,
//!     pac::Peripherals,
//!     time::U32Ext,
//! };
//!
//! let mut pac = Peripherals::take().unwrap();
//! let (mut buses, clocks, tokens) = clock_system_at_reset(
//!     pac.OSCCTRL,
//!     pac.OSC32KCTRL,
//!     pac.GCLK,
//!     pac.MCLK,
//!     &mut pac.NVMCTRL,
//! );
//! let pins = Pins::new(pac.PORT);
//! let xosc0 = Xosc::from_crystal(
//!     tokens.xosc0,
//!     pins.pa14,
//!     pins.pa15,
//!     8.mhz(),
//! )
//! .enable();
//! let (dpll0, xosc0) = Dpll::from_xosc(tokens.dpll0, xosc0);
//! let dpll0 = dpll0.prediv(4).loop_div(50, 0).enable();
//! let (gclk0, dfll, dpll0) = clocks.gclk0.swap_sources(clocks.dfll, dpll0);
//! let dfll_token = dfll.disable().free();
//! let apb_sercom0 = buses.apb.enable(tokens.apbs.sercom0);
//! let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, gclk0);
//! let (gclk0, gclk0_out) = gclk0.enable_gclk_out(pins.pb14);
//! ```
//!
//! [PAC]: crate::pac
//! [`OSCCTRL`]: crate::pac::OSCCTRL
//! [`OSC32KCTRL`]: crate::pac::OSC32KCTRL
//! [`GCLK`]: crate::pac::GCLK
//! [`MCLK`]: crate::pac::MCLK
//! [`Peripherals::steal`]: crate::pac::Peripherals::steal
//!
//! [`Ahb`]: ahb::Ahb
//! [`AhbClk`]: ahb::AhbClk
//! [`AhbClk<A>`]: ahb::AhbClk
//! [`AhbClks`]: ahb::AhbClks
//!
//! [`Apb`]: apb::Apb
//! [`Apb::enable`]: apb::Apb::enable
//! [`ApbClk`]: apb::ApbClk
//! [`ApbClk<A>`]: apb::ApbClk
//! [`ApbClk::enable`]: apb::ApbClk::enable
//! [`ApbClks`]: apb::ApbClks
//! [`ApbToken`]: apb::ApbToken
//!
//! [`Dfll`]: dfll::Dfll
//! [`Dfll<M>`]: dfll::Dfll
//! [`DfllId`]: dfll::DfllId
//! [`DfllToken`]: dfll::DfllToken
//! [`EnabledDfll`]: dfll::EnabledDfll
//!
//! [`Dpll`]: dpll::Dpll
//! [`Dpll<D, I>`]: dpll::Dpll
//! [`Dpll0`]: dpll::Dpll0
//! [`Dpll1`]: dpll::Dpll1
//! [`Dpll0Id`]: dpll::Dpll0Id
//! [`Dpll1Id`]: dpll::Dpll1Id
//! [`DpllSourceId`]: dpll::DpllSourceId
//! [`DpllToken`]: dpll::DpllToken
//! [`EnabledDpll0`]: dpll::EnabledDpll0
//!
//! [`Gclk0`]: gclk::Gclk0
//! [`GclkId`]: gclk::GclkId
//! [`EnabledGclk0`]: gclk::EnabledGclk0
//! [`swap_sources`]: gclk::EnabledGclk0::swap_sources
//! [`GclkOut`]: gclk::GclkOut
//!
//! [`OscUlp32kBase`]: osculp32k::OscUlp32kBase
//! [`OscUlp1k`]: osculp32k::OscUlp1k
//! [`OscUlp32k`]: osculp32k::OscUlp32k
//!
//! [`Pclk`]: pclk::Pclk
//! [`Pclk<P, I>`]: pclk::Pclk
//! [`Pclk::enable`]: pclk::Pclk::enable
//! [`Pclk::disable`]: pclk::Pclk::disable
//! [`PclkSourceId`]: pclk::PclkSourceId
//! [`PclkToken`]: pclk::PclkToken
//!
//! [`RtcOsc`]: rtcosc::RtcOsc
//!
//! [`Xosc`]: xosc::Xosc
//! [`Xosc::from_crystal`]: xosc::Xosc::from_crystal
//! [`Xosc::enable`]: xosc::Xosc::enable
//! [`Xosc0`]: xosc::Xosc0
//! [`Xosc0<M>`]: xosc::Xosc0
//! [`Xosc0Id`]: xosc::Xosc0Id
//! [`Xosc1Id`]: xosc::Xosc1Id
//! [`XoscToken`]: xosc::XoscToken
//! [`EnabledXosc0`]: xosc::EnabledXosc0
//! [`EnabledXosc0<M, N>`]: xosc::EnabledXosc0
//! [`CrystalMode`]: xosc::CrystalMode
//! [`ClockMode`]: xosc::ClockMode
//!
//! [`Xosc32kId`]: xosc32k::Xosc32kId
//!
//! [type-level]: crate::typelevel
//! [`Increment`]: crate::typelevel::Increment
//! [`Decrement`]: crate::typelevel::Decrement
//!
//! [`Id`]: Source::Id
//!
//! [`gpio::Pins`]: crate::gpio::Pins
//! [`Pin`]: crate::gpio::Pin
//!
//! [`U1`]: typenum::U1
//! [`Add1`]: typenum::Add1
//! [`Sub1`]: typenum::Sub1
//! [`Unsigned`]: typenum::Unsigned
//!
//! [interior mutability]: https://doc.rust-lang.org/reference/interior-mutability.html

use typenum::U0;

use crate::time::Hertz;
use crate::typelevel::{PrivateDecrement, PrivateIncrement, Sealed};

pub mod ahb;
pub mod apb;
pub mod dfll;
pub mod dpll;
pub mod gclk;
pub mod osculp32k;
pub mod pclk;
pub mod rtcosc;
pub mod types;
pub mod xosc;
pub mod xosc32k;

mod reset;
pub use reset::*;

// `Token` types and memory safety
//
// Each of the PAC [`Peripherals`] is a zero-sized, singleton struct that
// mediates access to the MMIO hardware registers. It is not possible to create
// two instances of any peripheral without causing a run-time panic. These
// structs implement [`Deref`] by conjuring a pointer to the corresponding
// register block, and each register within the block is represented by a
// `vcell::VolatileCell`. Because each register is wrapped in a `VolatileCell`,
// it is safe to both read and write them through shared references. However,
// because a read/modify/write operation is not atomic, the [`Peripherals`]
// structs do not implement [`Sync`].
//
// This is a reasonable approach for the PAC, since it is generated from an
// SVD file. However, it is not the ideal structure for our HAL API. In
// particular, each [`Peripherals`] struct represents an entire peripheral,
// rather than a particular functional unit. In the HAL, we want our API to
// focus on functional units, so we need to define our own abstraction for
// registers, which will involve `unsafe` code.
//
// In the `clock` module, we represent each functional unit with a
// corresponding `Token` type. Just like the [`Peripherals`], each `Token` type
// is meant to be a singleton. However, unlike the PAC, we do not have to
// allow users to create `Token`s directly. Instead, we can have users exchange
// [`Peripherals`] for the `Token`s. Because each PAC struct is a singleton, we
// can guarantee each `Token` will be a singleton as well. With this approach,
// we don't need to implement our own run-time panicking; we simply extend the
// existing guarantees of the PAC.
//
// To implement a memory safe API, we must ensure that all `Token` types access
// mutually exclusive sets of registers. In that way, we guarantee no two
// `Token` types can access the same register. Moreover, in contrast to the PAC
// [`Peripherals`], we can make our `Token`s [`Sync`] if we remove all interior
// mutability and guarantee that writing or modifying a register requires
// ownership or an `&mut` reference.
//
// Thus, our `Token`-based API should be memory safe if we always obey the
// following requirements:
//   - It should be `unsafe` to create a `Token` type unless it is created in
//     exchange for the corresponding PAC peripheral struct.
//   - Each `Token` type should have access to a mutually exclusive set of
//     registers relative to the other `Token`s.
//   - Writing or modifying a register should always require ownership of, or an
//     `&mut` reference to, the corresponding `Token`.
//   - When conjuring references to PAC registers or register blocks, we should
//     *only* use shared, `&` references. There is no need to use exclusive,
//     `&mut` references, because each register is wrapped in a `VolatileCell`.
//     Moreover, using `&mut` references could cause UB, if we accidentally
//     create two simultaneous references to the same register block from
//     different `Tokens`.
//
// [`Peripherals`]: crate::pac::Peripherals
// [`Deref`]: core::ops::Deref

/// Marks [`Enabled`] 1:N producer clocks that can act as a clock source
///
/// Implementers of this type act as producer clocks and feed consumer clocks in
/// the clock tree. All implementors are [`Enabled`], 1:N clocks. The `Id`
/// associated type maps to the corresponding [`Id` type](self#id-types) of the
/// implementer.
///
/// See the documentation on [`Source` clocks](self#acting-as-a-clock-source)
/// for more details.
pub trait Source: Sealed {
    /// Corresponding `Id` type for the implementer
    ///
    /// A given implementer of [`Source`] might have type parameters
    /// representing its configuration. For instance, [`EnabledXosc0<M>`] has a
    /// type parameter to track its [`Mode`]. However, a consumer clock
    /// typically does not care about such configuration. It only needs to
    /// know *which* upstream clock is its [`Source`].
    ///
    /// `Id` types exist to fill this role. They represent the *identity* of a
    /// given clock, regardless of any configuration. This is like the
    /// distinction between a passport and a person. A passport identifies a
    /// person, regardless of changes to their clothes or hair.
    ///
    /// Thus, [`EnabledXosc0<M>`] implements [`Source`] with `Id = `[`Xosc0Id`],
    /// regardless of `M`.
    ///
    /// See the documentation on [`Id` types](self#id-types) for more details.
    ///
    /// [`EnabledXosc0<M>`]: xosc::EnabledXosc0
    /// [`Mode`]: xosc::Mode
    /// [`Xosc0Id`]: xosc::Xosc0Id
    type Id;

    /// Return the frequency of the clock source
    fn freq(&self) -> Hertz;
}

/// An enabled, 1:N clock with a compile-time counter for N
///
/// This struct is a wrapper around other clock types from this module. It
/// represents a clock, `T`, that has been enabled, and it maintains a
/// compile-time counter, `N`, of its consumer clocks in the clock tree.
///
/// Compile-time counting allows the API to restrict when clocks may be modified
/// or disabled. For example, `Enabled` clocks can only be disabled when their
/// counter is [`U0`].
///
/// The type-level counter is implemented using [`Unsigned`] integers from
/// the [`typenum`] crate, and it is modified using the [`Increment`] and
/// [`Decrement`] traits.
///
/// See the [`Enabled` wrapper documentation](self#the-enabled-wrapper) for more
/// details.
///
/// [`EnabledGclk0`]: gclk::EnabledGclk0
/// [`Increment`]: crate::typelevel::Increment
/// [`Decrement`]: crate::typelevel::Decrement
/// [`Unsigned`]: typenum::Unsigned
pub struct Enabled<T, N = U0>(pub(crate) T, N);

impl<T, N> Sealed for Enabled<T, N> {}

impl<T, N: Default> Enabled<T, N> {
    #[inline]
    pub(crate) fn new(t: T) -> Self {
        Enabled(t, N::default())
    }
}

impl<T, N: PrivateIncrement> PrivateIncrement for Enabled<T, N> {
    type Inc = Enabled<T, N::Inc>;

    #[inline]
    fn inc(self) -> Self::Inc {
        Enabled(self.0, self.1.inc())
    }
}

impl<T, N: PrivateDecrement> PrivateDecrement for Enabled<T, N> {
    type Dec = Enabled<T, N::Dec>;

    #[inline]
    fn dec(self) -> Self::Dec {
        Enabled(self.0, self.1.dec())
    }
}
