#[doc = "Register `ACESR` reader"]
pub type R = crate::R<AcesrSpec>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmd12neselect {
    #[doc = "0: Executed"]
    Exec = 0,
    #[doc = "1: Not executed"]
    NotExec = 1,
}
impl From<Acmd12neselect> for bool {
    #[inline(always)]
    fn from(variant: Acmd12neselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12NE` reader - Auto CMD12 Not Executed"]
pub type Acmd12neR = crate::BitReader<Acmd12neselect>;
impl Acmd12neR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmd12neselect {
        match self.bits {
            false => Acmd12neselect::Exec,
            true => Acmd12neselect::NotExec,
        }
    }
    #[doc = "Executed"]
    #[inline(always)]
    pub fn is_exec(&self) -> bool {
        *self == Acmd12neselect::Exec
    }
    #[doc = "Not executed"]
    #[inline(always)]
    pub fn is_not_exec(&self) -> bool {
        *self == Acmd12neselect::NotExec
    }
}
#[doc = "Auto CMD Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdteoselect {
    #[doc = "0: No error"]
    No = 0,
    #[doc = "1: Timeout"]
    Yes = 1,
}
impl From<Acmdteoselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdteoselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDTEO` reader - Auto CMD Timeout Error"]
pub type AcmdteoR = crate::BitReader<Acmdteoselect>;
impl AcmdteoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdteoselect {
        match self.bits {
            false => Acmdteoselect::No,
            true => Acmdteoselect::Yes,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Acmdteoselect::No
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Acmdteoselect::Yes
    }
}
#[doc = "Auto CMD CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdcrcselect {
    #[doc = "0: No error"]
    No = 0,
    #[doc = "1: CRC Error Generated"]
    Yes = 1,
}
impl From<Acmdcrcselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdcrcselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDCRC` reader - Auto CMD CRC Error"]
pub type AcmdcrcR = crate::BitReader<Acmdcrcselect>;
impl AcmdcrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdcrcselect {
        match self.bits {
            false => Acmdcrcselect::No,
            true => Acmdcrcselect::Yes,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Acmdcrcselect::No
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Acmdcrcselect::Yes
    }
}
#[doc = "Auto CMD End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdendselect {
    #[doc = "0: No error"]
    No = 0,
    #[doc = "1: End Bit Error Generated"]
    Yes = 1,
}
impl From<Acmdendselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdendselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDEND` reader - Auto CMD End Bit Error"]
pub type AcmdendR = crate::BitReader<Acmdendselect>;
impl AcmdendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdendselect {
        match self.bits {
            false => Acmdendselect::No,
            true => Acmdendselect::Yes,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Acmdendselect::No
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Acmdendselect::Yes
    }
}
#[doc = "Auto CMD Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdidxselect {
    #[doc = "0: No error"]
    No = 0,
    #[doc = "1: Error"]
    Yes = 1,
}
impl From<Acmdidxselect> for bool {
    #[inline(always)]
    fn from(variant: Acmdidxselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDIDX` reader - Auto CMD Index Error"]
pub type AcmdidxR = crate::BitReader<Acmdidxselect>;
impl AcmdidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdidxselect {
        match self.bits {
            false => Acmdidxselect::No,
            true => Acmdidxselect::Yes,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Acmdidxselect::No
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Acmdidxselect::Yes
    }
}
#[doc = "Command not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdniselect {
    #[doc = "0: No error"]
    Ok = 0,
    #[doc = "1: Not Issued"]
    NotIssued = 1,
}
impl From<Cmdniselect> for bool {
    #[inline(always)]
    fn from(variant: Cmdniselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDNI` reader - Command not Issued By Auto CMD12 Error"]
pub type CmdniR = crate::BitReader<Cmdniselect>;
impl CmdniR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdniselect {
        match self.bits {
            false => Cmdniselect::Ok,
            true => Cmdniselect::NotIssued,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Cmdniselect::Ok
    }
    #[doc = "Not Issued"]
    #[inline(always)]
    pub fn is_not_issued(&self) -> bool {
        *self == Cmdniselect::NotIssued
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn acmd12ne(&self) -> Acmd12neR {
        Acmd12neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn acmdteo(&self) -> AcmdteoR {
        AcmdteoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn acmdcrc(&self) -> AcmdcrcR {
        AcmdcrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn acmdend(&self) -> AcmdendR {
        AcmdendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn acmdidx(&self) -> AcmdidxR {
        AcmdidxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cmdni(&self) -> CmdniR {
        CmdniR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Auto CMD Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`acesr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcesrSpec;
impl crate::RegisterSpec for AcesrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`acesr::R`](R) reader structure"]
impl crate::Readable for AcesrSpec {}
#[doc = "`reset()` method sets ACESR to value 0"]
impl crate::Resettable for AcesrSpec {}
