//! Module supporting type-level programming

mod private {
    /// Super trait used to mark traits with an exhaustive set of
    /// implementations
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

/// Type-level version of the [None](core::option::Option::None) variant
pub struct NoneT;
impl Sealed for NoneT {}

/// Marker trait for type identity
///
/// This trait must be implemented with `Self::Type == Self`. When used as a
/// trait bound with a constrained `Type`, i.e.
/// `where T: Is<Type = SpecificType>`, it allows easy conversion between the
/// generic type `T` and the `SpecificType` using [`Into`], [`AsRef`] and
/// [`AsMut`].
///
/// This trait is used throughout the HAL to create various `Any*` meta-types.
pub trait Is
where
    Self: From<IsType<Self>>,
    Self: Into<IsType<Self>>,
    Self: AsRef<IsType<Self>>,
    Self: AsMut<IsType<Self>>,
    IsType<Self>: AsRef<Self>,
    IsType<Self>: AsMut<Self>,
{
    type Type;
}

/// Alias for [`Is`]`::Type`
pub type IsType<T> = <T as Is>::Type;

/// Blanket implementation
impl<T> Is for T
where
    T: AsRef<T> + AsMut<T>,
{
    type Type = T;
}
