#[doc = "Register `PINTSMRY` reader"]
pub type R = crate::R<PintsmrySpec>;
#[doc = "Field `EPINT0` reader - Pipe 0 Interrupt"]
pub type Epint0R = crate::BitReader;
#[doc = "Field `EPINT1` reader - Pipe 1 Interrupt"]
pub type Epint1R = crate::BitReader;
#[doc = "Field `EPINT2` reader - Pipe 2 Interrupt"]
pub type Epint2R = crate::BitReader;
#[doc = "Field `EPINT3` reader - Pipe 3 Interrupt"]
pub type Epint3R = crate::BitReader;
#[doc = "Field `EPINT4` reader - Pipe 4 Interrupt"]
pub type Epint4R = crate::BitReader;
#[doc = "Field `EPINT5` reader - Pipe 5 Interrupt"]
pub type Epint5R = crate::BitReader;
#[doc = "Field `EPINT6` reader - Pipe 6 Interrupt"]
pub type Epint6R = crate::BitReader;
#[doc = "Field `EPINT7` reader - Pipe 7 Interrupt"]
pub type Epint7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn epint0(&self) -> Epint0R {
        Epint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn epint1(&self) -> Epint1R {
        Epint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn epint2(&self) -> Epint2R {
        Epint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn epint3(&self) -> Epint3R {
        Epint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn epint4(&self) -> Epint4R {
        Epint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn epint5(&self) -> Epint5R {
        Epint5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn epint6(&self) -> Epint6R {
        Epint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn epint7(&self) -> Epint7R {
        Epint7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "HOST Pipe Interrupt Summary\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsmry::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PintsmrySpec;
impl crate::RegisterSpec for PintsmrySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pintsmry::R`](R) reader structure"]
impl crate::Readable for PintsmrySpec {}
#[doc = "`reset()` method sets PINTSMRY to value 0"]
impl crate::Resettable for PintsmrySpec {}
