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
#[doc = "Field `CHINT6` reader - Channel 6 Pending Interrupt"]
pub type Chint6R = crate::BitReader;
#[doc = "Field `CHINT7` reader - Channel 7 Pending Interrupt"]
pub type Chint7R = crate::BitReader;
#[doc = "Field `CHINT8` reader - Channel 8 Pending Interrupt"]
pub type Chint8R = crate::BitReader;
#[doc = "Field `CHINT9` reader - Channel 9 Pending Interrupt"]
pub type Chint9R = crate::BitReader;
#[doc = "Field `CHINT10` reader - Channel 10 Pending Interrupt"]
pub type Chint10R = crate::BitReader;
#[doc = "Field `CHINT11` reader - Channel 11 Pending Interrupt"]
pub type Chint11R = crate::BitReader;
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
    #[doc = "Bit 6 - Channel 6 Pending Interrupt"]
    #[inline(always)]
    pub fn chint6(&self) -> Chint6R {
        Chint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Pending Interrupt"]
    #[inline(always)]
    pub fn chint7(&self) -> Chint7R {
        Chint7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Pending Interrupt"]
    #[inline(always)]
    pub fn chint8(&self) -> Chint8R {
        Chint8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Pending Interrupt"]
    #[inline(always)]
    pub fn chint9(&self) -> Chint9R {
        Chint9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Pending Interrupt"]
    #[inline(always)]
    pub fn chint10(&self) -> Chint10R {
        Chint10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Pending Interrupt"]
    #[inline(always)]
    pub fn chint11(&self) -> Chint11R {
        Chint11R::new(((self.bits >> 11) & 1) != 0)
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
