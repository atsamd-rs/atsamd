#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `READY` reader - Ready to accept a command"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `PRM` reader - Power Reduction Mode"]
pub type PrmR = crate::BitReader;
#[doc = "Field `LOAD` reader - NVM Page Buffer Active Loading"]
pub type LoadR = crate::BitReader;
#[doc = "Field `SUSP` reader - NVM Write Or Erase Operation Is Suspended"]
pub type SuspR = crate::BitReader;
#[doc = "Field `AFIRST` reader - BANKA First"]
pub type AfirstR = crate::BitReader;
#[doc = "Field `BPDIS` reader - Boot Loader Protection Disable"]
pub type BpdisR = crate::BitReader;
#[doc = "Boot Loader Protection Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootprotselect {
    #[doc = "15: 0 kbytes"]
    Bootprot0 = 15,
    #[doc = "14: 8 kbytes"]
    Bootprot8 = 14,
    #[doc = "13: 16 kbytes"]
    Bootprot16 = 13,
    #[doc = "12: 24 kbytes"]
    Bootprot24 = 12,
    #[doc = "11: 32 kbytes"]
    Bootprot32 = 11,
    #[doc = "10: 40 kbytes"]
    Bootprot40 = 10,
    #[doc = "9: 48 kbytes"]
    Bootprot48 = 9,
    #[doc = "8: 56 kbytes"]
    Bootprot56 = 8,
    #[doc = "7: 64 kbytes"]
    Bootprot64 = 7,
    #[doc = "6: 72 kbytes"]
    Bootprot72 = 6,
    #[doc = "5: 80 kbytes"]
    Bootprot80 = 5,
    #[doc = "4: 88 kbytes"]
    Bootprot88 = 4,
    #[doc = "3: 96 kbytes"]
    Bootprot96 = 3,
    #[doc = "2: 104 kbytes"]
    Bootprot104 = 2,
    #[doc = "1: 112 kbytes"]
    Bootprot112 = 1,
    #[doc = "0: 120 kbytes"]
    Bootprot120 = 0,
}
impl From<Bootprotselect> for u8 {
    #[inline(always)]
    fn from(variant: Bootprotselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootprotselect {
    type Ux = u8;
}
impl crate::IsEnum for Bootprotselect {}
#[doc = "Field `BOOTPROT` reader - Boot Loader Protection Size"]
pub type BootprotR = crate::FieldReader<Bootprotselect>;
impl BootprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootprotselect {
        match self.bits {
            15 => Bootprotselect::Bootprot0,
            14 => Bootprotselect::Bootprot8,
            13 => Bootprotselect::Bootprot16,
            12 => Bootprotselect::Bootprot24,
            11 => Bootprotselect::Bootprot32,
            10 => Bootprotselect::Bootprot40,
            9 => Bootprotselect::Bootprot48,
            8 => Bootprotselect::Bootprot56,
            7 => Bootprotselect::Bootprot64,
            6 => Bootprotselect::Bootprot72,
            5 => Bootprotselect::Bootprot80,
            4 => Bootprotselect::Bootprot88,
            3 => Bootprotselect::Bootprot96,
            2 => Bootprotselect::Bootprot104,
            1 => Bootprotselect::Bootprot112,
            0 => Bootprotselect::Bootprot120,
            _ => unreachable!(),
        }
    }
    #[doc = "0 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_0(&self) -> bool {
        *self == Bootprotselect::Bootprot0
    }
    #[doc = "8 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_8(&self) -> bool {
        *self == Bootprotselect::Bootprot8
    }
    #[doc = "16 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_16(&self) -> bool {
        *self == Bootprotselect::Bootprot16
    }
    #[doc = "24 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_24(&self) -> bool {
        *self == Bootprotselect::Bootprot24
    }
    #[doc = "32 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_32(&self) -> bool {
        *self == Bootprotselect::Bootprot32
    }
    #[doc = "40 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_40(&self) -> bool {
        *self == Bootprotselect::Bootprot40
    }
    #[doc = "48 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_48(&self) -> bool {
        *self == Bootprotselect::Bootprot48
    }
    #[doc = "56 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_56(&self) -> bool {
        *self == Bootprotselect::Bootprot56
    }
    #[doc = "64 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_64(&self) -> bool {
        *self == Bootprotselect::Bootprot64
    }
    #[doc = "72 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_72(&self) -> bool {
        *self == Bootprotselect::Bootprot72
    }
    #[doc = "80 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_80(&self) -> bool {
        *self == Bootprotselect::Bootprot80
    }
    #[doc = "88 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_88(&self) -> bool {
        *self == Bootprotselect::Bootprot88
    }
    #[doc = "96 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_96(&self) -> bool {
        *self == Bootprotselect::Bootprot96
    }
    #[doc = "104 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_104(&self) -> bool {
        *self == Bootprotselect::Bootprot104
    }
    #[doc = "112 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_112(&self) -> bool {
        *self == Bootprotselect::Bootprot112
    }
    #[doc = "120 kbytes"]
    #[inline(always)]
    pub fn is_bootprot_120(&self) -> bool {
        *self == Bootprotselect::Bootprot120
    }
}
#[doc = "Field `DBPE` reader - Dual Boot Protection Enable"]
pub type DbpeR = crate::BitReader;
#[doc = "Field `BPHL` reader - Boot Protect Hard Lock"]
pub type BphlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Ready to accept a command"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Reduction Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PrmR {
        PrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVM Page Buffer Active Loading"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NVM Write Or Erase Operation Is Suspended"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BANKA First"]
    #[inline(always)]
    pub fn afirst(&self) -> AfirstR {
        AfirstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Boot Loader Protection Disable"]
    #[inline(always)]
    pub fn bpdis(&self) -> BpdisR {
        BpdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Boot Loader Protection Size"]
    #[inline(always)]
    pub fn bootprot(&self) -> BootprotR {
        BootprotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Dual Boot Protection Enable"]
    #[inline(always)]
    pub fn dbpe(&self) -> DbpeR {
        DbpeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Boot Protect Hard Lock"]
    #[inline(always)]
    pub fn bphl(&self) -> BphlR {
        BphlR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u16 = 0;
}
