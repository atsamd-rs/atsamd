#[doc = "Reader of register NDAT2"]
pub type R = crate::R<u32, super::NDAT2>;
#[doc = "Writer for register NDAT2"]
pub type W = crate::W<u32, super::NDAT2>;
#[doc = "Register NDAT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NDAT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ND32`"]
pub type ND32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND32`"]
pub struct ND32_W<'a> {
    w: &'a mut W,
}
impl<'a> ND32_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ND33`"]
pub type ND33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND33`"]
pub struct ND33_W<'a> {
    w: &'a mut W,
}
impl<'a> ND33_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ND34`"]
pub type ND34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND34`"]
pub struct ND34_W<'a> {
    w: &'a mut W,
}
impl<'a> ND34_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ND35`"]
pub type ND35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND35`"]
pub struct ND35_W<'a> {
    w: &'a mut W,
}
impl<'a> ND35_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ND36`"]
pub type ND36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND36`"]
pub struct ND36_W<'a> {
    w: &'a mut W,
}
impl<'a> ND36_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ND37`"]
pub type ND37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND37`"]
pub struct ND37_W<'a> {
    w: &'a mut W,
}
impl<'a> ND37_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ND38`"]
pub type ND38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND38`"]
pub struct ND38_W<'a> {
    w: &'a mut W,
}
impl<'a> ND38_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ND39`"]
pub type ND39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND39`"]
pub struct ND39_W<'a> {
    w: &'a mut W,
}
impl<'a> ND39_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ND40`"]
pub type ND40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND40`"]
pub struct ND40_W<'a> {
    w: &'a mut W,
}
impl<'a> ND40_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ND41`"]
pub type ND41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND41`"]
pub struct ND41_W<'a> {
    w: &'a mut W,
}
impl<'a> ND41_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ND42`"]
pub type ND42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND42`"]
pub struct ND42_W<'a> {
    w: &'a mut W,
}
impl<'a> ND42_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ND43`"]
pub type ND43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND43`"]
pub struct ND43_W<'a> {
    w: &'a mut W,
}
impl<'a> ND43_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ND44`"]
pub type ND44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND44`"]
pub struct ND44_W<'a> {
    w: &'a mut W,
}
impl<'a> ND44_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ND45`"]
pub type ND45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND45`"]
pub struct ND45_W<'a> {
    w: &'a mut W,
}
impl<'a> ND45_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ND46`"]
pub type ND46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND46`"]
pub struct ND46_W<'a> {
    w: &'a mut W,
}
impl<'a> ND46_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ND47`"]
pub type ND47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND47`"]
pub struct ND47_W<'a> {
    w: &'a mut W,
}
impl<'a> ND47_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ND48`"]
pub type ND48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND48`"]
pub struct ND48_W<'a> {
    w: &'a mut W,
}
impl<'a> ND48_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ND49`"]
pub type ND49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND49`"]
pub struct ND49_W<'a> {
    w: &'a mut W,
}
impl<'a> ND49_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ND50`"]
pub type ND50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND50`"]
pub struct ND50_W<'a> {
    w: &'a mut W,
}
impl<'a> ND50_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ND51`"]
pub type ND51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND51`"]
pub struct ND51_W<'a> {
    w: &'a mut W,
}
impl<'a> ND51_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ND52`"]
pub type ND52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND52`"]
pub struct ND52_W<'a> {
    w: &'a mut W,
}
impl<'a> ND52_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ND53`"]
pub type ND53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND53`"]
pub struct ND53_W<'a> {
    w: &'a mut W,
}
impl<'a> ND53_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ND54`"]
pub type ND54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND54`"]
pub struct ND54_W<'a> {
    w: &'a mut W,
}
impl<'a> ND54_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ND55`"]
pub type ND55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND55`"]
pub struct ND55_W<'a> {
    w: &'a mut W,
}
impl<'a> ND55_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ND56`"]
pub type ND56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND56`"]
pub struct ND56_W<'a> {
    w: &'a mut W,
}
impl<'a> ND56_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ND57`"]
pub type ND57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND57`"]
pub struct ND57_W<'a> {
    w: &'a mut W,
}
impl<'a> ND57_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ND58`"]
pub type ND58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND58`"]
pub struct ND58_W<'a> {
    w: &'a mut W,
}
impl<'a> ND58_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ND59`"]
pub type ND59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND59`"]
pub struct ND59_W<'a> {
    w: &'a mut W,
}
impl<'a> ND59_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ND60`"]
pub type ND60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND60`"]
pub struct ND60_W<'a> {
    w: &'a mut W,
}
impl<'a> ND60_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ND61`"]
pub type ND61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND61`"]
pub struct ND61_W<'a> {
    w: &'a mut W,
}
impl<'a> ND61_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ND62`"]
pub type ND62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND62`"]
pub struct ND62_W<'a> {
    w: &'a mut W,
}
impl<'a> ND62_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ND63`"]
pub type ND63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ND63`"]
pub struct ND63_W<'a> {
    w: &'a mut W,
}
impl<'a> ND63_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - New Data 32"]
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - New Data 33"]
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - New Data 34"]
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - New Data 35"]
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - New Data 36"]
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - New Data 37"]
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - New Data 38"]
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - New Data 39"]
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - New Data 40"]
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - New Data 41"]
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - New Data 42"]
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - New Data 43"]
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - New Data 44"]
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - New Data 45"]
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - New Data 46"]
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - New Data 47"]
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - New Data 48"]
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - New Data 49"]
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - New Data 50"]
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - New Data 51"]
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - New Data 52"]
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - New Data 53"]
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - New Data 54"]
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - New Data 55"]
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - New Data 56"]
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - New Data 57"]
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - New Data 58"]
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - New Data 59"]
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - New Data 60"]
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New Data 61"]
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - New Data 62"]
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - New Data 63"]
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New Data 32"]
    #[inline(always)]
    pub fn nd32(&mut self) -> ND32_W {
        ND32_W { w: self }
    }
    #[doc = "Bit 1 - New Data 33"]
    #[inline(always)]
    pub fn nd33(&mut self) -> ND33_W {
        ND33_W { w: self }
    }
    #[doc = "Bit 2 - New Data 34"]
    #[inline(always)]
    pub fn nd34(&mut self) -> ND34_W {
        ND34_W { w: self }
    }
    #[doc = "Bit 3 - New Data 35"]
    #[inline(always)]
    pub fn nd35(&mut self) -> ND35_W {
        ND35_W { w: self }
    }
    #[doc = "Bit 4 - New Data 36"]
    #[inline(always)]
    pub fn nd36(&mut self) -> ND36_W {
        ND36_W { w: self }
    }
    #[doc = "Bit 5 - New Data 37"]
    #[inline(always)]
    pub fn nd37(&mut self) -> ND37_W {
        ND37_W { w: self }
    }
    #[doc = "Bit 6 - New Data 38"]
    #[inline(always)]
    pub fn nd38(&mut self) -> ND38_W {
        ND38_W { w: self }
    }
    #[doc = "Bit 7 - New Data 39"]
    #[inline(always)]
    pub fn nd39(&mut self) -> ND39_W {
        ND39_W { w: self }
    }
    #[doc = "Bit 8 - New Data 40"]
    #[inline(always)]
    pub fn nd40(&mut self) -> ND40_W {
        ND40_W { w: self }
    }
    #[doc = "Bit 9 - New Data 41"]
    #[inline(always)]
    pub fn nd41(&mut self) -> ND41_W {
        ND41_W { w: self }
    }
    #[doc = "Bit 10 - New Data 42"]
    #[inline(always)]
    pub fn nd42(&mut self) -> ND42_W {
        ND42_W { w: self }
    }
    #[doc = "Bit 11 - New Data 43"]
    #[inline(always)]
    pub fn nd43(&mut self) -> ND43_W {
        ND43_W { w: self }
    }
    #[doc = "Bit 12 - New Data 44"]
    #[inline(always)]
    pub fn nd44(&mut self) -> ND44_W {
        ND44_W { w: self }
    }
    #[doc = "Bit 13 - New Data 45"]
    #[inline(always)]
    pub fn nd45(&mut self) -> ND45_W {
        ND45_W { w: self }
    }
    #[doc = "Bit 14 - New Data 46"]
    #[inline(always)]
    pub fn nd46(&mut self) -> ND46_W {
        ND46_W { w: self }
    }
    #[doc = "Bit 15 - New Data 47"]
    #[inline(always)]
    pub fn nd47(&mut self) -> ND47_W {
        ND47_W { w: self }
    }
    #[doc = "Bit 16 - New Data 48"]
    #[inline(always)]
    pub fn nd48(&mut self) -> ND48_W {
        ND48_W { w: self }
    }
    #[doc = "Bit 17 - New Data 49"]
    #[inline(always)]
    pub fn nd49(&mut self) -> ND49_W {
        ND49_W { w: self }
    }
    #[doc = "Bit 18 - New Data 50"]
    #[inline(always)]
    pub fn nd50(&mut self) -> ND50_W {
        ND50_W { w: self }
    }
    #[doc = "Bit 19 - New Data 51"]
    #[inline(always)]
    pub fn nd51(&mut self) -> ND51_W {
        ND51_W { w: self }
    }
    #[doc = "Bit 20 - New Data 52"]
    #[inline(always)]
    pub fn nd52(&mut self) -> ND52_W {
        ND52_W { w: self }
    }
    #[doc = "Bit 21 - New Data 53"]
    #[inline(always)]
    pub fn nd53(&mut self) -> ND53_W {
        ND53_W { w: self }
    }
    #[doc = "Bit 22 - New Data 54"]
    #[inline(always)]
    pub fn nd54(&mut self) -> ND54_W {
        ND54_W { w: self }
    }
    #[doc = "Bit 23 - New Data 55"]
    #[inline(always)]
    pub fn nd55(&mut self) -> ND55_W {
        ND55_W { w: self }
    }
    #[doc = "Bit 24 - New Data 56"]
    #[inline(always)]
    pub fn nd56(&mut self) -> ND56_W {
        ND56_W { w: self }
    }
    #[doc = "Bit 25 - New Data 57"]
    #[inline(always)]
    pub fn nd57(&mut self) -> ND57_W {
        ND57_W { w: self }
    }
    #[doc = "Bit 26 - New Data 58"]
    #[inline(always)]
    pub fn nd58(&mut self) -> ND58_W {
        ND58_W { w: self }
    }
    #[doc = "Bit 27 - New Data 59"]
    #[inline(always)]
    pub fn nd59(&mut self) -> ND59_W {
        ND59_W { w: self }
    }
    #[doc = "Bit 28 - New Data 60"]
    #[inline(always)]
    pub fn nd60(&mut self) -> ND60_W {
        ND60_W { w: self }
    }
    #[doc = "Bit 29 - New Data 61"]
    #[inline(always)]
    pub fn nd61(&mut self) -> ND61_W {
        ND61_W { w: self }
    }
    #[doc = "Bit 30 - New Data 62"]
    #[inline(always)]
    pub fn nd62(&mut self) -> ND62_W {
        ND62_W { w: self }
    }
    #[doc = "Bit 31 - New Data 63"]
    #[inline(always)]
    pub fn nd63(&mut self) -> ND63_W {
        ND63_W { w: self }
    }
}
