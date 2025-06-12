#[doc = "Register `PRICTRL0` reader"]
pub type R = crate::R<Prictrl0Spec>;
#[doc = "Register `PRICTRL0` writer"]
pub type W = crate::W<Prictrl0Spec>;
#[doc = "Field `LVLPRI0` reader - Level 0 Channel Priority Number"]
pub type Lvlpri0R = crate::FieldReader;
#[doc = "Field `LVLPRI0` writer - Level 0 Channel Priority Number"]
pub type Lvlpri0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Level 0 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos0select {
    #[doc = "0: Regular delivery"]
    Regular = 0,
    #[doc = "1: Bandwidth shortage"]
    Shortage = 1,
    #[doc = "2: Latency sensitive"]
    Sensitive = 2,
    #[doc = "3: Latency critical"]
    Critical = 3,
}
impl From<Qos0select> for u8 {
    #[inline(always)]
    fn from(variant: Qos0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos0select {
    type Ux = u8;
}
impl crate::IsEnum for Qos0select {}
#[doc = "Field `QOS0` reader - Level 0 Quality of Service"]
pub type Qos0R = crate::FieldReader<Qos0select>;
impl Qos0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos0select {
        match self.bits {
            0 => Qos0select::Regular,
            1 => Qos0select::Shortage,
            2 => Qos0select::Sensitive,
            3 => Qos0select::Critical,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Qos0select::Regular
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == Qos0select::Shortage
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == Qos0select::Sensitive
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == Qos0select::Critical
    }
}
#[doc = "Field `QOS0` writer - Level 0 Quality of Service"]
pub type Qos0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos0select, crate::Safe>;
impl<'a, REG> Qos0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Qos0select::Regular)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(Qos0select::Shortage)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Qos0select::Sensitive)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(Qos0select::Critical)
    }
}
#[doc = "Field `RRLVLEN0` reader - Level 0 Round-Robin Scheduling Enable"]
pub type Rrlvlen0R = crate::BitReader;
#[doc = "Field `RRLVLEN0` writer - Level 0 Round-Robin Scheduling Enable"]
pub type Rrlvlen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI1` reader - Level 1 Channel Priority Number"]
pub type Lvlpri1R = crate::FieldReader;
#[doc = "Field `LVLPRI1` writer - Level 1 Channel Priority Number"]
pub type Lvlpri1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Level 1 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos1select {
    #[doc = "0: Regular delivery"]
    Regular = 0,
    #[doc = "1: Bandwidth shortage"]
    Shortage = 1,
    #[doc = "2: Latency sensitive"]
    Sensitive = 2,
    #[doc = "3: Latency critical"]
    Critical = 3,
}
impl From<Qos1select> for u8 {
    #[inline(always)]
    fn from(variant: Qos1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos1select {
    type Ux = u8;
}
impl crate::IsEnum for Qos1select {}
#[doc = "Field `QOS1` reader - Level 1 Quality of Service"]
pub type Qos1R = crate::FieldReader<Qos1select>;
impl Qos1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos1select {
        match self.bits {
            0 => Qos1select::Regular,
            1 => Qos1select::Shortage,
            2 => Qos1select::Sensitive,
            3 => Qos1select::Critical,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Qos1select::Regular
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == Qos1select::Shortage
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == Qos1select::Sensitive
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == Qos1select::Critical
    }
}
#[doc = "Field `QOS1` writer - Level 1 Quality of Service"]
pub type Qos1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos1select, crate::Safe>;
impl<'a, REG> Qos1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Regular)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Shortage)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Sensitive)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Critical)
    }
}
#[doc = "Field `RRLVLEN1` reader - Level 1 Round-Robin Scheduling Enable"]
pub type Rrlvlen1R = crate::BitReader;
#[doc = "Field `RRLVLEN1` writer - Level 1 Round-Robin Scheduling Enable"]
pub type Rrlvlen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI2` reader - Level 2 Channel Priority Number"]
pub type Lvlpri2R = crate::FieldReader;
#[doc = "Field `LVLPRI2` writer - Level 2 Channel Priority Number"]
pub type Lvlpri2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Level 2 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos2select {
    #[doc = "0: Regular delivery"]
    Regular = 0,
    #[doc = "1: Bandwidth shortage"]
    Shortage = 1,
    #[doc = "2: Latency sensitive"]
    Sensitive = 2,
    #[doc = "3: Latency critical"]
    Critical = 3,
}
impl From<Qos2select> for u8 {
    #[inline(always)]
    fn from(variant: Qos2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos2select {
    type Ux = u8;
}
impl crate::IsEnum for Qos2select {}
#[doc = "Field `QOS2` reader - Level 2 Quality of Service"]
pub type Qos2R = crate::FieldReader<Qos2select>;
impl Qos2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos2select {
        match self.bits {
            0 => Qos2select::Regular,
            1 => Qos2select::Shortage,
            2 => Qos2select::Sensitive,
            3 => Qos2select::Critical,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Qos2select::Regular
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == Qos2select::Shortage
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == Qos2select::Sensitive
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == Qos2select::Critical
    }
}
#[doc = "Field `QOS2` writer - Level 2 Quality of Service"]
pub type Qos2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos2select, crate::Safe>;
impl<'a, REG> Qos2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Regular)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Shortage)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Sensitive)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Critical)
    }
}
#[doc = "Field `RRLVLEN2` reader - Level 2 Round-Robin Scheduling Enable"]
pub type Rrlvlen2R = crate::BitReader;
#[doc = "Field `RRLVLEN2` writer - Level 2 Round-Robin Scheduling Enable"]
pub type Rrlvlen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLPRI3` reader - Level 3 Channel Priority Number"]
pub type Lvlpri3R = crate::FieldReader;
#[doc = "Field `LVLPRI3` writer - Level 3 Channel Priority Number"]
pub type Lvlpri3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Level 3 Quality of Service\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos3select {
    #[doc = "0: Regular delivery"]
    Regular = 0,
    #[doc = "1: Bandwidth shortage"]
    Shortage = 1,
    #[doc = "2: Latency sensitive"]
    Sensitive = 2,
    #[doc = "3: Latency critical"]
    Critical = 3,
}
impl From<Qos3select> for u8 {
    #[inline(always)]
    fn from(variant: Qos3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos3select {
    type Ux = u8;
}
impl crate::IsEnum for Qos3select {}
#[doc = "Field `QOS3` reader - Level 3 Quality of Service"]
pub type Qos3R = crate::FieldReader<Qos3select>;
impl Qos3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos3select {
        match self.bits {
            0 => Qos3select::Regular,
            1 => Qos3select::Shortage,
            2 => Qos3select::Sensitive,
            3 => Qos3select::Critical,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Qos3select::Regular
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn is_shortage(&self) -> bool {
        *self == Qos3select::Shortage
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn is_sensitive(&self) -> bool {
        *self == Qos3select::Sensitive
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn is_critical(&self) -> bool {
        *self == Qos3select::Critical
    }
}
#[doc = "Field `QOS3` writer - Level 3 Quality of Service"]
pub type Qos3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos3select, crate::Safe>;
impl<'a, REG> Qos3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular delivery"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Regular)
    }
    #[doc = "Bandwidth shortage"]
    #[inline(always)]
    pub fn shortage(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Shortage)
    }
    #[doc = "Latency sensitive"]
    #[inline(always)]
    pub fn sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Sensitive)
    }
    #[doc = "Latency critical"]
    #[inline(always)]
    pub fn critical(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Critical)
    }
}
#[doc = "Field `RRLVLEN3` reader - Level 3 Round-Robin Scheduling Enable"]
pub type Rrlvlen3R = crate::BitReader;
#[doc = "Field `RRLVLEN3` writer - Level 3 Round-Robin Scheduling Enable"]
pub type Rrlvlen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&self) -> Lvlpri0R {
        Lvlpri0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    pub fn qos0(&self) -> Qos0R {
        Qos0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&self) -> Rrlvlen0R {
        Rrlvlen0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&self) -> Lvlpri1R {
        Lvlpri1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    pub fn qos1(&self) -> Qos1R {
        Qos1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&self) -> Rrlvlen1R {
        Rrlvlen1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&self) -> Lvlpri2R {
        Lvlpri2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    pub fn qos2(&self) -> Qos2R {
        Qos2R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&self) -> Rrlvlen2R {
        Rrlvlen2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&self) -> Lvlpri3R {
        Lvlpri3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    pub fn qos3(&self) -> Qos3R {
        Qos3R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&self) -> Rrlvlen3R {
        Rrlvlen3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Level 0 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri0(&mut self) -> Lvlpri0W<Prictrl0Spec> {
        Lvlpri0W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Level 0 Quality of Service"]
    #[inline(always)]
    pub fn qos0(&mut self) -> Qos0W<Prictrl0Spec> {
        Qos0W::new(self, 5)
    }
    #[doc = "Bit 7 - Level 0 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen0(&mut self) -> Rrlvlen0W<Prictrl0Spec> {
        Rrlvlen0W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Level 1 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri1(&mut self) -> Lvlpri1W<Prictrl0Spec> {
        Lvlpri1W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Level 1 Quality of Service"]
    #[inline(always)]
    pub fn qos1(&mut self) -> Qos1W<Prictrl0Spec> {
        Qos1W::new(self, 13)
    }
    #[doc = "Bit 15 - Level 1 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen1(&mut self) -> Rrlvlen1W<Prictrl0Spec> {
        Rrlvlen1W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Level 2 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri2(&mut self) -> Lvlpri2W<Prictrl0Spec> {
        Lvlpri2W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Level 2 Quality of Service"]
    #[inline(always)]
    pub fn qos2(&mut self) -> Qos2W<Prictrl0Spec> {
        Qos2W::new(self, 21)
    }
    #[doc = "Bit 23 - Level 2 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen2(&mut self) -> Rrlvlen2W<Prictrl0Spec> {
        Rrlvlen2W::new(self, 23)
    }
    #[doc = "Bits 24:28 - Level 3 Channel Priority Number"]
    #[inline(always)]
    pub fn lvlpri3(&mut self) -> Lvlpri3W<Prictrl0Spec> {
        Lvlpri3W::new(self, 24)
    }
    #[doc = "Bits 29:30 - Level 3 Quality of Service"]
    #[inline(always)]
    pub fn qos3(&mut self) -> Qos3W<Prictrl0Spec> {
        Qos3W::new(self, 29)
    }
    #[doc = "Bit 31 - Level 3 Round-Robin Scheduling Enable"]
    #[inline(always)]
    pub fn rrlvlen3(&mut self) -> Rrlvlen3W<Prictrl0Spec> {
        Rrlvlen3W::new(self, 31)
    }
}
#[doc = "Priority Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`prictrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prictrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prictrl0Spec;
impl crate::RegisterSpec for Prictrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prictrl0::R`](R) reader structure"]
impl crate::Readable for Prictrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`prictrl0::W`](W) writer structure"]
impl crate::Writable for Prictrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRICTRL0 to value 0x4040_4040"]
impl crate::Resettable for Prictrl0Spec {
    const RESET_VALUE: u32 = 0x4040_4040;
}
