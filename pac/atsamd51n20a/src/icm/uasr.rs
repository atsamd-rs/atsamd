#[doc = "Reader of register UASR"]
pub type R = crate::R<u32, super::UASR>;
#[doc = "Undefined Register Access Trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum URAT_A {
    #[doc = "0: Unspecified structure member set to one detected when the descriptor is loaded"]
    UNSPEC_STRUCT_MEMBER = 0,
    #[doc = "1: CFG modified during active monitoring"]
    CFG_MODIFIED = 1,
    #[doc = "2: DSCR modified during active monitoring"]
    DSCR_MODIFIED = 2,
    #[doc = "3: HASH modified during active monitoring"]
    HASH_MODIFIED = 3,
    #[doc = "4: Write-only register read access"]
    READ_ACCESS = 4,
}
impl From<URAT_A> for u8 {
    #[inline(always)]
    fn from(variant: URAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `URAT`"]
pub type URAT_R = crate::R<u8, URAT_A>;
impl URAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, URAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(URAT_A::UNSPEC_STRUCT_MEMBER),
            1 => Val(URAT_A::CFG_MODIFIED),
            2 => Val(URAT_A::DSCR_MODIFIED),
            3 => Val(URAT_A::HASH_MODIFIED),
            4 => Val(URAT_A::READ_ACCESS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNSPEC_STRUCT_MEMBER`"]
    #[inline(always)]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == URAT_A::UNSPEC_STRUCT_MEMBER
    }
    #[doc = "Checks if the value of the field is `CFG_MODIFIED`"]
    #[inline(always)]
    pub fn is_cfg_modified(&self) -> bool {
        *self == URAT_A::CFG_MODIFIED
    }
    #[doc = "Checks if the value of the field is `DSCR_MODIFIED`"]
    #[inline(always)]
    pub fn is_dscr_modified(&self) -> bool {
        *self == URAT_A::DSCR_MODIFIED
    }
    #[doc = "Checks if the value of the field is `HASH_MODIFIED`"]
    #[inline(always)]
    pub fn is_hash_modified(&self) -> bool {
        *self == URAT_A::HASH_MODIFIED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS`"]
    #[inline(always)]
    pub fn is_read_access(&self) -> bool {
        *self == URAT_A::READ_ACCESS
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new((self.bits & 0x07) as u8)
    }
}
