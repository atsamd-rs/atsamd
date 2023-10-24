#[doc = "Register `ADDR` reader"]
pub type R = crate::R<ADDR_SPEC>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<ADDR_SPEC>;
#[doc = "Field `ADDR` reader - Address Value"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Address Value"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `LENEN` reader - Length Enable"]
pub type LENEN_R = crate::BitReader;
#[doc = "Field `LENEN` writer - Length Enable"]
pub type LENEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HS` reader - High Speed Mode"]
pub type HS_R = crate::BitReader;
#[doc = "Field `HS` writer - High Speed Mode"]
pub type HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TENBITEN` reader - Ten Bit Addressing Enable"]
pub type TENBITEN_R = crate::BitReader;
#[doc = "Field `TENBITEN` writer - Ten Bit Addressing Enable"]
pub type TENBITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEN` reader - Length"]
pub type LEN_R = crate::FieldReader;
#[doc = "Field `LEN` writer - Length"]
pub type LEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    pub fn lenen(&self) -> LENEN_R {
        LENEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    pub fn tenbiten(&self) -> TENBITEN_R {
        TENBITEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<ADDR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 13 - Length Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lenen(&mut self) -> LENEN_W<ADDR_SPEC, 13> {
        LENEN_W::new(self)
    }
    #[doc = "Bit 14 - High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<ADDR_SPEC, 14> {
        HS_W::new(self)
    }
    #[doc = "Bit 15 - Ten Bit Addressing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tenbiten(&mut self) -> TENBITEN_W<ADDR_SPEC, 15> {
        TENBITEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<ADDR_SPEC, 16> {
        LEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2CM Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
