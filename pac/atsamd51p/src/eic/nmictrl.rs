#[doc = "Register `NMICTRL` reader"]
pub type R = crate::R<NmictrlSpec>;
#[doc = "Register `NMICTRL` writer"]
pub type W = crate::W<NmictrlSpec>;
#[doc = "Non-Maskable Interrupt Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nmisenseselect {
    #[doc = "0: No detection"]
    None = 0,
    #[doc = "1: Rising-edge detection"]
    Rise = 1,
    #[doc = "2: Falling-edge detection"]
    Fall = 2,
    #[doc = "3: Both-edges detection"]
    Both = 3,
    #[doc = "4: High-level detection"]
    High = 4,
    #[doc = "5: Low-level detection"]
    Low = 5,
}
impl From<Nmisenseselect> for u8 {
    #[inline(always)]
    fn from(variant: Nmisenseselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nmisenseselect {
    type Ux = u8;
}
impl crate::IsEnum for Nmisenseselect {}
#[doc = "Field `NMISENSE` reader - Non-Maskable Interrupt Sense Configuration"]
pub type NmisenseR = crate::FieldReader<Nmisenseselect>;
impl NmisenseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nmisenseselect> {
        match self.bits {
            0 => Some(Nmisenseselect::None),
            1 => Some(Nmisenseselect::Rise),
            2 => Some(Nmisenseselect::Fall),
            3 => Some(Nmisenseselect::Both),
            4 => Some(Nmisenseselect::High),
            5 => Some(Nmisenseselect::Low),
            _ => None,
        }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nmisenseselect::None
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Nmisenseselect::Rise
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Nmisenseselect::Fall
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Nmisenseselect::Both
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Nmisenseselect::High
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Nmisenseselect::Low
    }
}
#[doc = "Field `NMISENSE` writer - Non-Maskable Interrupt Sense Configuration"]
pub type NmisenseW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nmisenseselect>;
impl<'a, REG> NmisenseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::None)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::Rise)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::Fall)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::Both)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::High)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Nmisenseselect::Low)
    }
}
#[doc = "Field `NMIFILTEN` reader - Non-Maskable Interrupt Filter Enable"]
pub type NmifiltenR = crate::BitReader;
#[doc = "Field `NMIFILTEN` writer - Non-Maskable Interrupt Filter Enable"]
pub type NmifiltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Asynchronous Edge Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmiasynchselect {
    #[doc = "0: Edge detection is clock synchronously operated"]
    Sync = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    Async = 1,
}
impl From<Nmiasynchselect> for bool {
    #[inline(always)]
    fn from(variant: Nmiasynchselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIASYNCH` reader - Asynchronous Edge Detection Mode"]
pub type NmiasynchR = crate::BitReader<Nmiasynchselect>;
impl NmiasynchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmiasynchselect {
        match self.bits {
            false => Nmiasynchselect::Sync,
            true => Nmiasynchselect::Async,
        }
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Nmiasynchselect::Sync
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Nmiasynchselect::Async
    }
}
#[doc = "Field `NMIASYNCH` writer - Asynchronous Edge Detection Mode"]
pub type NmiasynchW<'a, REG> = crate::BitWriter<'a, REG, Nmiasynchselect>;
impl<'a, REG> NmiasynchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiasynchselect::Sync)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Nmiasynchselect::Async)
    }
}
impl R {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&self) -> NmisenseR {
        NmisenseR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NmifiltenR {
        NmifiltenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    pub fn nmiasynch(&self) -> NmiasynchR {
        NmiasynchR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Non-Maskable Interrupt Sense Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nmisense(&mut self) -> NmisenseW<NmictrlSpec> {
        NmisenseW::new(self, 0)
    }
    #[doc = "Bit 3 - Non-Maskable Interrupt Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmifilten(&mut self) -> NmifiltenW<NmictrlSpec> {
        NmifiltenW::new(self, 3)
    }
    #[doc = "Bit 4 - Asynchronous Edge Detection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn nmiasynch(&mut self) -> NmiasynchW<NmictrlSpec> {
        NmiasynchW::new(self, 4)
    }
}
#[doc = "Non-Maskable Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`nmictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmictrlSpec;
impl crate::RegisterSpec for NmictrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nmictrl::R`](R) reader structure"]
impl crate::Readable for NmictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`nmictrl::W`](W) writer structure"]
impl crate::Writable for NmictrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NmictrlSpec {
    const RESET_VALUE: u8 = 0;
}
