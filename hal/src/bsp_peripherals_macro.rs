//==============================================================================
//  bsp_peripherals
//==============================================================================

/// # Helper macro to give meaningful names to peripherals
///
/// The [`pac::Peripherals`](crate::pac::Peripherals) struct refers to each
/// peripheral by its datasheet name. However, in the context of a BSP,
/// peripherals can often be given more meaningful names. This macro gives BSP
/// authors a convenient way to provide custom names for each peripheral.
///
/// ## Calling the macro
///
/// The `bsp_peripherals!` macro takes a series of `PERIPHERAL` blocks. Each
/// block starts with a peripheral name from the `pac::Peripherals` struct and
/// is delimited by curly brackets. The brackets should contain a
/// comma-separated list of alternate names for the peripheral, in `PascalCase`.
/// The example below defines `Uart` as an alias for the `SERCOM2` peripheral
/// and `DisplaySpi` as an alias for the `SERCOM4` peripheral.
///
/// ```
/// atsamd_hal::bsp_peripherals!(
///     SERCOM2 { Uart }
///     SERCOM4 { DisplaySpi }
/// );
/// ```
///
/// The macro defines a type alias for each name within curly brackets. The
/// example above would exand to
///
/// ```
/// pub type Uart = pac::SERCOM2;
/// pub type DisplaySpi = pac::SERCOM4;
/// ```
///
/// While these type aliases are useful, they do not completely separate the
/// mapping of each peripheral from the corresponding code.
///
/// In Rust, each struct field can only have one name. Fields of the
/// `pac::Peripherals` struct are named according to their datasheet identifier.
/// Consequently, you must access the peripheral using that name. For example,
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let uart = peripherals.SERCOM2;
/// ```
///
/// To provide access to the same struct field using *different* names, the
/// `bsp_peripherals!` macro defines another macro, `periph_alias!`. Based on
/// the example above, we could use the `periph_alias!` macro to access the
/// `SERCOM2` peripheral without ever referring to it directly.
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let uart = periph_alias!(peripherals.uart);
/// ```
///
/// Note that the `Uart` alias was translated to `snake_case` when accessing
/// the `pac::Peripherals` field. The same is true for the `DisplaySpi` alias.
///
/// ```
/// let mut peripherals = pac::Peripherals::take().unwrap();
/// let display_spi = periph_alias!(peripherals.display_spi);
/// ```
///
/// ## Attributes and documentation
///
/// BSP authors can also add attributes to various parts of the macro
/// declaration. Attributes can be added to the entire `PERIPHERAL` block. These
/// attributes will be propagated to every use of the corresponding
/// `PERIPHERAL`. Attributes applied to each alias, on the other hand, will only
/// be propagated to items specific to that alias, like the corresponding type
/// alias.
///
/// ```
/// atsamd_hal::bsp_peripherals!(
///     SERCOM2 {
///         #[cfg(feature = "uart")]
///         Uart
///     }
///     #[cfg(feature = "display")]
///     SERCOM4 { DisplaySpi }
/// );
/// ```
#[macro_export]
macro_rules! bsp_peripherals {
    (
        $(
            $( #[$peripheral_cfg:meta] )*
            $PERIPHERAL:ident {
                $(
                    $( #[$alias_cfg:meta] )*
                    $Alias:ident $(,)?
                )+
            } $(,)?
        )+
    ) => {
        $(
            $( #[$peripheral_cfg] )*
            $crate::__create_peripheral_aliases!(
                $PERIPHERAL
                $(
                    $( #[$alias_cfg] )*
                    $Alias
                )+
            );
        )+

        $crate::__define_periph_alias_macro!(
            $(
                $(
                    $( #[$alias_cfg] )*
                    ($PERIPHERAL, $Alias)
                )+
            )+
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __create_peripheral_aliases {
    (
        $PERIPHERAL:ident
        $(
            $( #[$attr:meta] )*
            $Alias:ident
        )+
    ) => {
        $crate::paste::paste! {
            $(
                $( #[$attr] )*
                #[
                    doc = "Alias for the "
                    "[`" $PERIPHERAL "`](atsamd_hal::pac::" $PERIPHERAL ") "
                    "peripheral"
                ]
                pub type $Alias = atsamd_hal::pac::$PERIPHERAL;
            )+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __define_periph_alias_macro {
    (
        $(
            $( #[$attr:meta] )*
            ($PERIPHERAL:ident, $Alias:ident)
        )+
    ) => {
        $crate::paste::paste! {
            /// Refer to fields of the [`Peripherals`](atsamd_hal::pac::Peripherals)
            /// struct by alternate names
            ///
            /// This macro can be used to access fields of the `Peripherals`
            /// struct by alternate names. The available aliases are:
            ///
            #[ doc =
                $(
                    "    - [`" $PERIPHERAL "`](atsamd_hal::pac::" $PERIPHERAL ") \
                    can be refered to with the type alias [`" $Alias "`] and \
                    accessed as the field name `" $Alias:snake "`\n"
                )+
            ]
            ///
            /// For example. suppose `display_spi` were an alternate name for
            /// the `SERCOM4` peripheral. You could use the `periph_alias!`
            /// macro to access it like this:
            ///
            /// ```
            /// let mut peripherals = pac::Peripherals::take().unwrap();
            /// // Replace this
            /// let display_spi = peripherals.SERCOM4;
            /// // With this
            /// let display_spi = periph_alias!(peripherals.display_spi);
            /// ```
            #[macro_export]
            macro_rules! periph_alias {
                $(
                    ( $peripherals:ident . [<$Alias:snake>] ) => {
                        {
                            $( #[$attr] )*
                            macro_rules! [<peripheral_alias_ $Alias:snake>] {
                                () => { $peripherals.$PERIPHERAL };
                            }
                            [<peripheral_alias_ $Alias:snake>]!()
                        }
                    };
                )+
            }
        }
    }
}
