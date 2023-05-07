#[doc = "Register `UASR` reader"]
pub struct R(crate::R<UASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `URAT` reader - Undefined Register Access Trace"]
pub type URAT_R = crate::FieldReader<u8, URATSELECT_A>;
#[doc = "Undefined Register Access Trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum URATSELECT_A {
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
impl From<URATSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: URATSELECT_A) -> Self {
        variant as _
    }
}
impl URAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<URATSELECT_A> {
        match self.bits {
            0 => Some(URATSELECT_A::UNSPEC_STRUCT_MEMBER),
            1 => Some(URATSELECT_A::CFG_MODIFIED),
            2 => Some(URATSELECT_A::DSCR_MODIFIED),
            3 => Some(URATSELECT_A::HASH_MODIFIED),
            4 => Some(URATSELECT_A::READ_ACCESS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNSPEC_STRUCT_MEMBER`"]
    #[inline(always)]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == URATSELECT_A::UNSPEC_STRUCT_MEMBER
    }
    #[doc = "Checks if the value of the field is `CFG_MODIFIED`"]
    #[inline(always)]
    pub fn is_cfg_modified(&self) -> bool {
        *self == URATSELECT_A::CFG_MODIFIED
    }
    #[doc = "Checks if the value of the field is `DSCR_MODIFIED`"]
    #[inline(always)]
    pub fn is_dscr_modified(&self) -> bool {
        *self == URATSELECT_A::DSCR_MODIFIED
    }
    #[doc = "Checks if the value of the field is `HASH_MODIFIED`"]
    #[inline(always)]
    pub fn is_hash_modified(&self) -> bool {
        *self == URATSELECT_A::HASH_MODIFIED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS`"]
    #[inline(always)]
    pub fn is_read_access(&self) -> bool {
        *self == URATSELECT_A::READ_ACCESS
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Undefined Access Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uasr](index.html) module"]
pub struct UASR_SPEC;
impl crate::RegisterSpec for UASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uasr::R](R) reader structure"]
impl crate::Readable for UASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UASR to value 0"]
impl crate::Resettable for UASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
