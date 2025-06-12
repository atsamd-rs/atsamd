#[doc = "Register `CFSR` reader"]
pub type R = crate::R<CfsrSpec>;
#[doc = "Register `CFSR` writer"]
pub type W = crate::W<CfsrSpec>;
#[doc = "Field `IACCVIOL` reader - Instruction access violation"]
pub type IaccviolR = crate::BitReader;
#[doc = "Field `IACCVIOL` writer - Instruction access violation"]
pub type IaccviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCVIOL` reader - Data access violation"]
pub type DaccviolR = crate::BitReader;
#[doc = "Field `DACCVIOL` writer - Data access violation"]
pub type DaccviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUNSTKERR` reader - MemManage Fault on unstacking for exception return"]
pub type MunstkerrR = crate::BitReader;
#[doc = "Field `MUNSTKERR` writer - MemManage Fault on unstacking for exception return"]
pub type MunstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTKERR` reader - MemManage Fault on stacking for exception entry"]
pub type MstkerrR = crate::BitReader;
#[doc = "Field `MSTKERR` writer - MemManage Fault on stacking for exception entry"]
pub type MstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MLSPERR` reader - MemManager Fault occured during FP lazy state preservation"]
pub type MlsperrR = crate::BitReader;
#[doc = "Field `MLSPERR` writer - MemManager Fault occured during FP lazy state preservation"]
pub type MlsperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMARVALID` reader - MemManage Fault Address Register valid"]
pub type MmarvalidR = crate::BitReader;
#[doc = "Field `MMARVALID` writer - MemManage Fault Address Register valid"]
pub type MmarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUSERR` reader - Instruction bus error"]
pub type IbuserrR = crate::BitReader;
#[doc = "Field `IBUSERR` writer - Instruction bus error"]
pub type IbuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRECISERR` reader - Precise data bus error"]
pub type PreciserrR = crate::BitReader;
#[doc = "Field `PRECISERR` writer - Precise data bus error"]
pub type PreciserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPRECISERR` reader - Imprecise data bus error"]
pub type ImpreciserrR = crate::BitReader;
#[doc = "Field `IMPRECISERR` writer - Imprecise data bus error"]
pub type ImpreciserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNSTKERR` reader - BusFault on unstacking for exception return"]
pub type UnstkerrR = crate::BitReader;
#[doc = "Field `UNSTKERR` writer - BusFault on unstacking for exception return"]
pub type UnstkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STKERR` reader - BusFault on stacking for exception entry"]
pub type StkerrR = crate::BitReader;
#[doc = "Field `STKERR` writer - BusFault on stacking for exception entry"]
pub type StkerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPERR` reader - BusFault occured during FP lazy state preservation"]
pub type LsperrR = crate::BitReader;
#[doc = "Field `LSPERR` writer - BusFault occured during FP lazy state preservation"]
pub type LsperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFARVALID` reader - BusFault Address Register valid"]
pub type BfarvalidR = crate::BitReader;
#[doc = "Field `BFARVALID` writer - BusFault Address Register valid"]
pub type BfarvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEFINSTR` reader - Undefined instruction UsageFault"]
pub type UndefinstrR = crate::BitReader;
#[doc = "Field `UNDEFINSTR` writer - Undefined instruction UsageFault"]
pub type UndefinstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSTATE` reader - Invalid state UsageFault"]
pub type InvstateR = crate::BitReader;
#[doc = "Field `INVSTATE` writer - Invalid state UsageFault"]
pub type InvstateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVPC` reader - Invalid PC load UsageFault"]
pub type InvpcR = crate::BitReader;
#[doc = "Field `INVPC` writer - Invalid PC load UsageFault"]
pub type InvpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCP` reader - No coprocessor UsageFault"]
pub type NocpR = crate::BitReader;
#[doc = "Field `NOCP` writer - No coprocessor UsageFault"]
pub type NocpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNALIGNED` reader - Unaligned access UsageFault"]
pub type UnalignedR = crate::BitReader;
#[doc = "Field `UNALIGNED` writer - Unaligned access UsageFault"]
pub type UnalignedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVBYZERO` reader - Divide by zero UsageFault"]
pub type DivbyzeroR = crate::BitReader;
#[doc = "Field `DIVBYZERO` writer - Divide by zero UsageFault"]
pub type DivbyzeroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Instruction access violation"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IaccviolR {
        IaccviolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data access violation"]
    #[inline(always)]
    pub fn daccviol(&self) -> DaccviolR {
        DaccviolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - MemManage Fault on unstacking for exception return"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MunstkerrR {
        MunstkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MemManage Fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MstkerrR {
        MstkerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MemManager Fault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MlsperrR {
        MlsperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register valid"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MmarvalidR {
        MmarvalidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IbuserrR {
        IbuserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PreciserrR {
        PreciserrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> ImpreciserrR {
        ImpreciserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for exception return"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UnstkerrR {
        UnstkerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> StkerrR {
        StkerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BusFault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LsperrR {
        LsperrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register valid"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BfarvalidR {
        BfarvalidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UndefinstrR {
        UndefinstrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> InvstateR {
        InvstateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&self) -> InvpcR {
        InvpcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NocpR {
        NocpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UnalignedR {
        UnalignedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DivbyzeroR {
        DivbyzeroR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> IaccviolW<CfsrSpec> {
        IaccviolW::new(self, 0)
    }
    #[doc = "Bit 1 - Data access violation"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> DaccviolW<CfsrSpec> {
        DaccviolW::new(self, 1)
    }
    #[doc = "Bit 3 - MemManage Fault on unstacking for exception return"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> MunstkerrW<CfsrSpec> {
        MunstkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - MemManage Fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> MstkerrW<CfsrSpec> {
        MstkerrW::new(self, 4)
    }
    #[doc = "Bit 5 - MemManager Fault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&mut self) -> MlsperrW<CfsrSpec> {
        MlsperrW::new(self, 5)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register valid"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> MmarvalidW<CfsrSpec> {
        MmarvalidW::new(self, 7)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> IbuserrW<CfsrSpec> {
        IbuserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> PreciserrW<CfsrSpec> {
        PreciserrW::new(self, 9)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> ImpreciserrW<CfsrSpec> {
        ImpreciserrW::new(self, 10)
    }
    #[doc = "Bit 11 - BusFault on unstacking for exception return"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> UnstkerrW<CfsrSpec> {
        UnstkerrW::new(self, 11)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> StkerrW<CfsrSpec> {
        StkerrW::new(self, 12)
    }
    #[doc = "Bit 13 - BusFault occured during FP lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LsperrW<CfsrSpec> {
        LsperrW::new(self, 13)
    }
    #[doc = "Bit 15 - BusFault Address Register valid"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> BfarvalidW<CfsrSpec> {
        BfarvalidW::new(self, 15)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> UndefinstrW<CfsrSpec> {
        UndefinstrW::new(self, 16)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&mut self) -> InvstateW<CfsrSpec> {
        InvstateW::new(self, 17)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc(&mut self) -> InvpcW<CfsrSpec> {
        InvpcW::new(self, 18)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NocpW<CfsrSpec> {
        NocpW::new(self, 19)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> UnalignedW<CfsrSpec> {
        UnalignedW::new(self, 24)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> DivbyzeroW<CfsrSpec> {
        DivbyzeroW::new(self, 25)
    }
}
#[doc = "Configurable Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfsrSpec;
impl crate::RegisterSpec for CfsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfsr::R`](R) reader structure"]
impl crate::Readable for CfsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfsr::W`](W) writer structure"]
impl crate::Writable for CfsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CfsrSpec {}
