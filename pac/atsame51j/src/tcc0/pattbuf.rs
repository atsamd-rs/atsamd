#[doc = "Register `PATTBUF` reader"]
pub type R = crate::R<PATTBUF_SPEC>;
#[doc = "Register `PATTBUF` writer"]
pub type W = crate::W<PATTBUF_SPEC>;
#[doc = "Field `PGEB0` reader - Pattern Generator 0 Output Enable Buffer"]
pub type PGEB0_R = crate::BitReader;
#[doc = "Field `PGEB0` writer - Pattern Generator 0 Output Enable Buffer"]
pub type PGEB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB1` reader - Pattern Generator 1 Output Enable Buffer"]
pub type PGEB1_R = crate::BitReader;
#[doc = "Field `PGEB1` writer - Pattern Generator 1 Output Enable Buffer"]
pub type PGEB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB2` reader - Pattern Generator 2 Output Enable Buffer"]
pub type PGEB2_R = crate::BitReader;
#[doc = "Field `PGEB2` writer - Pattern Generator 2 Output Enable Buffer"]
pub type PGEB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB3` reader - Pattern Generator 3 Output Enable Buffer"]
pub type PGEB3_R = crate::BitReader;
#[doc = "Field `PGEB3` writer - Pattern Generator 3 Output Enable Buffer"]
pub type PGEB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB4` reader - Pattern Generator 4 Output Enable Buffer"]
pub type PGEB4_R = crate::BitReader;
#[doc = "Field `PGEB4` writer - Pattern Generator 4 Output Enable Buffer"]
pub type PGEB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB5` reader - Pattern Generator 5 Output Enable Buffer"]
pub type PGEB5_R = crate::BitReader;
#[doc = "Field `PGEB5` writer - Pattern Generator 5 Output Enable Buffer"]
pub type PGEB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB6` reader - Pattern Generator 6 Output Enable Buffer"]
pub type PGEB6_R = crate::BitReader;
#[doc = "Field `PGEB6` writer - Pattern Generator 6 Output Enable Buffer"]
pub type PGEB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGEB7` reader - Pattern Generator 7 Output Enable Buffer"]
pub type PGEB7_R = crate::BitReader;
#[doc = "Field `PGEB7` writer - Pattern Generator 7 Output Enable Buffer"]
pub type PGEB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB0` reader - Pattern Generator 0 Output Enable"]
pub type PGVB0_R = crate::BitReader;
#[doc = "Field `PGVB0` writer - Pattern Generator 0 Output Enable"]
pub type PGVB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB1` reader - Pattern Generator 1 Output Enable"]
pub type PGVB1_R = crate::BitReader;
#[doc = "Field `PGVB1` writer - Pattern Generator 1 Output Enable"]
pub type PGVB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB2` reader - Pattern Generator 2 Output Enable"]
pub type PGVB2_R = crate::BitReader;
#[doc = "Field `PGVB2` writer - Pattern Generator 2 Output Enable"]
pub type PGVB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB3` reader - Pattern Generator 3 Output Enable"]
pub type PGVB3_R = crate::BitReader;
#[doc = "Field `PGVB3` writer - Pattern Generator 3 Output Enable"]
pub type PGVB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB4` reader - Pattern Generator 4 Output Enable"]
pub type PGVB4_R = crate::BitReader;
#[doc = "Field `PGVB4` writer - Pattern Generator 4 Output Enable"]
pub type PGVB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB5` reader - Pattern Generator 5 Output Enable"]
pub type PGVB5_R = crate::BitReader;
#[doc = "Field `PGVB5` writer - Pattern Generator 5 Output Enable"]
pub type PGVB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB6` reader - Pattern Generator 6 Output Enable"]
pub type PGVB6_R = crate::BitReader;
#[doc = "Field `PGVB6` writer - Pattern Generator 6 Output Enable"]
pub type PGVB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGVB7` reader - Pattern Generator 7 Output Enable"]
pub type PGVB7_R = crate::BitReader;
#[doc = "Field `PGVB7` writer - Pattern Generator 7 Output Enable"]
pub type PGVB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb0(&self) -> PGEB0_R {
        PGEB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb1(&self) -> PGEB1_R {
        PGEB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb2(&self) -> PGEB2_R {
        PGEB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb3(&self) -> PGEB3_R {
        PGEB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb4(&self) -> PGEB4_R {
        PGEB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb5(&self) -> PGEB5_R {
        PGEB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb6(&self) -> PGEB6_R {
        PGEB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    pub fn pgeb7(&self) -> PGEB7_R {
        PGEB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    pub fn pgvb0(&self) -> PGVB0_R {
        PGVB0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    pub fn pgvb1(&self) -> PGVB1_R {
        PGVB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    pub fn pgvb2(&self) -> PGVB2_R {
        PGVB2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    pub fn pgvb3(&self) -> PGVB3_R {
        PGVB3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    pub fn pgvb4(&self) -> PGVB4_R {
        PGVB4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    pub fn pgvb5(&self) -> PGVB5_R {
        PGVB5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    pub fn pgvb6(&self) -> PGVB6_R {
        PGVB6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    pub fn pgvb7(&self) -> PGVB7_R {
        PGVB7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern Generator 0 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb0(&mut self) -> PGEB0_W<PATTBUF_SPEC, 0> {
        PGEB0_W::new(self)
    }
    #[doc = "Bit 1 - Pattern Generator 1 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb1(&mut self) -> PGEB1_W<PATTBUF_SPEC, 1> {
        PGEB1_W::new(self)
    }
    #[doc = "Bit 2 - Pattern Generator 2 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb2(&mut self) -> PGEB2_W<PATTBUF_SPEC, 2> {
        PGEB2_W::new(self)
    }
    #[doc = "Bit 3 - Pattern Generator 3 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb3(&mut self) -> PGEB3_W<PATTBUF_SPEC, 3> {
        PGEB3_W::new(self)
    }
    #[doc = "Bit 4 - Pattern Generator 4 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb4(&mut self) -> PGEB4_W<PATTBUF_SPEC, 4> {
        PGEB4_W::new(self)
    }
    #[doc = "Bit 5 - Pattern Generator 5 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb5(&mut self) -> PGEB5_W<PATTBUF_SPEC, 5> {
        PGEB5_W::new(self)
    }
    #[doc = "Bit 6 - Pattern Generator 6 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb6(&mut self) -> PGEB6_W<PATTBUF_SPEC, 6> {
        PGEB6_W::new(self)
    }
    #[doc = "Bit 7 - Pattern Generator 7 Output Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn pgeb7(&mut self) -> PGEB7_W<PATTBUF_SPEC, 7> {
        PGEB7_W::new(self)
    }
    #[doc = "Bit 8 - Pattern Generator 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb0(&mut self) -> PGVB0_W<PATTBUF_SPEC, 8> {
        PGVB0_W::new(self)
    }
    #[doc = "Bit 9 - Pattern Generator 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb1(&mut self) -> PGVB1_W<PATTBUF_SPEC, 9> {
        PGVB1_W::new(self)
    }
    #[doc = "Bit 10 - Pattern Generator 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb2(&mut self) -> PGVB2_W<PATTBUF_SPEC, 10> {
        PGVB2_W::new(self)
    }
    #[doc = "Bit 11 - Pattern Generator 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb3(&mut self) -> PGVB3_W<PATTBUF_SPEC, 11> {
        PGVB3_W::new(self)
    }
    #[doc = "Bit 12 - Pattern Generator 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb4(&mut self) -> PGVB4_W<PATTBUF_SPEC, 12> {
        PGVB4_W::new(self)
    }
    #[doc = "Bit 13 - Pattern Generator 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb5(&mut self) -> PGVB5_W<PATTBUF_SPEC, 13> {
        PGVB5_W::new(self)
    }
    #[doc = "Bit 14 - Pattern Generator 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb6(&mut self) -> PGVB6_W<PATTBUF_SPEC, 14> {
        PGVB6_W::new(self)
    }
    #[doc = "Bit 15 - Pattern Generator 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgvb7(&mut self) -> PGVB7_W<PATTBUF_SPEC, 15> {
        PGVB7_W::new(self)
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
#[doc = "Pattern Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pattbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pattbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATTBUF_SPEC;
impl crate::RegisterSpec for PATTBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pattbuf::R`](R) reader structure"]
impl crate::Readable for PATTBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pattbuf::W`](W) writer structure"]
impl crate::Writable for PATTBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PATTBUF to value 0"]
impl crate::Resettable for PATTBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
