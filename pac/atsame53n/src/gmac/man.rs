#[doc = "Register `MAN` reader"]
pub type R = crate::R<MAN_SPEC>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<MAN_SPEC>;
#[doc = "Field `DATA` reader - PHY Data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - PHY Data"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `WTN` reader - Write Ten"]
pub type WTN_R = crate::FieldReader;
#[doc = "Field `WTN` writer - Write Ten"]
pub type WTN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REGA` reader - Register Address"]
pub type REGA_R = crate::FieldReader;
#[doc = "Field `REGA` writer - Register Address"]
pub type REGA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PHYA` reader - PHY Address"]
pub type PHYA_R = crate::FieldReader;
#[doc = "Field `PHYA` writer - PHY Address"]
pub type PHYA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OP` reader - Operation"]
pub type OP_R = crate::FieldReader;
#[doc = "Field `OP` writer - Operation"]
pub type OP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLTTO` reader - Clause 22 Operation"]
pub type CLTTO_R = crate::BitReader;
#[doc = "Field `CLTTO` writer - Clause 22 Operation"]
pub type CLTTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WZO` reader - Write ZERO"]
pub type WZO_R = crate::BitReader;
#[doc = "Field `WZO` writer - Write ZERO"]
pub type WZO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&self) -> WTN_R {
        WTN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> REGA_R {
        REGA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PHYA_R {
        PHYA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&self) -> CLTTO_R {
        CLTTO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&self) -> WZO_R {
        WZO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<MAN_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    #[must_use]
    pub fn wtn(&mut self) -> WTN_W<MAN_SPEC, 16> {
        WTN_W::new(self)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn rega(&mut self) -> REGA_W<MAN_SPEC, 18> {
        REGA_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phya(&mut self) -> PHYA_W<MAN_SPEC, 23> {
        PHYA_W::new(self)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    #[must_use]
    pub fn op(&mut self) -> OP_W<MAN_SPEC, 28> {
        OP_W::new(self)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    #[must_use]
    pub fn cltto(&mut self) -> CLTTO_W<MAN_SPEC, 30> {
        CLTTO_W::new(self)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    #[must_use]
    pub fn wzo(&mut self) -> WZO_W<MAN_SPEC, 31> {
        WZO_W::new(self)
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
#[doc = "PHY Maintenance Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for MAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
