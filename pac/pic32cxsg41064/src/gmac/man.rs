#[doc = "Register `MAN` reader"]
pub type R = crate::R<ManSpec>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<ManSpec>;
#[doc = "Field `DATA` reader - PHY Data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - PHY Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WTN` reader - Write Ten"]
pub type WtnR = crate::FieldReader;
#[doc = "Field `WTN` writer - Write Ten"]
pub type WtnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGA` reader - Register Address"]
pub type RegaR = crate::FieldReader;
#[doc = "Field `REGA` writer - Register Address"]
pub type RegaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHYA` reader - PHY Address"]
pub type PhyaR = crate::FieldReader;
#[doc = "Field `PHYA` writer - PHY Address"]
pub type PhyaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OP` reader - Operation"]
pub type OpR = crate::FieldReader;
#[doc = "Field `OP` writer - Operation"]
pub type OpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLTTO` reader - Clause 22 Operation"]
pub type ClttoR = crate::BitReader;
#[doc = "Field `CLTTO` writer - Clause 22 Operation"]
pub type ClttoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WZO` reader - Write ZERO"]
pub type WzoR = crate::BitReader;
#[doc = "Field `WZO` writer - Write ZERO"]
pub type WzoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    pub fn wtn(&self) -> WtnR {
        WtnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> RegaR {
        RegaR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PhyaR {
        PhyaR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    pub fn op(&self) -> OpR {
        OpR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    pub fn cltto(&self) -> ClttoR {
        ClttoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    pub fn wzo(&self) -> WzoR {
        WzoR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<ManSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Write Ten"]
    #[inline(always)]
    #[must_use]
    pub fn wtn(&mut self) -> WtnW<ManSpec> {
        WtnW::new(self, 16)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn rega(&mut self) -> RegaW<ManSpec> {
        RegaW::new(self, 18)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phya(&mut self) -> PhyaW<ManSpec> {
        PhyaW::new(self, 23)
    }
    #[doc = "Bits 28:29 - Operation"]
    #[inline(always)]
    #[must_use]
    pub fn op(&mut self) -> OpW<ManSpec> {
        OpW::new(self, 28)
    }
    #[doc = "Bit 30 - Clause 22 Operation"]
    #[inline(always)]
    #[must_use]
    pub fn cltto(&mut self) -> ClttoW<ManSpec> {
        ClttoW::new(self, 30)
    }
    #[doc = "Bit 31 - Write ZERO"]
    #[inline(always)]
    #[must_use]
    pub fn wzo(&mut self) -> WzoW<ManSpec> {
        WzoW::new(self, 31)
    }
}
#[doc = "PHY Maintenance Register\n\nYou can [`read`](crate::Reg::read) this register and get [`man::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ManSpec;
impl crate::RegisterSpec for ManSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for ManSpec {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for ManSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for ManSpec {
    const RESET_VALUE: u32 = 0;
}
