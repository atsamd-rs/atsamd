#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PCFG {
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
#[doc = r" Value of the field"]
pub struct PTOKENR {
    bits: u8,
}
impl PTOKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BKR {
    bits: bool,
}
impl BKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PTYPER {
    bits: u8,
}
impl PTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PTOKENW<'a> {
    w: &'a mut W,
}
impl<'a> _PTOKENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u8) & 0x03) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BKW<'a> {
    w: &'a mut W,
}
impl<'a> _BKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u8) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _PTYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 3);
        self.w.bits |= ((value as u8) & 0x07) << 3;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline]
    pub fn ptoken(&self) -> PTOKENR {
        let bits = ((self.bits >> 0) & 0x03) as u8;
        PTOKENR { bits }
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline]
    pub fn bk(&self) -> BKR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        BKR { bits }
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline]
    pub fn ptype(&self) -> PTYPER {
        let bits = ((self.bits >> 3) & 0x07) as u8;
        PTYPER { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Pipe Token"]
    #[inline]
    pub fn ptoken(&mut self) -> _PTOKENW {
        _PTOKENW { w: self }
    }
    #[doc = "Bit 2 - Pipe Bank"]
    #[inline]
    pub fn bk(&mut self) -> _BKW {
        _BKW { w: self }
    }
    #[doc = "Bits 3:5 - Pipe Type"]
    #[inline]
    pub fn ptype(&mut self) -> _PTYPEW {
        _PTYPEW { w: self }
    }
}
