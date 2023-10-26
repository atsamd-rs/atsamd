use super::{marker, BitM, FieldSpec, RegisterSpec, Unsafe, Writable};
pub struct R<REG: RegisterSpec> {
    pub(crate) bits: REG::Ux,
    pub(super) _reg: marker::PhantomData<REG>,
}
pub struct W<REG: RegisterSpec> {
    #[doc = "Writable bits"]
    pub(crate) bits: REG::Ux,
    pub(super) _reg: marker::PhantomData<REG>,
}
pub struct FieldReader<FI = u8>
where
    FI: FieldSpec,
{
    pub(crate) bits: FI::Ux,
    _reg: marker::PhantomData<FI>,
}
impl<FI: FieldSpec> FieldReader<FI> {
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn new(bits: FI::Ux) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
pub struct BitReader<FI = bool> {
    pub(crate) bits: bool,
    _reg: marker::PhantomData<FI>,
}
impl<FI> BitReader<FI> {
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn new(bits: bool) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
pub struct FieldWriter<'a, REG, const WI: u8, const O: u8, FI = u8, Safety = Unsafe>
where
    REG: Writable + RegisterSpec,
    FI: FieldSpec,
{
    pub(crate) w: &'a mut W<REG>,
    _field: marker::PhantomData<(FI, Safety)>,
}
impl<'a, REG, const WI: u8, const O: u8, FI, Safety> FieldWriter<'a, REG, WI, O, FI, Safety>
where
    REG: Writable + RegisterSpec,
    FI: FieldSpec,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut W<REG>) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
pub struct BitWriter<'a, REG, const O: u8, FI = bool, M = BitM>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    pub(crate) w: &'a mut W<REG>,
    _field: marker::PhantomData<(FI, M)>,
}
impl<'a, REG, const O: u8, FI, M> BitWriter<'a, REG, O, FI, M>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut W<REG>) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
