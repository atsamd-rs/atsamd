#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `LBCK` reader - Loop Back Mode"]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - Loop Back Mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control of Transmit Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txselect {
    #[doc = "0: TX controlled by CAN core"]
    Core = 0,
    #[doc = "1: TX monitoring sample point"]
    Sample = 1,
    #[doc = "2: Dominant (0) level at pin CAN_TX"]
    Dominant = 2,
    #[doc = "3: Recessive (1) level at pin CAN_TX"]
    Recessive = 3,
}
impl From<Txselect> for u8 {
    #[inline(always)]
    fn from(variant: Txselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txselect {
    type Ux = u8;
}
impl crate::IsEnum for Txselect {}
#[doc = "Field `TX` reader - Control of Transmit Pin"]
pub type TxR = crate::FieldReader<Txselect>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txselect {
        match self.bits {
            0 => Txselect::Core,
            1 => Txselect::Sample,
            2 => Txselect::Dominant,
            3 => Txselect::Recessive,
            _ => unreachable!(),
        }
    }
    #[doc = "TX controlled by CAN core"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == Txselect::Core
    }
    #[doc = "TX monitoring sample point"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == Txselect::Sample
    }
    #[doc = "Dominant (0) level at pin CAN_TX"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == Txselect::Dominant
    }
    #[doc = "Recessive (1) level at pin CAN_TX"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == Txselect::Recessive
    }
}
#[doc = "Field `TX` writer - Control of Transmit Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txselect, crate::Safe>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX controlled by CAN core"]
    #[inline(always)]
    pub fn core(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Core)
    }
    #[doc = "TX monitoring sample point"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Sample)
    }
    #[doc = "Dominant (0) level at pin CAN_TX"]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Dominant)
    }
    #[doc = "Recessive (1) level at pin CAN_TX"]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut crate::W<REG> {
        self.variant(Txselect::Recessive)
    }
}
#[doc = "Field `RX` reader - Receive Pin"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - Receive Pin"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LbckW<TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<TestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - Receive Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<TestSpec> {
        RxW::new(self, 7)
    }
}
#[doc = "Test\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
