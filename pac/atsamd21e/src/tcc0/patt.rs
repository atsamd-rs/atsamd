#[doc = "Register `PATT` reader"]
pub type R = crate::R<PATT_SPEC>;
#[doc = "Register `PATT` writer"]
pub type W = crate::W<PATT_SPEC>;
#[doc = "Field `PGE0` reader - Pattern Generator 0 Output Enable"]
pub type PGE0_R = crate::BitReader;
#[doc = "Field `PGE0` writer - Pattern Generator 0 Output Enable"]
pub type PGE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE1` reader - Pattern Generator 1 Output Enable"]
pub type PGE1_R = crate::BitReader;
#[doc = "Field `PGE1` writer - Pattern Generator 1 Output Enable"]
pub type PGE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE2` reader - Pattern Generator 2 Output Enable"]
pub type PGE2_R = crate::BitReader;
#[doc = "Field `PGE2` writer - Pattern Generator 2 Output Enable"]
pub type PGE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE3` reader - Pattern Generator 3 Output Enable"]
pub type PGE3_R = crate::BitReader;
#[doc = "Field `PGE3` writer - Pattern Generator 3 Output Enable"]
pub type PGE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE4` reader - Pattern Generator 4 Output Enable"]
pub type PGE4_R = crate::BitReader;
#[doc = "Field `PGE4` writer - Pattern Generator 4 Output Enable"]
pub type PGE4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE5` reader - Pattern Generator 5 Output Enable"]
pub type PGE5_R = crate::BitReader;
#[doc = "Field `PGE5` writer - Pattern Generator 5 Output Enable"]
pub type PGE5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE6` reader - Pattern Generator 6 Output Enable"]
pub type PGE6_R = crate::BitReader;
#[doc = "Field `PGE6` writer - Pattern Generator 6 Output Enable"]
pub type PGE6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE7` reader - Pattern Generator 7 Output Enable"]
pub type PGE7_R = crate::BitReader;
#[doc = "Field `PGE7` writer - Pattern Generator 7 Output Enable"]
pub type PGE7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV0` reader - Pattern Generator 0 Output Value"]
pub type PGV0_R = crate::BitReader;
#[doc = "Field `PGV0` writer - Pattern Generator 0 Output Value"]
pub type PGV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV1` reader - Pattern Generator 1 Output Value"]
pub type PGV1_R = crate::BitReader;
#[doc = "Field `PGV1` writer - Pattern Generator 1 Output Value"]
pub type PGV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV2` reader - Pattern Generator 2 Output Value"]
pub type PGV2_R = crate::BitReader;
#[doc = "Field `PGV2` writer - Pattern Generator 2 Output Value"]
pub type PGV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV3` reader - Pattern Generator 3 Output Value"]
pub type PGV3_R = crate::BitReader;
#[doc = "Field `PGV3` writer - Pattern Generator 3 Output Value"]
pub type PGV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV4` reader - Pattern Generator 4 Output Value"]
pub type PGV4_R = crate::BitReader;
#[doc = "Field `PGV4` writer - Pattern Generator 4 Output Value"]
pub type PGV4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV5` reader - Pattern Generator 5 Output Value"]
pub type PGV5_R = crate::BitReader;
#[doc = "Field `PGV5` writer - Pattern Generator 5 Output Value"]
pub type PGV5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV6` reader - Pattern Generator 6 Output Value"]
pub type PGV6_R = crate::BitReader;
#[doc = "Field `PGV6` writer - Pattern Generator 6 Output Value"]
pub type PGV6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGV7` reader - Pattern Generator 7 Output Value"]
pub type PGV7_R = crate::BitReader;
#[doc = "Field `PGV7` writer - Pattern Generator 7 Output Value"]
pub type PGV7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pge0(&self) -> PGE0_R {
        PGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pge1(&self) -> PGE1_R {
        PGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pge2(&self) -> PGE2_R {
        PGE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pge3(&self) -> PGE3_R {
        PGE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pge4(&self) -> PGE4_R {
        PGE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pge5(&self) -> PGE5_R {
        PGE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pge6(&self) -> PGE6_R {
        PGE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pge7(&self) -> PGE7_R {
        PGE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    pub fn pgv0(&self) -> PGV0_R {
        PGV0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    pub fn pgv1(&self) -> PGV1_R {
        PGV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    pub fn pgv2(&self) -> PGV2_R {
        PGV2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    pub fn pgv3(&self) -> PGV3_R {
        PGV3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    pub fn pgv4(&self) -> PGV4_R {
        PGV4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    pub fn pgv5(&self) -> PGV5_R {
        PGV5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    pub fn pgv6(&self) -> PGV6_R {
        PGV6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    pub fn pgv7(&self) -> PGV7_R {
        PGV7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge0(&mut self) -> PGE0_W<PATT_SPEC, 0> {
        PGE0_W::new(self)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge1(&mut self) -> PGE1_W<PATT_SPEC, 1> {
        PGE1_W::new(self)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge2(&mut self) -> PGE2_W<PATT_SPEC, 2> {
        PGE2_W::new(self)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge3(&mut self) -> PGE3_W<PATT_SPEC, 3> {
        PGE3_W::new(self)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge4(&mut self) -> PGE4_W<PATT_SPEC, 4> {
        PGE4_W::new(self)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge5(&mut self) -> PGE5_W<PATT_SPEC, 5> {
        PGE5_W::new(self)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge6(&mut self) -> PGE6_W<PATT_SPEC, 6> {
        PGE6_W::new(self)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pge7(&mut self) -> PGE7_W<PATT_SPEC, 7> {
        PGE7_W::new(self)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv0(&mut self) -> PGV0_W<PATT_SPEC, 8> {
        PGV0_W::new(self)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv1(&mut self) -> PGV1_W<PATT_SPEC, 9> {
        PGV1_W::new(self)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv2(&mut self) -> PGV2_W<PATT_SPEC, 10> {
        PGV2_W::new(self)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv3(&mut self) -> PGV3_W<PATT_SPEC, 11> {
        PGV3_W::new(self)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv4(&mut self) -> PGV4_W<PATT_SPEC, 12> {
        PGV4_W::new(self)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv5(&mut self) -> PGV5_W<PATT_SPEC, 13> {
        PGV5_W::new(self)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv6(&mut self) -> PGV6_W<PATT_SPEC, 14> {
        PGV6_W::new(self)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn pgv7(&mut self) -> PGV7_W<PATT_SPEC, 15> {
        PGV7_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`patt::R`](R) reader structure"]
impl crate::Readable for PATT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`patt::W`](W) writer structure"]
impl crate::Writable for PATT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PATT to value 0"]
impl crate::Resettable for PATT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
