use core::marker;
#[doc = " Raw register type"]
pub trait RegisterSpec {
    #[doc = " Raw register type (`u8`, `u16`, `u32`, ...)."]
    type Ux: Copy;
}
#[doc = " Trait implemented by readable registers to enable the `read` method."]
#[doc = ""]
#[doc = " Registers marked with `Writable` can be also `modify`'ed."]
pub trait Readable: RegisterSpec {
    #[doc = " Result from a call to `read` and argument to `modify`."]
    type Reader: From<R<Self>> + core::ops::Deref<Target = R<Self>>;
}
#[doc = " Trait implemented by writeable registers."]
#[doc = ""]
#[doc = " This enables the  `write`, `write_with_zero` and `reset` methods."]
#[doc = ""]
#[doc = " Registers marked with `Readable` can be also `modify`'ed."]
pub trait Writable: RegisterSpec {
    #[doc = " Writer type argument to `write`, et al."]
    type Writer: From<W<Self>> + core::ops::DerefMut<Target = W<Self>>;
}
#[doc = " Reset value of the register."]
#[doc = ""]
#[doc = " This value is the initial value for the `write` method. It can also be directly written to the"]
#[doc = " register by using the `reset` method."]
pub trait Resettable: RegisterSpec {
    #[doc = " Reset value of the register."]
    fn reset_value() -> Self::Ux;
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
        self.register.set(REG::reset_value())
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
    #[doc = " In the latter case, other fields will be set to their reset value."]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::reset_value(),
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}
impl<REG: Writable> Reg<REG>
where
    REG::Ux: Default,
{
    #[doc = " Writes 0 to a `Writable` register."]
    #[doc = ""]
    #[doc = " Similar to `write`, but unused bits will contain 0."]
    #[inline(always)]
    pub unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            (*f(&mut REG::Writer::from(W {
                bits: REG::Ux::default(),
                _reg: marker::PhantomData,
            })))
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
                    bits,
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
    FI: Copy + Into<REG::Ux>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&(*other).into())
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
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: REG::Ux) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = " Field reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of fields."]
pub struct FieldReader<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}
impl<U, T> FieldReader<U, T>
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
    #[doc = " Reads raw bits from field."]
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}
impl<U, T, FI> PartialEq<FI> for FieldReader<U, T>
where
    U: PartialEq,
    FI: Copy + Into<U>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&(*other).into())
    }
}
impl<FI> FieldReader<bool, FI> {
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
