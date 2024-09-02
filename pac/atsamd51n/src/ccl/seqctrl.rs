#[doc = "Register `SEQCTRL[%s]` reader"]
pub type R = crate::R<SeqctrlSpec>;
#[doc = "Register `SEQCTRL[%s]` writer"]
pub type W = crate::W<SeqctrlSpec>;
#[doc = "Sequential Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seqselselect {
    #[doc = "0: Sequential logic is disabled"]
    Disable = 0,
    #[doc = "1: D flip flop"]
    Dff = 1,
    #[doc = "2: JK flip flop"]
    Jk = 2,
    #[doc = "3: D latch"]
    Latch = 3,
    #[doc = "4: RS latch"]
    Rs = 4,
}
impl From<Seqselselect> for u8 {
    #[inline(always)]
    fn from(variant: Seqselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seqselselect {
    type Ux = u8;
}
impl crate::IsEnum for Seqselselect {}
#[doc = "Field `SEQSEL` reader - Sequential Selection"]
pub type SeqselR = crate::FieldReader<Seqselselect>;
impl SeqselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seqselselect> {
        match self.bits {
            0 => Some(Seqselselect::Disable),
            1 => Some(Seqselselect::Dff),
            2 => Some(Seqselselect::Jk),
            3 => Some(Seqselselect::Latch),
            4 => Some(Seqselselect::Rs),
            _ => None,
        }
    }
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Seqselselect::Disable
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn is_dff(&self) -> bool {
        *self == Seqselselect::Dff
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn is_jk(&self) -> bool {
        *self == Seqselselect::Jk
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn is_latch(&self) -> bool {
        *self == Seqselselect::Latch
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn is_rs(&self) -> bool {
        *self == Seqselselect::Rs
    }
}
#[doc = "Field `SEQSEL` writer - Sequential Selection"]
pub type SeqselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Seqselselect>;
impl<'a, REG> SeqselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sequential logic is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Seqselselect::Disable)
    }
    #[doc = "D flip flop"]
    #[inline(always)]
    pub fn dff(self) -> &'a mut crate::W<REG> {
        self.variant(Seqselselect::Dff)
    }
    #[doc = "JK flip flop"]
    #[inline(always)]
    pub fn jk(self) -> &'a mut crate::W<REG> {
        self.variant(Seqselselect::Jk)
    }
    #[doc = "D latch"]
    #[inline(always)]
    pub fn latch(self) -> &'a mut crate::W<REG> {
        self.variant(Seqselselect::Latch)
    }
    #[doc = "RS latch"]
    #[inline(always)]
    pub fn rs(self) -> &'a mut crate::W<REG> {
        self.variant(Seqselselect::Rs)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    pub fn seqsel(&self) -> SeqselR {
        SeqselR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequential Selection"]
    #[inline(always)]
    #[must_use]
    pub fn seqsel(&mut self) -> SeqselW<SeqctrlSpec> {
        SeqselW::new(self, 0)
    }
}
#[doc = "SEQ Control x\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlSpec;
impl crate::RegisterSpec for SeqctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SeqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SeqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SEQCTRL[%s]
to value 0"]
impl crate::Resettable for SeqctrlSpec {
    const RESET_VALUE: u8 = 0;
}
