#[doc = "Register `PRICTRL0` reader"]
pub type R = crate::R<Prictrl0Spec>;
#[doc = "Register `PRICTRL0` writer"]
pub type W = crate::W<Prictrl0Spec>;
#[doc = "Field `LVLPRI0` reader - Level 0 Channel Priority Number"]
pub type Lvlpri0R = crate::FieldReader;
#[doc = "Field `LVLPRI0` writer - Level 0 Channel Priority Number"]
pub type Lvlpri0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RRLVLEN0` reader - Level 0 Round-Robin Scheduling Enable"]
pub type Rrlvlen0R = crate::BitReader;
#[doc = "Field `RRLVLEN0` writer - Level 0 Round-Robin Scheduling Enable"]
pub type Rrlvlen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI1` reader - Level 1 Channel Priority Number"]
pub type Lvlpri1R = crate::FieldReader;
#[doc = "Field `LVLPRI1` writer - Level 1 Channel Priority Number"]
pub type Lvlpri1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RRLVLEN1` reader - Level 1 Round-Robin Scheduling Enable"]
pub type Rrlvlen1R = crate::BitReader;
#[doc = "Field `RRLVLEN1` writer - Level 1 Round-Robin Scheduling Enable"]
pub type Rrlvlen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI2` reader - Level 2 Channel Priority Number"]
pub type Lvlpri2R = crate::FieldReader;
#[doc = "Field `LVLPRI2` writer - Level 2 Channel Priority Number"]
pub type Lvlpri2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RRLVLEN2` reader - Level 2 Round-Robin Scheduling Enable"]
pub type Rrlvlen2R = crate::BitReader;
#[doc = "Field `RRLVLEN2` writer - Level 2 Round-Robin Scheduling Enable"]
pub type Rrlvlen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI3` reader - Level 3 Channel Priority Number"]
pub type Lvlpri3R = crate::FieldReader;
#[doc = "Field `LVLPRI3` writer - Level 3 Channel Priority Number"]
pub type Lvlpri3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RRLVLEN3` reader - Level 3 Round-Robin Scheduling Enable"]
pub type Rrlvlen3R = crate::BitReader;
#[doc = "Field `RRLVLEN3` writer - Level 3 Round-Robin Scheduling Enable"]
pub type Rrlvlen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&self) -> Lvlpri0R {
        Lvlpri0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&self) -> Rrlvlen0R {
        Rrlvlen0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&self) -> Lvlpri1R {
        Lvlpri1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&self) -> Rrlvlen1R {
        Rrlvlen1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&self) -> Lvlpri2R {
        Lvlpri2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&self) -> Rrlvlen2R {
        Rrlvlen2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&self) -> Lvlpri3R {
        Lvlpri3R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&self) -> Rrlvlen3R {
        Rrlvlen3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Level 0 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri0(&mut self) -> Lvlpri0W<Prictrl0Spec> {
        Lvlpri0W::new(self, 0)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen0(&mut self) -> Rrlvlen0W<Prictrl0Spec> {
        Rrlvlen0W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Level 1 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri1(&mut self) -> Lvlpri1W<Prictrl0Spec> {
        Lvlpri1W::new(self, 8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen1(&mut self) -> Rrlvlen1W<Prictrl0Spec> {
        Rrlvlen1W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Level 2 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri2(&mut self) -> Lvlpri2W<Prictrl0Spec> {
        Lvlpri2W::new(self, 16)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen2(&mut self) -> Rrlvlen2W<Prictrl0Spec> {
        Rrlvlen2W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Level 3 Channel Priority Number"]
    #[inline(always)]
    #[must_use]
    pub fn lvlpri3(&mut self) -> Lvlpri3W<Prictrl0Spec> {
        Lvlpri3W::new(self, 24)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrlvlen3(&mut self) -> Rrlvlen3W<Prictrl0Spec> {
        Rrlvlen3W::new(self, 31)
    }
}
#[doc = "Priority Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prictrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prictrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prictrl0Spec;
impl crate::RegisterSpec for Prictrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prictrl0::R`](R) reader structure"]
impl crate::Readable for Prictrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`prictrl0::W`](W) writer structure"]
impl crate::Writable for Prictrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRICTRL0 to value 0"]
impl crate::Resettable for Prictrl0Spec {
    const RESET_VALUE: u32 = 0;
}
