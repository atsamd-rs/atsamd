use core::marker;
#[doc = " Raw register type (`u8`, `u16`, `u32`, ...)"]
pub trait RawReg:
    Copy
    + Default
    + From<bool>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOrAssign
    + core::ops::BitAndAssign
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u8, Output = Self>
{
    #[doc = " Mask for bits of width `WI`"]
    fn mask<const WI: u8>() -> Self;
    #[doc = " Mask for bits of width 1"]
    fn one() -> Self;
}
macro_rules! raw_reg {
    ($ U : ty , $ size : literal , $ mask : ident) => {
        impl RawReg for $U {
            #[inline(always)]
            fn mask<const WI: u8>() -> Self {
                $mask::<WI>()
            }
            #[inline(always)]
            fn one() -> Self {
                1
            }
        }
        const fn $mask<const WI: u8>() -> $U {
            <$U>::MAX >> ($size - WI)
        }
    };
}
raw_reg!(u8, 8, mask_u8);
raw_reg!(u16, 16, mask_u16);
raw_reg!(u32, 32, mask_u32);
raw_reg!(u64, 64, mask_u64);
#[doc = " Raw register type"]
pub trait RegisterSpec {
    #[doc = " Raw register type (`u8`, `u16`, `u32`, ...)."]
    type Ux: RawReg;
}
#[doc = " Trait implemented by readable registers to enable the `read` method."]
#[doc = ""]
#[doc = " Registers marked with `Writable` can be also be `modify`'ed."]
pub trait Readable: RegisterSpec {
    #[doc = " Result from a call to `read` and argument to `modify`."]
    type Reader: From<R<Self>> + core::ops::Deref<Target = R<Self>>;
}
#[doc = " Trait implemented by writeable registers."]
#[doc = ""]
#[doc = " This enables the  `write`, `write_with_zero` and `reset` methods."]
#[doc = ""]
#[doc = " Registers marked with `Readable` can be also be `modify`'ed."]
pub trait Writable: RegisterSpec {
    #[doc = " Writer type argument to `write`, et al."]
    type Writer: From<W<Self>> + core::ops::DerefMut<Target = W<Self>>;
    #[doc = " Specifies the register bits that are not changed if you pass `1` and are changed if you pass `0`"]
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux;
    #[doc = " Specifies the register bits that are not changed if you pass `0` and are changed if you pass `1`"]
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux;
}
#[doc = " Reset value of the register."]
#[doc = ""]
#[doc = " This value is the initial value for the `write` method. It can also be directly written to the"]
#[doc = " register by using the `reset` method."]
pub trait Resettable: RegisterSpec {
    #[doc = " Reset value of the register."]
    const RESET_VALUE: Self::Ux;
    #[doc = " Reset value of the register."]
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        Self::RESET_VALUE
    }
}
#[doc = " This structure provides volatile access to registers."]
#[repr(transparent)]
pub struct Reg<REG: RegisterSpec> {
    register: vcell::VolatileCell<REG::Ux>,
    _marker: marker::PhantomData<REG>,
}
unsafe impl<REG: RegisterSpec> Send for Reg<REG> where REG::Ux: Send {}
impl<REG: RegisterSpec> Reg<REG> {
    #[doc = " Returns the underlying memory address of register."]
    #[doc = ""]
    #[doc = " ```ignore"]
    #[doc = " let reg_ptr = periph.reg.as_ptr();"]
    #[doc = " ```"]
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut REG::Ux {
        self.register.as_ptr()
    }
}
impl<REG: Readable> Reg<REG> {
    #[doc = " Reads the contents of a `Readable` register."]
    #[doc = ""]
    #[doc = " You can read the raw contents of a register by using `bits`:"]
    #[doc = " ```ignore"]
    #[doc = " let bits = periph.reg.read().bits();"]
    #[doc = " ```"]
    #[doc = " or get the content of a particular field of a register:"]
    #[doc = " ```ignore"]
    #[doc = " let reader = periph.reg.read();"]
    #[doc = " let bits = reader.field1().bits();"]
    #[doc = " let flag = reader.field2().bit_is_set();"]
    #[doc = " ```"]
    #[inline(always)]
    pub fn read(&self) -> REG::Reader {
        REG::Reader::from(R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        })
    }
}
impl<REG: Resettable + Writable> Reg<REG> {
    #[doc = " Writes the reset value to `Writable` register."]
    #[doc = ""]
    #[doc = " Resets the register to its initial state."]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(REG::RESET_VALUE)
    }
    #[doc = " Writes bits to a `Writable` register."]
    #[doc = ""]
    #[doc = " You can write raw bits into a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });"]
    #[doc = " ```"]
    #[doc = " or write only the fields you need:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " or an alternative way of saying the same:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| {"]
    #[doc = "     w.field1().bits(newfield1bits);"]
    #[doc = "     w.field2().set_bit();"]
    #[doc = "     w.field3().variant(VARIANT)"]
    #[doc = " });"]
    #[doc = " ```"]
    #[doc = " In the latter case, other fields will be set to their reset value."]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::RESET_VALUE & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                    | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}
impl<REG: Writable> Reg<REG> {
    #[doc = " Writes 0 to a `Writable` register."]
    #[doc = ""]
    #[doc = " Similar to `write`, but unused bits will contain 0."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Unsafe to use with registers which don't allow to write 0."]
    #[inline(always)]
    pub unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::Ux::default(),
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}
impl<REG: Readable + Writable> Reg<REG> {
    #[doc = " Modifies the contents of the register by reading and then writing it."]
    #[doc = ""]
    #[doc = " E.g. to do a read-modify-write sequence to change parts of a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|r, w| unsafe { w.bits("]
    #[doc = "    r.bits() | 3"]
    #[doc = " ) });"]
    #[doc = " ```"]
    #[doc = " or"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|_, w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " or an alternative way of saying the same:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|_, w| {"]
    #[doc = "     w.field1().bits(newfield1bits);"]
    #[doc = "     w.field2().set_bit();"]
    #[doc = "     w.field3().variant(VARIANT)"]
    #[doc = " });"]
    #[doc = " ```"]
    #[doc = " Other fields will have the value they had before the call to `modify`."]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&REG::Reader, &'w mut REG::Writer) -> &'w mut W<REG>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &REG::Reader::from(R {
                    bits,
                    _reg: marker::PhantomData,
                }),
                &mut REG::Writer::from(W {
                    bits: bits & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                        | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                    _reg: marker::PhantomData,
                }),
            )
            .bits,
        );
    }
}
#[doc = " Register reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`"]
#[doc = " method."]
pub struct R<REG: RegisterSpec + ?Sized> {
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}
impl<REG: RegisterSpec> R<REG> {
    #[doc = " Reads raw bits from register."]
    #[inline(always)]
    pub fn bits(&self) -> REG::Ux {
        self.bits
    }
}
impl<REG: RegisterSpec, FI> PartialEq<FI> for R<REG>
where
    REG::Ux: PartialEq,
    FI: Copy,
    REG::Ux: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&REG::Ux::from(*other))
    }
}
#[doc = " Register writer."]
#[doc = ""]
#[doc = " Used as an argument to the closures in the `write` and `modify` methods of the register."]
pub struct W<REG: RegisterSpec + ?Sized> {
    #[doc = "Writable bits"]
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}
impl<REG: RegisterSpec> W<REG> {
    #[doc = " Writes raw bits to the register."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Read datasheet or reference manual to find what values are allowed to pass."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: REG::Ux) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc(hidden)]
pub struct FieldReaderRaw<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}
impl<U, FI> FieldReaderRaw<U, FI>
where
    U: Copy,
{
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
#[doc(hidden)]
pub struct BitReaderRaw<T> {
    pub(crate) bits: bool,
    _reg: marker::PhantomData<T>,
}
impl<FI> BitReaderRaw<FI> {
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
#[doc = " Field reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of fields."]
pub type FieldReader<U, FI> = FieldReaderRaw<U, FI>;
#[doc = " Bit-wise field reader"]
pub type BitReader<FI> = BitReaderRaw<FI>;
impl<U, FI> FieldReader<U, FI>
where
    U: Copy,
{
    #[doc = " Reads raw bits from field."]
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}
impl<U, FI> PartialEq<FI> for FieldReader<U, FI>
where
    U: PartialEq,
    FI: Copy,
    U: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&U::from(*other))
    }
}
impl<FI> PartialEq<FI> for BitReader<FI>
where
    FI: Copy,
    bool: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&bool::from(*other))
    }
}
impl<FI> BitReader<FI> {
    #[doc = " Value of the field as raw bits."]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = " Returns `true` if the bit is clear (0)."]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = " Returns `true` if the bit is set (1)."]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc(hidden)]
pub struct Safe;
#[doc(hidden)]
pub struct Unsafe;
#[doc(hidden)]
pub struct FieldWriterRaw<'a, U, REG, N, FI, Safety, const WI: u8, const O: u8>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(N, FI, Safety)>,
}
impl<'a, U, REG, N, FI, Safety, const WI: u8, const O: u8>
    FieldWriterRaw<'a, U, REG, N, FI, Safety, WI, O>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
#[doc(hidden)]
pub struct BitWriterRaw<'a, U, REG, FI, M, const O: u8>
where
    REG: Writable + RegisterSpec<Ux = U>,
    bool: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(FI, M)>,
}
impl<'a, U, REG, FI, M, const O: u8> BitWriterRaw<'a, U, REG, FI, M, O>
where
    REG: Writable + RegisterSpec<Ux = U>,
    bool: From<FI>,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
#[doc = " Write field Proxy with unsafe `bits`"]
pub type FieldWriter<'a, U, REG, N, FI, const WI: u8, const O: u8> =
    FieldWriterRaw<'a, U, REG, N, FI, Unsafe, WI, O>;
#[doc = " Write field Proxy with safe `bits`"]
pub type FieldWriterSafe<'a, U, REG, N, FI, const WI: u8, const O: u8> =
    FieldWriterRaw<'a, U, REG, N, FI, Safe, WI, O>;
impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriter<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    #[doc = " Field width"]
    pub const WIDTH: u8 = WI;
}
impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriterSafe<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    N: From<FI>,
{
    #[doc = " Field width"]
    pub const WIDTH: u8 = WI;
}
macro_rules! bit_proxy {
    ($ writer : ident , $ mwv : ident) => {
        #[doc(hidden)]
        pub struct $mwv;
        #[doc = " Bit-wise write field proxy"]
        pub type $writer<'a, U, REG, FI, const O: u8> = BitWriterRaw<'a, U, REG, FI, $mwv, O>;
        impl<'a, U, REG, FI, const OF: u8> $writer<'a, U, REG, FI, OF>
        where
            REG: Writable + RegisterSpec<Ux = U>,
            bool: From<FI>,
        {
            #[doc = " Field width"]
            pub const WIDTH: u8 = 1;
        }
    };
}
macro_rules! impl_bit_proxy {
    ($ writer : ident) => {
        impl<'a, U, REG, FI, const OF: u8> $writer<'a, U, REG, FI, OF>
        where
            REG: Writable + RegisterSpec<Ux = U>,
            U: RawReg,
            bool: From<FI>,
        {
            #[doc = " Writes bit to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut REG::Writer {
                self.w.bits &= !(U::one() << OF);
                self.w.bits |= (U::from(value) & U::one()) << OF;
                self.w
            }
            #[doc = " Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
                self.bit(bool::from(variant))
            }
        }
    };
}
bit_proxy!(BitWriter, BitM);
bit_proxy!(BitWriter1S, Bit1S);
bit_proxy!(BitWriter0C, Bit0C);
bit_proxy!(BitWriter1C, Bit1C);
bit_proxy!(BitWriter0S, Bit0S);
bit_proxy!(BitWriter1T, Bit1T);
bit_proxy!(BitWriter0T, Bit0T);
impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriter<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg + From<N>,
    N: From<FI>,
{
    #[doc = " Writes raw bits to the field"]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(U::mask::<WI>() << OF);
        self.w.bits |= (U::from(value) & U::mask::<WI>()) << OF;
        self.w
    }
    #[doc = " Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        unsafe { self.bits(N::from(variant)) }
    }
}
impl<'a, U, REG, N, FI, const WI: u8, const OF: u8> FieldWriterSafe<'a, U, REG, N, FI, WI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg + From<N>,
    N: From<FI>,
{
    #[doc = " Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(U::mask::<WI>() << OF);
        self.w.bits |= (U::from(value) & U::mask::<WI>()) << OF;
        self.w
    }
    #[doc = " Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        self.bits(N::from(variant))
    }
}
impl_bit_proxy!(BitWriter);
impl_bit_proxy!(BitWriter1S);
impl_bit_proxy!(BitWriter0C);
impl_bit_proxy!(BitWriter1C);
impl_bit_proxy!(BitWriter0S);
impl_bit_proxy!(BitWriter1T);
impl_bit_proxy!(BitWriter0T);
impl<'a, U, REG, FI, const OF: u8> BitWriter<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = " Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
    #[doc = " Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter1S<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = " Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter0C<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = " Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter1C<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = "Clears the field bit by passing one"]
    #[inline(always)]
    pub fn clear_bit_by_one(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter0S<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = "Sets the field bit by passing zero"]
    #[inline(always)]
    pub fn set_bit_by_zero(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter1T<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = "Toggle the field bit by passing one"]
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= U::one() << OF;
        self.w
    }
}
impl<'a, U, REG, FI, const OF: u8> BitWriter0T<'a, U, REG, FI, OF>
where
    REG: Writable + RegisterSpec<Ux = U>,
    U: RawReg,
    bool: From<FI>,
{
    #[doc = "Toggle the field bit by passing zero"]
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(U::one() << OF);
        self.w
    }
}
