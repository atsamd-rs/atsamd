#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Reader of field `Present`"]
pub type PRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `Access`"]
pub type ACCESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ByteAcc`"]
pub type BYTEACC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn byte_acc(&self) -> BYTEACC_R {
        BYTEACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
