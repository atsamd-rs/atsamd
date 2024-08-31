#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TsccSpec>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TsccSpec>;
#[doc = "Timestamp Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tssselect {
    #[doc = "0: Timestamp counter value always 0x0000"]
    Zero = 0,
    #[doc = "1: Timestamp counter value incremented by TCP"]
    Inc = 1,
    #[doc = "2: External timestamp counter value used"]
    Ext = 2,
}
impl From<Tssselect> for u8 {
    #[inline(always)]
    fn from(variant: Tssselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tssselect {
    type Ux = u8;
}
impl crate::IsEnum for Tssselect {}
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TssR = crate::FieldReader<Tssselect>;
impl TssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tssselect> {
        match self.bits {
            0 => Some(Tssselect::Zero),
            1 => Some(Tssselect::Inc),
            2 => Some(Tssselect::Ext),
            _ => None,
        }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Tssselect::Zero
    }
    #[doc = "Timestamp counter value incremented by TCP"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == Tssselect::Inc
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == Tssselect::Ext
    }
}
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tssselect>;
impl<'a, REG> TssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Tssselect::Zero)
    }
    #[doc = "Timestamp counter value incremented by TCP"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut crate::W<REG> {
        self.variant(Tssselect::Inc)
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(Tssselect::Ext)
    }
}
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<TsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TcpW<TsccSpec> {
        TcpW::new(self, 16)
    }
}
#[doc = "Timestamp Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsccSpec;
impl crate::RegisterSpec for TsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TsccSpec {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TsccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TsccSpec {
    const RESET_VALUE: u32 = 0;
}
