#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `PID0` reader - PORT Event Pin Identifier 0"]
pub type Pid0R = crate::FieldReader;
#[doc = "Field `PID0` writer - PORT Event Pin Identifier 0"]
pub type Pid0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "PORT Event Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evact0select {
    #[doc = "0: Event output to pin"]
    Out = 0,
    #[doc = "1: Set output register of pin on event"]
    Set = 1,
    #[doc = "2: Clear output register of pin on event"]
    Clr = 2,
    #[doc = "3: Toggle output register of pin on event"]
    Tgl = 3,
}
impl From<Evact0select> for u8 {
    #[inline(always)]
    fn from(variant: Evact0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evact0select {
    type Ux = u8;
}
impl crate::IsEnum for Evact0select {}
#[doc = "Field `EVACT0` reader - PORT Event Action 0"]
pub type Evact0R = crate::FieldReader<Evact0select>;
impl Evact0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evact0select {
        match self.bits {
            0 => Evact0select::Out,
            1 => Evact0select::Set,
            2 => Evact0select::Clr,
            3 => Evact0select::Tgl,
            _ => unreachable!(),
        }
    }
    #[doc = "Event output to pin"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Evact0select::Out
    }
    #[doc = "Set output register of pin on event"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Evact0select::Set
    }
    #[doc = "Clear output register of pin on event"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Evact0select::Clr
    }
    #[doc = "Toggle output register of pin on event"]
    #[inline(always)]
    pub fn is_tgl(&self) -> bool {
        *self == Evact0select::Tgl
    }
}
#[doc = "Field `EVACT0` writer - PORT Event Action 0"]
pub type Evact0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Evact0select, crate::Safe>;
impl<'a, REG> Evact0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event output to pin"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Out)
    }
    #[doc = "Set output register of pin on event"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Set)
    }
    #[doc = "Clear output register of pin on event"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Clr)
    }
    #[doc = "Toggle output register of pin on event"]
    #[inline(always)]
    pub fn tgl(self) -> &'a mut crate::W<REG> {
        self.variant(Evact0select::Tgl)
    }
}
#[doc = "Field `PORTEI0` reader - PORT Event Input Enable 0"]
pub type Portei0R = crate::BitReader;
#[doc = "Field `PORTEI0` writer - PORT Event Input Enable 0"]
pub type Portei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID1` reader - PORT Event Pin Identifier 1"]
pub type Pid1R = crate::FieldReader;
#[doc = "Field `PID1` writer - PORT Event Pin Identifier 1"]
pub type Pid1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EVACT1` reader - PORT Event Action 1"]
pub type Evact1R = crate::FieldReader;
#[doc = "Field `EVACT1` writer - PORT Event Action 1"]
pub type Evact1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PORTEI1` reader - PORT Event Input Enable 1"]
pub type Portei1R = crate::BitReader;
#[doc = "Field `PORTEI1` writer - PORT Event Input Enable 1"]
pub type Portei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID2` reader - PORT Event Pin Identifier 2"]
pub type Pid2R = crate::FieldReader;
#[doc = "Field `PID2` writer - PORT Event Pin Identifier 2"]
pub type Pid2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EVACT2` reader - PORT Event Action 2"]
pub type Evact2R = crate::FieldReader;
#[doc = "Field `EVACT2` writer - PORT Event Action 2"]
pub type Evact2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PORTEI2` reader - PORT Event Input Enable 2"]
pub type Portei2R = crate::BitReader;
#[doc = "Field `PORTEI2` writer - PORT Event Input Enable 2"]
pub type Portei2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID3` reader - PORT Event Pin Identifier 3"]
pub type Pid3R = crate::FieldReader;
#[doc = "Field `PID3` writer - PORT Event Pin Identifier 3"]
pub type Pid3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EVACT3` reader - PORT Event Action 3"]
pub type Evact3R = crate::FieldReader;
#[doc = "Field `EVACT3` writer - PORT Event Action 3"]
pub type Evact3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PORTEI3` reader - PORT Event Input Enable 3"]
pub type Portei3R = crate::BitReader;
#[doc = "Field `PORTEI3` writer - PORT Event Input Enable 3"]
pub type Portei3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline(always)]
    pub fn pid0(&self) -> Pid0R {
        Pid0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline(always)]
    pub fn evact0(&self) -> Evact0R {
        Evact0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline(always)]
    pub fn portei0(&self) -> Portei0R {
        Portei0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline(always)]
    pub fn pid1(&self) -> Pid1R {
        Pid1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline(always)]
    pub fn evact1(&self) -> Evact1R {
        Evact1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline(always)]
    pub fn portei1(&self) -> Portei1R {
        Portei1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline(always)]
    pub fn pid2(&self) -> Pid2R {
        Pid2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline(always)]
    pub fn evact2(&self) -> Evact2R {
        Evact2R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline(always)]
    pub fn portei2(&self) -> Portei2R {
        Portei2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline(always)]
    pub fn pid3(&self) -> Pid3R {
        Pid3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline(always)]
    pub fn evact3(&self) -> Evact3R {
        Evact3R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline(always)]
    pub fn portei3(&self) -> Portei3R {
        Portei3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PORT Event Pin Identifier 0"]
    #[inline(always)]
    #[must_use]
    pub fn pid0(&mut self) -> Pid0W<EvctrlSpec> {
        Pid0W::new(self, 0)
    }
    #[doc = "Bits 5:6 - PORT Event Action 0"]
    #[inline(always)]
    #[must_use]
    pub fn evact0(&mut self) -> Evact0W<EvctrlSpec> {
        Evact0W::new(self, 5)
    }
    #[doc = "Bit 7 - PORT Event Input Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn portei0(&mut self) -> Portei0W<EvctrlSpec> {
        Portei0W::new(self, 7)
    }
    #[doc = "Bits 8:12 - PORT Event Pin Identifier 1"]
    #[inline(always)]
    #[must_use]
    pub fn pid1(&mut self) -> Pid1W<EvctrlSpec> {
        Pid1W::new(self, 8)
    }
    #[doc = "Bits 13:14 - PORT Event Action 1"]
    #[inline(always)]
    #[must_use]
    pub fn evact1(&mut self) -> Evact1W<EvctrlSpec> {
        Evact1W::new(self, 13)
    }
    #[doc = "Bit 15 - PORT Event Input Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn portei1(&mut self) -> Portei1W<EvctrlSpec> {
        Portei1W::new(self, 15)
    }
    #[doc = "Bits 16:20 - PORT Event Pin Identifier 2"]
    #[inline(always)]
    #[must_use]
    pub fn pid2(&mut self) -> Pid2W<EvctrlSpec> {
        Pid2W::new(self, 16)
    }
    #[doc = "Bits 21:22 - PORT Event Action 2"]
    #[inline(always)]
    #[must_use]
    pub fn evact2(&mut self) -> Evact2W<EvctrlSpec> {
        Evact2W::new(self, 21)
    }
    #[doc = "Bit 23 - PORT Event Input Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn portei2(&mut self) -> Portei2W<EvctrlSpec> {
        Portei2W::new(self, 23)
    }
    #[doc = "Bits 24:28 - PORT Event Pin Identifier 3"]
    #[inline(always)]
    #[must_use]
    pub fn pid3(&mut self) -> Pid3W<EvctrlSpec> {
        Pid3W::new(self, 24)
    }
    #[doc = "Bits 29:30 - PORT Event Action 3"]
    #[inline(always)]
    #[must_use]
    pub fn evact3(&mut self) -> Evact3W<EvctrlSpec> {
        Evact3W::new(self, 29)
    }
    #[doc = "Bit 31 - PORT Event Input Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn portei3(&mut self) -> Portei3W<EvctrlSpec> {
        Portei3W::new(self, 31)
    }
}
#[doc = "Event Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u32 = 0;
}
