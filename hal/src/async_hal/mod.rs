//! Async APIs

pub mod interrupts;
pub mod timer;

/// Macro to bind interrupts to handlers.
///
/// This defines the right interrupt handlers, creates a unit struct (like
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

        unsafe impl $crate::async_hal::interrupts::Binding<$crate::async_hal::interrupts::$int_source, $handler> for $name {}
    };
}
