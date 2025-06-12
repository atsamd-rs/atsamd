#[doc = "Register `CACR` reader"]
pub type R = crate::R<CacrSpec>;
#[doc = "Register `CACR` writer"]
pub type W = crate::W<CacrSpec>;
#[doc = "Field `CAPWREN` reader - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CapwrenR = crate::BitReader;
#[doc = "Field `CAPWREN` writer - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
pub type CapwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Key (0x46)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "70: Key"]
    Key = 70,
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
#[doc = "Field `KEY` reader - Key (0x46)"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            70 => Some(Keyselect::Key),
            _ => None,
        }
    }
    #[doc = "Key"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == Keyselect::Key
    }
}
#[doc = "Field `KEY` writer - Key (0x46)"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Key)
    }
}
impl R {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&self) -> CapwrenR {
        CapwrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capabilities Registers Write Enable (Required to write the correct frequencies in the Capabilities Registers)"]
    #[inline(always)]
    pub fn capwren(&mut self) -> CapwrenW<CacrSpec> {
        CapwrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key (0x46)"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CacrSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Capabilities Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacrSpec;
impl crate::RegisterSpec for CacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CacrSpec {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CacrSpec {}
