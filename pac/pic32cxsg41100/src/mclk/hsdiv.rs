#[doc = "Register `HSDIV` reader"]
pub type R = crate::R<HsdivSpec>;
#[doc = "CPU Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divselect {
    #[doc = "1: Divide by 1"]
    Div1 = 1,
}
impl From<Divselect> for u8 {
    #[inline(always)]
    fn from(variant: Divselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divselect {
    type Ux = u8;
}
impl crate::IsEnum for Divselect {}
#[doc = "Field `DIV` reader - CPU Clock Division Factor"]
pub type DivR = crate::FieldReader<Divselect>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divselect> {
        match self.bits {
            1 => Some(Divselect::Div1),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divselect::Div1
    }
}
impl R {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(self.bits)
    }
}
#[doc = "HS Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`hsdiv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsdivSpec;
impl crate::RegisterSpec for HsdivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hsdiv::R`](R) reader structure"]
impl crate::Readable for HsdivSpec {}
#[doc = "`reset()` method sets HSDIV to value 0x01"]
impl crate::Resettable for HsdivSpec {
    const RESET_VALUE: u8 = 0x01;
}
