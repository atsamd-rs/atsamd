#[doc = "Reader of register SVLAN"]
pub type R = crate::R<u32, super::SVLAN>;
#[doc = "Writer for register SVLAN"]
pub type W = crate::W<u32, super::SVLAN>;
#[doc = "Register SVLAN `reset()`'s with value 0"]
impl crate::ResetValue for super::SVLAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLAN_TYPE`"]
pub type VLAN_TYPE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLAN_TYPE`"]
pub struct VLAN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLAN_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ESVLAN`"]
pub type ESVLAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESVLAN`"]
pub struct ESVLAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVLAN_W<'a> {
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
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&self) -> VLAN_TYPE_R {
        VLAN_TYPE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&self) -> ESVLAN_R {
        ESVLAN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined VLAN_TYPE Field"]
    #[inline(always)]
    pub fn vlan_type(&mut self) -> VLAN_TYPE_W {
        VLAN_TYPE_W { w: self }
    }
    #[doc = "Bit 31 - Enable Stacked VLAN Processing Mode"]
    #[inline(always)]
    pub fn esvlan(&mut self) -> ESVLAN_W {
        ESVLAN_W { w: self }
    }
}
