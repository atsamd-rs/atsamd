#[doc = "Register `ICSR` reader"]
pub type R = crate::R<IcsrSpec>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<IcsrSpec>;
#[doc = "Field `VECTACTIVE` reader - Active exception number"]
pub type VectactiveR = crate::FieldReader<u16>;
#[doc = "Field `VECTACTIVE` writer - Active exception number"]
pub type VectactiveW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RETTOBASE` reader - No preempted active exceptions to execute"]
pub type RettobaseR = crate::BitReader;
#[doc = "Field `RETTOBASE` writer - No preempted active exceptions to execute"]
pub type RettobaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub type VectpendingR = crate::FieldReader;
#[doc = "Field `VECTPENDING` writer - Exception number of the highest priority pending enabled exception"]
pub type VectpendingW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ISRPENDING` reader - Interrupt pending flag"]
pub type IsrpendingR = crate::BitReader;
#[doc = "Field `ISRPENDING` writer - Interrupt pending flag"]
pub type IsrpendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISRPREEMPT` reader - Debug only"]
pub type IsrpreemptR = crate::BitReader;
#[doc = "Field `ISRPREEMPT` writer - Debug only"]
pub type IsrpreemptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SysTick clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstclrselect {
    #[doc = "0: No effect"]
    Value0 = 0,
    #[doc = "1: Removes the pending state from the SysTick exception"]
    Value1 = 1,
}
impl From<Pendstclrselect> for bool {
    #[inline(always)]
    fn from(variant: Pendstclrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` reader - SysTick clear-pending bit"]
pub type PendstclrR = crate::BitReader<Pendstclrselect>;
impl PendstclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendstclrselect {
        match self.bits {
            false => Pendstclrselect::Value0,
            true => Pendstclrselect::Value1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Pendstclrselect::Value0
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pendstclrselect::Value1
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick clear-pending bit"]
pub type PendstclrW<'a, REG> = crate::BitWriter<'a, REG, Pendstclrselect>;
impl<'a, REG> PendstclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclrselect::Value0)
    }
    #[doc = "Removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclrselect::Value1)
    }
}
#[doc = "SysTick set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstsetselect {
    #[doc = "0: Write: no effect; read: SysTick exception is not pending"]
    Value0 = 0,
    #[doc = "1: Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    Value1 = 1,
}
impl From<Pendstsetselect> for bool {
    #[inline(always)]
    fn from(variant: Pendstsetselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick set-pending bit"]
pub type PendstsetR = crate::BitReader<Pendstsetselect>;
impl PendstsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendstsetselect {
        match self.bits {
            false => Pendstsetselect::Value0,
            true => Pendstsetselect::Value1,
        }
    }
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Pendstsetselect::Value0
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pendstsetselect::Value1
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick set-pending bit"]
pub type PendstsetW<'a, REG> = crate::BitWriter<'a, REG, Pendstsetselect>;
impl<'a, REG> PendstsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstsetselect::Value0)
    }
    #[doc = "Write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstsetselect::Value1)
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvclrselect {
    #[doc = "0: No effect"]
    Value0 = 0,
    #[doc = "1: Removes the pending state from the PendSV exception"]
    Value1 = 1,
}
impl From<Pendsvclrselect> for bool {
    #[inline(always)]
    fn from(variant: Pendsvclrselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` reader - PendSV clear-pending bit"]
pub type PendsvclrR = crate::BitReader<Pendsvclrselect>;
impl PendsvclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendsvclrselect {
        match self.bits {
            false => Pendsvclrselect::Value0,
            true => Pendsvclrselect::Value1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Pendsvclrselect::Value0
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pendsvclrselect::Value1
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub type PendsvclrW<'a, REG> = crate::BitWriter<'a, REG, Pendsvclrselect>;
impl<'a, REG> PendsvclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclrselect::Value0)
    }
    #[doc = "Removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclrselect::Value1)
    }
}
#[doc = "PendSV set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvsetselect {
    #[doc = "0: Write: no effect; read: PendSV exception is not pending"]
    Value0 = 0,
    #[doc = "1: Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    Value1 = 1,
}
impl From<Pendsvsetselect> for bool {
    #[inline(always)]
    fn from(variant: Pendsvsetselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit"]
pub type PendsvsetR = crate::BitReader<Pendsvsetselect>;
impl PendsvsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendsvsetselect {
        match self.bits {
            false => Pendsvsetselect::Value0,
            true => Pendsvsetselect::Value1,
        }
    }
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Pendsvsetselect::Value0
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Pendsvsetselect::Value1
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit"]
pub type PendsvsetW<'a, REG> = crate::BitWriter<'a, REG, Pendsvsetselect>;
impl<'a, REG> PendsvsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvsetselect::Value0)
    }
    #[doc = "Write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvsetselect::Value1)
    }
}
#[doc = "NMI set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmipendsetselect {
    #[doc = "0: Write: no effect; read: NMI exception is not pending"]
    Value0 = 0,
    #[doc = "1: Write: changes NMI exception state to pending; read: NMI exception is pending"]
    Value1 = 1,
}
impl From<Nmipendsetselect> for bool {
    #[inline(always)]
    fn from(variant: Nmipendsetselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit"]
pub type NmipendsetR = crate::BitReader<Nmipendsetselect>;
impl NmipendsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmipendsetselect {
        match self.bits {
            false => Nmipendsetselect::Value0,
            true => Nmipendsetselect::Value1,
        }
    }
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Nmipendsetselect::Value0
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Nmipendsetselect::Value1
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit"]
pub type NmipendsetW<'a, REG> = crate::BitWriter<'a, REG, Nmipendsetselect>;
impl<'a, REG> NmipendsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmipendsetselect::Value0)
    }
    #[doc = "Write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmipendsetselect::Value1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VectactiveR {
        VectactiveR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - No preempted active exceptions to execute"]
    #[inline(always)]
    pub fn rettobase(&self) -> RettobaseR {
        RettobaseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VectpendingR {
        VectpendingR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> IsrpendingR {
        IsrpendingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> IsrpreemptR {
        IsrpreemptR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PendstclrR {
        PendstclrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PendstsetR {
        PendstsetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PendsvclrR {
        PendsvclrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PendsvsetR {
        PendsvsetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NmipendsetR {
        NmipendsetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&mut self) -> VectactiveW<IcsrSpec> {
        VectactiveW::new(self, 0)
    }
    #[doc = "Bit 11 - No preempted active exceptions to execute"]
    #[inline(always)]
    pub fn rettobase(&mut self) -> RettobaseW<IcsrSpec> {
        RettobaseW::new(self, 11)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&mut self) -> VectpendingW<IcsrSpec> {
        VectpendingW::new(self, 12)
    }
    #[doc = "Bit 22 - Interrupt pending flag"]
    #[inline(always)]
    pub fn isrpending(&mut self) -> IsrpendingW<IcsrSpec> {
        IsrpendingW::new(self, 22)
    }
    #[doc = "Bit 23 - Debug only"]
    #[inline(always)]
    pub fn isrpreempt(&mut self) -> IsrpreemptW<IcsrSpec> {
        IsrpreemptW::new(self, 23)
    }
    #[doc = "Bit 25 - SysTick clear-pending bit"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PendstclrW<IcsrSpec> {
        PendstclrW::new(self, 25)
    }
    #[doc = "Bit 26 - SysTick set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PendstsetW<IcsrSpec> {
        PendstsetW::new(self, 26)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PendsvclrW<IcsrSpec> {
        PendsvclrW::new(self, 27)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PendsvsetW<IcsrSpec> {
        PendsvsetW::new(self, 28)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> NmipendsetW<IcsrSpec> {
        NmipendsetW::new(self, 31)
    }
}
#[doc = "Interrupt Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsrSpec;
impl crate::RegisterSpec for IcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for IcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for IcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for IcsrSpec {}
