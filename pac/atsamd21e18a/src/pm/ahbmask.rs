#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBMASK {
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
    pub const fn reset_value() -> u32 {
        0x7f
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct HPB0_R {
    bits: bool,
}
impl HPB0_R {
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
pub struct HPB1_R {
    bits: bool,
}
impl HPB1_R {
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
pub struct HPB2_R {
    bits: bool,
}
impl HPB2_R {
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
pub struct DSU_R {
    bits: bool,
}
impl DSU_R {
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
pub struct NVMCTRL_R {
    bits: bool,
}
impl NVMCTRL_R {
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
pub struct DMAC_R {
    bits: bool,
}
impl DMAC_R {
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
pub struct USB_R {
    bits: bool,
}
impl USB_R {
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
#[doc = r" Proxy"]
pub struct _HPB0_W<'a> {
    w: &'a mut W,
}
impl<'a> _HPB0_W<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u32) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPB1_W<'a> {
    w: &'a mut W,
}
impl<'a> _HPB1_W<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPB2_W<'a> {
    w: &'a mut W,
}
impl<'a> _HPB2_W<'a> {
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
        self.w.bits |= ((value as u32) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSU_W<'a> {
    w: &'a mut W,
}
impl<'a> _DSU_W<'a> {
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
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u32) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NVMCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> _NVMCTRL_W<'a> {
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
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u32) & 0x01) << 4;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAC_W<'a> {
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u32) & 0x01) << 5;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USB_W<'a> {
    w: &'a mut W,
}
impl<'a> _USB_W<'a> {
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u32) & 0x01) << 6;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline]
    pub fn hpb0_(&self) -> HPB0_R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        HPB0_R { bits }
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline]
    pub fn hpb1_(&self) -> HPB1_R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        HPB1_R { bits }
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline]
    pub fn hpb2_(&self) -> HPB2_R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        HPB2_R { bits }
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline]
    pub fn dsu_(&self) -> DSU_R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        DSU_R { bits }
    }
    #[doc = "Bit 4 - NVMCTRL AHB Clock Mask"]
    #[inline]
    pub fn nvmctrl_(&self) -> NVMCTRL_R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        NVMCTRL_R { bits }
    }
    #[doc = "Bit 5 - DMAC AHB Clock Mask"]
    #[inline]
    pub fn dmac_(&self) -> DMAC_R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        DMAC_R { bits }
    }
    #[doc = "Bit 6 - USB AHB Clock Mask"]
    #[inline]
    pub fn usb_(&self) -> USB_R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        USB_R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline]
    pub fn hpb0_(&mut self) -> _HPB0_W {
        _HPB0_W { w: self }
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline]
    pub fn hpb1_(&mut self) -> _HPB1_W {
        _HPB1_W { w: self }
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline]
    pub fn hpb2_(&mut self) -> _HPB2_W {
        _HPB2_W { w: self }
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline]
    pub fn dsu_(&mut self) -> _DSU_W {
        _DSU_W { w: self }
    }
    #[doc = "Bit 4 - NVMCTRL AHB Clock Mask"]
    #[inline]
    pub fn nvmctrl_(&mut self) -> _NVMCTRL_W {
        _NVMCTRL_W { w: self }
    }
    #[doc = "Bit 5 - DMAC AHB Clock Mask"]
    #[inline]
    pub fn dmac_(&mut self) -> _DMAC_W {
        _DMAC_W { w: self }
    }
    #[doc = "Bit 6 - USB AHB Clock Mask"]
    #[inline]
    pub fn usb_(&mut self) -> _USB_W {
        _USB_W { w: self }
    }
}
