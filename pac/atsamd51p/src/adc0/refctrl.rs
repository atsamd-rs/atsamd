#[doc = "Register `REFCTRL` reader"]
pub type R = crate::R<RefctrlSpec>;
#[doc = "Register `REFCTRL` writer"]
pub type W = crate::W<RefctrlSpec>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: Internal Bandgap Reference"]
    Intref = 0,
    #[doc = "2: 1/2 VDDANA"]
    Intvcc0 = 2,
    #[doc = "3: VDDANA"]
    Intvcc1 = 3,
    #[doc = "4: External Reference A"]
    Arefa = 4,
    #[doc = "5: External Reference B"]
    Arefb = 5,
    #[doc = "6: External Reference C (only on ADC1)"]
    Arefc = 6,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refselselect> {
        match self.bits {
            0 => Some(Refselselect::Intref),
            2 => Some(Refselselect::Intvcc0),
            3 => Some(Refselselect::Intvcc1),
            4 => Some(Refselselect::Arefa),
            5 => Some(Refselselect::Arefb),
            6 => Some(Refselselect::Arefc),
            _ => None,
        }
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Refselselect::Intref
    }
    #[doc = "1/2 VDDANA"]
    #[inline(always)]
    pub fn is_intvcc0(&self) -> bool {
        *self == Refselselect::Intvcc0
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn is_intvcc1(&self) -> bool {
        *self == Refselselect::Intvcc1
    }
    #[doc = "External Reference A"]
    #[inline(always)]
    pub fn is_arefa(&self) -> bool {
        *self == Refselselect::Arefa
    }
    #[doc = "External Reference B"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == Refselselect::Arefb
    }
    #[doc = "External Reference C (only on ADC1)"]
    #[inline(always)]
    pub fn is_arefc(&self) -> bool {
        *self == Refselselect::Arefc
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Refselselect>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intref)
    }
    #[doc = "1/2 VDDANA"]
    #[inline(always)]
    pub fn intvcc0(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intvcc0)
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn intvcc1(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intvcc1)
    }
    #[doc = "External Reference A"]
    #[inline(always)]
    pub fn arefa(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Arefa)
    }
    #[doc = "External Reference B"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Arefb)
    }
    #[doc = "External Reference C (only on ADC1)"]
    #[inline(always)]
    pub fn arefc(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Arefc)
    }
}
#[doc = "Field `REFCOMP` reader - Reference Buffer Offset Compensation Enable"]
pub type RefcompR = crate::BitReader;
#[doc = "Field `REFCOMP` writer - Reference Buffer Offset Compensation Enable"]
pub type RefcompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&self) -> RefcompR {
        RefcompR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<RefctrlSpec> {
        RefselW::new(self, 0)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline(always)]
    pub fn refcomp(&mut self) -> RefcompW<RefctrlSpec> {
        RefcompW::new(self, 7)
    }
}
#[doc = "Reference Control\n\nYou can [`read`](crate::Reg::read) this register and get [`refctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefctrlSpec;
impl crate::RegisterSpec for RefctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`refctrl::R`](R) reader structure"]
impl crate::Readable for RefctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`refctrl::W`](W) writer structure"]
impl crate::Writable for RefctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFCTRL to value 0"]
impl crate::Resettable for RefctrlSpec {}
