#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SLEEP {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "The CPU clock domain is stopped"]
    CPU,
    #[doc = "The CPU and AHB clock domains are stopped"]
    AHB,
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    APB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLER::CPU => 0,
            IDLER::AHB => 0x01,
            IDLER::APB => 0x02,
            IDLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLER {
        match value {
            0 => IDLER::CPU,
            1 => IDLER::AHB,
            2 => IDLER::APB,
            i => IDLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPU`"]
    #[inline]
    pub fn is_cpu(&self) -> bool {
        *self == IDLER::CPU
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline]
    pub fn is_ahb(&self) -> bool {
        *self == IDLER::AHB
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline]
    pub fn is_apb(&self) -> bool {
        *self == IDLER::APB
    }
}
#[doc = "Values that can be written to the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEW {
    #[doc = "The CPU clock domain is stopped"]
    CPU,
    #[doc = "The CPU and AHB clock domains are stopped"]
    AHB,
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    APB,
}
impl IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLEW::CPU => 0,
            IDLEW::AHB => 1,
            IDLEW::APB => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The CPU clock domain is stopped"]
    #[inline]
    pub fn cpu(self) -> &'a mut W {
        self.variant(IDLEW::CPU)
    }
    #[doc = "The CPU and AHB clock domains are stopped"]
    #[inline]
    pub fn ahb(self) -> &'a mut W {
        self.variant(IDLEW::AHB)
    }
    #[doc = "The CPU, AHB and APB clock domains are stopped"]
    #[inline]
    pub fn apb(self) -> &'a mut W {
        self.variant(IDLEW::APB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from(((self.bits >> 0) & 0x03) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Idle Mode Configuration"]
    #[inline]
    pub fn idle(&mut self) -> _IDLEW {
        _IDLEW { w: self }
    }
}
