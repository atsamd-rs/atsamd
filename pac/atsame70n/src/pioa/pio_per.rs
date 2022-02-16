#[doc = "Register `PIO_PER` writer"]
pub struct W(crate::W<PIO_PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PIO_PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` writer - PIO Enable"]
pub struct P0_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `P1` writer - PIO Enable"]
pub struct P1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `P2` writer - PIO Enable"]
pub struct P2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `P3` writer - PIO Enable"]
pub struct P3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `P4` writer - PIO Enable"]
pub struct P4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `P5` writer - PIO Enable"]
pub struct P5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `P6` writer - PIO Enable"]
pub struct P6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `P7` writer - PIO Enable"]
pub struct P7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `P8` writer - PIO Enable"]
pub struct P8_W<'a> {
    w: &'a mut W,
}
impl<'a> P8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `P9` writer - PIO Enable"]
pub struct P9_W<'a> {
    w: &'a mut W,
}
impl<'a> P9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `P10` writer - PIO Enable"]
pub struct P10_W<'a> {
    w: &'a mut W,
}
impl<'a> P10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `P11` writer - PIO Enable"]
pub struct P11_W<'a> {
    w: &'a mut W,
}
impl<'a> P11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `P12` writer - PIO Enable"]
pub struct P12_W<'a> {
    w: &'a mut W,
}
impl<'a> P12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `P13` writer - PIO Enable"]
pub struct P13_W<'a> {
    w: &'a mut W,
}
impl<'a> P13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `P14` writer - PIO Enable"]
pub struct P14_W<'a> {
    w: &'a mut W,
}
impl<'a> P14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `P15` writer - PIO Enable"]
pub struct P15_W<'a> {
    w: &'a mut W,
}
impl<'a> P15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `P16` writer - PIO Enable"]
pub struct P16_W<'a> {
    w: &'a mut W,
}
impl<'a> P16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `P17` writer - PIO Enable"]
pub struct P17_W<'a> {
    w: &'a mut W,
}
impl<'a> P17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `P18` writer - PIO Enable"]
pub struct P18_W<'a> {
    w: &'a mut W,
}
impl<'a> P18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `P19` writer - PIO Enable"]
pub struct P19_W<'a> {
    w: &'a mut W,
}
impl<'a> P19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `P20` writer - PIO Enable"]
pub struct P20_W<'a> {
    w: &'a mut W,
}
impl<'a> P20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `P21` writer - PIO Enable"]
pub struct P21_W<'a> {
    w: &'a mut W,
}
impl<'a> P21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `P22` writer - PIO Enable"]
pub struct P22_W<'a> {
    w: &'a mut W,
}
impl<'a> P22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `P23` writer - PIO Enable"]
pub struct P23_W<'a> {
    w: &'a mut W,
}
impl<'a> P23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `P24` writer - PIO Enable"]
pub struct P24_W<'a> {
    w: &'a mut W,
}
impl<'a> P24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `P25` writer - PIO Enable"]
pub struct P25_W<'a> {
    w: &'a mut W,
}
impl<'a> P25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `P26` writer - PIO Enable"]
pub struct P26_W<'a> {
    w: &'a mut W,
}
impl<'a> P26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `P27` writer - PIO Enable"]
pub struct P27_W<'a> {
    w: &'a mut W,
}
impl<'a> P27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `P28` writer - PIO Enable"]
pub struct P28_W<'a> {
    w: &'a mut W,
}
impl<'a> P28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `P29` writer - PIO Enable"]
pub struct P29_W<'a> {
    w: &'a mut W,
}
impl<'a> P29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `P30` writer - PIO Enable"]
pub struct P30_W<'a> {
    w: &'a mut W,
}
impl<'a> P30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `P31` writer - PIO Enable"]
pub struct P31_W<'a> {
    w: &'a mut W,
}
impl<'a> P31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - PIO Enable"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W {
        P0_W { w: self }
    }
    #[doc = "Bit 1 - PIO Enable"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W {
        P1_W { w: self }
    }
    #[doc = "Bit 2 - PIO Enable"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W {
        P2_W { w: self }
    }
    #[doc = "Bit 3 - PIO Enable"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W {
        P3_W { w: self }
    }
    #[doc = "Bit 4 - PIO Enable"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W {
        P4_W { w: self }
    }
    #[doc = "Bit 5 - PIO Enable"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W {
        P5_W { w: self }
    }
    #[doc = "Bit 6 - PIO Enable"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W {
        P6_W { w: self }
    }
    #[doc = "Bit 7 - PIO Enable"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W {
        P7_W { w: self }
    }
    #[doc = "Bit 8 - PIO Enable"]
    #[inline(always)]
    pub fn p8(&mut self) -> P8_W {
        P8_W { w: self }
    }
    #[doc = "Bit 9 - PIO Enable"]
    #[inline(always)]
    pub fn p9(&mut self) -> P9_W {
        P9_W { w: self }
    }
    #[doc = "Bit 10 - PIO Enable"]
    #[inline(always)]
    pub fn p10(&mut self) -> P10_W {
        P10_W { w: self }
    }
    #[doc = "Bit 11 - PIO Enable"]
    #[inline(always)]
    pub fn p11(&mut self) -> P11_W {
        P11_W { w: self }
    }
    #[doc = "Bit 12 - PIO Enable"]
    #[inline(always)]
    pub fn p12(&mut self) -> P12_W {
        P12_W { w: self }
    }
    #[doc = "Bit 13 - PIO Enable"]
    #[inline(always)]
    pub fn p13(&mut self) -> P13_W {
        P13_W { w: self }
    }
    #[doc = "Bit 14 - PIO Enable"]
    #[inline(always)]
    pub fn p14(&mut self) -> P14_W {
        P14_W { w: self }
    }
    #[doc = "Bit 15 - PIO Enable"]
    #[inline(always)]
    pub fn p15(&mut self) -> P15_W {
        P15_W { w: self }
    }
    #[doc = "Bit 16 - PIO Enable"]
    #[inline(always)]
    pub fn p16(&mut self) -> P16_W {
        P16_W { w: self }
    }
    #[doc = "Bit 17 - PIO Enable"]
    #[inline(always)]
    pub fn p17(&mut self) -> P17_W {
        P17_W { w: self }
    }
    #[doc = "Bit 18 - PIO Enable"]
    #[inline(always)]
    pub fn p18(&mut self) -> P18_W {
        P18_W { w: self }
    }
    #[doc = "Bit 19 - PIO Enable"]
    #[inline(always)]
    pub fn p19(&mut self) -> P19_W {
        P19_W { w: self }
    }
    #[doc = "Bit 20 - PIO Enable"]
    #[inline(always)]
    pub fn p20(&mut self) -> P20_W {
        P20_W { w: self }
    }
    #[doc = "Bit 21 - PIO Enable"]
    #[inline(always)]
    pub fn p21(&mut self) -> P21_W {
        P21_W { w: self }
    }
    #[doc = "Bit 22 - PIO Enable"]
    #[inline(always)]
    pub fn p22(&mut self) -> P22_W {
        P22_W { w: self }
    }
    #[doc = "Bit 23 - PIO Enable"]
    #[inline(always)]
    pub fn p23(&mut self) -> P23_W {
        P23_W { w: self }
    }
    #[doc = "Bit 24 - PIO Enable"]
    #[inline(always)]
    pub fn p24(&mut self) -> P24_W {
        P24_W { w: self }
    }
    #[doc = "Bit 25 - PIO Enable"]
    #[inline(always)]
    pub fn p25(&mut self) -> P25_W {
        P25_W { w: self }
    }
    #[doc = "Bit 26 - PIO Enable"]
    #[inline(always)]
    pub fn p26(&mut self) -> P26_W {
        P26_W { w: self }
    }
    #[doc = "Bit 27 - PIO Enable"]
    #[inline(always)]
    pub fn p27(&mut self) -> P27_W {
        P27_W { w: self }
    }
    #[doc = "Bit 28 - PIO Enable"]
    #[inline(always)]
    pub fn p28(&mut self) -> P28_W {
        P28_W { w: self }
    }
    #[doc = "Bit 29 - PIO Enable"]
    #[inline(always)]
    pub fn p29(&mut self) -> P29_W {
        P29_W { w: self }
    }
    #[doc = "Bit 30 - PIO Enable"]
    #[inline(always)]
    pub fn p30(&mut self) -> P30_W {
        P30_W { w: self }
    }
    #[doc = "Bit 31 - PIO Enable"]
    #[inline(always)]
    pub fn p31(&mut self) -> P31_W {
        P31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_per](index.html) module"]
pub struct PIO_PER_SPEC;
impl crate::RegisterSpec for PIO_PER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pio_per::W](W) writer structure"]
impl crate::Writable for PIO_PER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO_PER to value 0"]
impl crate::Resettable for PIO_PER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
