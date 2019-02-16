#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UASR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `URAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URATR {
    #[doc = "Unspecified structure member set to one detected when the descriptor is loaded"]
    UNSPEC_STRUCT_MEMBER,
    #[doc = "CFG modified during active monitoring"]
    CFG_MODIFIED,
    #[doc = "DSCR modified during active monitoring"]
    DSCR_MODIFIED,
    #[doc = "HASH modified during active monitoring"]
    HASH_MODIFIED,
    #[doc = "Write-only register read access"]
    READ_ACCESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl URATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            URATR::UNSPEC_STRUCT_MEMBER => 0,
            URATR::CFG_MODIFIED => 1,
            URATR::DSCR_MODIFIED => 2,
            URATR::HASH_MODIFIED => 3,
            URATR::READ_ACCESS => 4,
            URATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> URATR {
        match value {
            0 => URATR::UNSPEC_STRUCT_MEMBER,
            1 => URATR::CFG_MODIFIED,
            2 => URATR::DSCR_MODIFIED,
            3 => URATR::HASH_MODIFIED,
            4 => URATR::READ_ACCESS,
            i => URATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNSPEC_STRUCT_MEMBER`"]
    #[inline]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == URATR::UNSPEC_STRUCT_MEMBER
    }
    #[doc = "Checks if the value of the field is `CFG_MODIFIED`"]
    #[inline]
    pub fn is_cfg_modified(&self) -> bool {
        *self == URATR::CFG_MODIFIED
    }
    #[doc = "Checks if the value of the field is `DSCR_MODIFIED`"]
    #[inline]
    pub fn is_dscr_modified(&self) -> bool {
        *self == URATR::DSCR_MODIFIED
    }
    #[doc = "Checks if the value of the field is `HASH_MODIFIED`"]
    #[inline]
    pub fn is_hash_modified(&self) -> bool {
        *self == URATR::HASH_MODIFIED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS`"]
    #[inline]
    pub fn is_read_access(&self) -> bool {
        *self == URATR::READ_ACCESS
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline]
    pub fn urat(&self) -> URATR {
        URATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
