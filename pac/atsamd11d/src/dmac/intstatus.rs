#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Field `CHINT0` reader - Channel 0 Pending Interrupt"]
pub type Chint0R = crate::BitReader;
#[doc = "Field `CHINT1` reader - Channel 1 Pending Interrupt"]
pub type Chint1R = crate::BitReader;
#[doc = "Field `CHINT2` reader - Channel 2 Pending Interrupt"]
pub type Chint2R = crate::BitReader;
#[doc = "Field `CHINT3` reader - Channel 3 Pending Interrupt"]
pub type Chint3R = crate::BitReader;
#[doc = "Field `CHINT4` reader - Channel 4 Pending Interrupt"]
pub type Chint4R = crate::BitReader;
#[doc = "Field `CHINT5` reader - Channel 5 Pending Interrupt"]
pub type Chint5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Pending Interrupt"]
    #[inline(always)]
    pub fn chint0(&self) -> Chint0R {
        Chint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Pending Interrupt"]
    #[inline(always)]
    pub fn chint1(&self) -> Chint1R {
        Chint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Pending Interrupt"]
    #[inline(always)]
    pub fn chint2(&self) -> Chint2R {
        Chint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Pending Interrupt"]
    #[inline(always)]
    pub fn chint3(&self) -> Chint3R {
        Chint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Pending Interrupt"]
    #[inline(always)]
    pub fn chint4(&self) -> Chint4R {
        Chint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Pending Interrupt"]
    #[inline(always)]
    pub fn chint5(&self) -> Chint5R {
        Chint5R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for IntstatusSpec {
    const RESET_VALUE: u32 = 0;
}
