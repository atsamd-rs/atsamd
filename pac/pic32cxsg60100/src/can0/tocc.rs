#[doc = "Register `TOCC` reader"]
pub type R = crate::R<ToccSpec>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<ToccSpec>;
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type EtocR = crate::BitReader;
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tosselect {
    #[doc = "0: Continuout operation"]
    Cont = 0,
    #[doc = "1: Timeout controlled by TX Event FIFO"]
    Txef = 1,
    #[doc = "2: Timeout controlled by Rx FIFO 0"]
    Rxf0 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO 1"]
    Rxf1 = 3,
}
impl From<Tosselect> for u8 {
    #[inline(always)]
    fn from(variant: Tosselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tosselect {
    type Ux = u8;
}
impl crate::IsEnum for Tosselect {}
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TosR = crate::FieldReader<Tosselect>;
impl TosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tosselect {
        match self.bits {
            0 => Tosselect::Cont,
            1 => Tosselect::Txef,
            2 => Tosselect::Rxf0,
            3 => Tosselect::Rxf1,
            _ => unreachable!(),
        }
    }
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Tosselect::Cont
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn is_txef(&self) -> bool {
        *self == Tosselect::Txef
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn is_rxf0(&self) -> bool {
        *self == Tosselect::Rxf0
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn is_rxf1(&self) -> bool {
        *self == Tosselect::Rxf1
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tosselect, crate::Safe>;
impl<'a, REG> TosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Continuout operation"]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Cont)
    }
    #[doc = "Timeout controlled by TX Event FIFO"]
    #[inline(always)]
    pub fn txef(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Txef)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn rxf0(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Rxf0)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn rxf1(self) -> &'a mut crate::W<REG> {
        self.variant(Tosselect::Rxf1)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> EtocW<ToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TosW<ToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<ToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "Timeout Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToccSpec;
impl crate::RegisterSpec for ToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for ToccSpec {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for ToccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for ToccSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
