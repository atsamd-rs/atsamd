#[doc = "Register `UASR` reader"]
pub type R = crate::R<UasrSpec>;
#[doc = "Undefined Register Access Trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uratselect {
    #[doc = "0: Unspecified structure member set to one detected when the descriptor is loaded"]
    UnspecStructMember = 0,
    #[doc = "1: CFG modified during active monitoring"]
    CfgModified = 1,
    #[doc = "2: DSCR modified during active monitoring"]
    DscrModified = 2,
    #[doc = "3: HASH modified during active monitoring"]
    HashModified = 3,
    #[doc = "4: Write-only register read access"]
    ReadAccess = 4,
}
impl From<Uratselect> for u8 {
    #[inline(always)]
    fn from(variant: Uratselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uratselect {
    type Ux = u8;
}
impl crate::IsEnum for Uratselect {}
#[doc = "Field `URAT` reader - Undefined Register Access Trace"]
pub type UratR = crate::FieldReader<Uratselect>;
impl UratR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uratselect> {
        match self.bits {
            0 => Some(Uratselect::UnspecStructMember),
            1 => Some(Uratselect::CfgModified),
            2 => Some(Uratselect::DscrModified),
            3 => Some(Uratselect::HashModified),
            4 => Some(Uratselect::ReadAccess),
            _ => None,
        }
    }
    #[doc = "Unspecified structure member set to one detected when the descriptor is loaded"]
    #[inline(always)]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == Uratselect::UnspecStructMember
    }
    #[doc = "CFG modified during active monitoring"]
    #[inline(always)]
    pub fn is_cfg_modified(&self) -> bool {
        *self == Uratselect::CfgModified
    }
    #[doc = "DSCR modified during active monitoring"]
    #[inline(always)]
    pub fn is_dscr_modified(&self) -> bool {
        *self == Uratselect::DscrModified
    }
    #[doc = "HASH modified during active monitoring"]
    #[inline(always)]
    pub fn is_hash_modified(&self) -> bool {
        *self == Uratselect::HashModified
    }
    #[doc = "Write-only register read access"]
    #[inline(always)]
    pub fn is_read_access(&self) -> bool {
        *self == Uratselect::ReadAccess
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline(always)]
    pub fn urat(&self) -> UratR {
        UratR::new((self.bits & 7) as u8)
    }
}
#[doc = "Undefined Access Status\n\nYou can [`read`](crate::Reg::read) this register and get [`uasr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UasrSpec;
impl crate::RegisterSpec for UasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uasr::R`](R) reader structure"]
impl crate::Readable for UasrSpec {}
#[doc = "`reset()` method sets UASR to value 0"]
impl crate::Resettable for UasrSpec {}
