//! Async APIs

pub mod interrupts;

// #[cfg(feature = "samd11")]
// #[path = "irqs_samd11.rs"]
// mod irqs;
//
// #[cfg(feature = "samd21")]
// #[path = "irqs_samd21.rs"]
// mod irqs;
//
// #[cfg(feature = "samd51")]
// #[path = "irqs_thumbv7em.rs"]
// mod irqs;
//
// pub mod interrupt {
//     pub use super::irqs::*;
//     pub use critical_section::{CriticalSection, Mutex};
//     pub use embassy::interrupt::{declare, take, Interrupt};
//
//     #[cfg(any(feature = "samd11", feature = "samd21"))]
//     /// thumbv6m targets have 4 different interrupt priority levels
//     pub use embassy_hal_common::interrupt::Priority2 as Priority;
//
//     #[cfg(feature = "min-samd51g")]
//     /// thumbv7em targets have 8 different interrupt priority levels
//     pub use embassy_hal_common::interrupt::Priority3 as Priority;
// }

/// Macro to bind interrupts to handlers.
///
/// This defines the right interrupt handlers, and creates a unit struct (like
/// `struct Irqs;`) and implements the right [`Binding`]s for it. You can pass
/// this struct to drivers to prove at compile-time that the right interrupts
/// have been bound.
///
/// # Example
/// ```
/// use atsamd_hal::bind_interrupts;
///
/// bind_interrupts!(struct Irqs {
///     SERCOM0 => sercom::InterruptHandler;
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
				unsafe impl $crate::async_hal::interrupts::Binding<$crate::async_hal::interrupts::$irq, $handler> for $name {}
			)*
		)*
	};
}

pub mod timer;
