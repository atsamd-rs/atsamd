//! Module supporting type-level programming

mod private {
    /// Super trait used to mark traits with an exhaustive set of
    /// implementations
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

/// Type-level version of the `None` variant
pub struct NoneT;
impl Sealed for NoneT {}
