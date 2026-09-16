#[doc = "Register `WRCTRL` reader"]
pub type R = crate::R<WrctrlSpec>;
#[doc = "Register `WRCTRL` writer"]
pub type W = crate::W<WrctrlSpec>;
#[doc = "Field `PERID` reader - Peripheral identifier"]
pub type PeridR = crate::FieldReader<u16>;
#[doc = "Field `PERID` writer - Peripheral identifier"]
pub type PeridW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Peripheral access control key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "0: No action"]
    Off = 0,
    #[doc = "1: Clear protection"]
    Clr = 1,
    #[doc = "2: Set protection"]
    Set = 2,
    #[doc = "3: Set and lock protection"]
    Setlck = 3,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` reader - Peripheral access control key"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            0 => Some(Keyselect::Off),
            1 => Some(Keyselect::Clr),
            2 => Some(Keyselect::Set),
            3 => Some(Keyselect::Setlck),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Keyselect::Off
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Keyselect::Clr
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Keyselect::Set
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn is_setlck(&self) -> bool {
        *self == Keyselect::Setlck
    }
}
#[doc = "Field `KEY` writer - Peripheral access control key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Off)
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Clr)
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Set)
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn setlck(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Setlck)
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PeridR {
        PeridR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn perid(&mut self) -> PeridW<WrctrlSpec> {
        PeridW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<WrctrlSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Write control\n\nYou can [`read`](crate::Reg::read) this register and get [`wrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrctrlSpec;
impl crate::RegisterSpec for WrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrctrl::R`](R) reader structure"]
impl crate::Readable for WrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wrctrl::W`](W) writer structure"]
impl crate::Writable for WrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
