#[doc = "Register `GFC` reader"]
pub type R = crate::R<GfcSpec>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GfcSpec>;
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Accept Non-matching Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfeselect {
    #[doc = "0: Accept in Rx FIFO 0"]
    Rxf0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    Rxf1 = 1,
    #[doc = "2: Reject"]
    Reject = 2,
}
impl From<Anfeselect> for u8 {
    #[inline(always)]
    fn from(variant: Anfeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfeselect {
    type Ux = u8;
}
impl crate::IsEnum for Anfeselect {}
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type AnfeR = crate::FieldReader<Anfeselect>;
impl AnfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anfeselect> {
        match self.bits {
            0 => Some(Anfeselect::Rxf0),
            1 => Some(Anfeselect::Rxf1),
            2 => Some(Anfeselect::Reject),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == Anfeselect::Rxf0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == Anfeselect::Rxf1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == Anfeselect::Reject
    }
}
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfeselect>;
impl<'a, REG> AnfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfeselect::Rxf0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfeselect::Rxf1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(Anfeselect::Reject)
    }
}
#[doc = "Accept Non-matching Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anfsselect {
    #[doc = "0: Accept in Rx FIFO 0"]
    Rxf0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    Rxf1 = 1,
    #[doc = "2: Reject"]
    Reject = 2,
}
impl From<Anfsselect> for u8 {
    #[inline(always)]
    fn from(variant: Anfsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anfsselect {
    type Ux = u8;
}
impl crate::IsEnum for Anfsselect {}
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type AnfsR = crate::FieldReader<Anfsselect>;
impl AnfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anfsselect> {
        match self.bits {
            0 => Some(Anfsselect::Rxf0),
            1 => Some(Anfsselect::Rxf1),
            2 => Some(Anfsselect::Reject),
            _ => None,
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == Anfsselect::Rxf0
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == Anfsselect::Rxf1
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == Anfsselect::Reject
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anfsselect>;
impl<'a, REG> AnfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(Anfsselect::Rxf0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(Anfsselect::Rxf1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn reject(self) -> &'a mut crate::W<REG> {
        self.variant(Anfsselect::Reject)
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RrfeW<GfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RrfsW<GfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> AnfeW<GfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> AnfsW<GfcSpec> {
        AnfsW::new(self, 4)
    }
}
#[doc = "Global Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfcSpec;
impl crate::RegisterSpec for GfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GfcSpec {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GfcSpec {
    const RESET_VALUE: u32 = 0;
}
