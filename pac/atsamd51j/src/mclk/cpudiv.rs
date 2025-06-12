#[doc = "Register `CPUDIV` reader"]
pub type R = crate::R<CpudivSpec>;
#[doc = "Register `CPUDIV` writer"]
pub type W = crate::W<CpudivSpec>;
#[doc = "Low-Power Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divselect {
    #[doc = "1: Divide by 1"]
    Div1 = 1,
    #[doc = "2: Divide by 2"]
    Div2 = 2,
    #[doc = "4: Divide by 4"]
    Div4 = 4,
    #[doc = "8: Divide by 8"]
    Div8 = 8,
    #[doc = "16: Divide by 16"]
    Div16 = 16,
    #[doc = "32: Divide by 32"]
    Div32 = 32,
    #[doc = "64: Divide by 64"]
    Div64 = 64,
    #[doc = "128: Divide by 128"]
    Div128 = 128,
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
#[doc = "Field `DIV` reader - Low-Power Clock Division Factor"]
pub type DivR = crate::FieldReader<Divselect>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divselect> {
        match self.bits {
            1 => Some(Divselect::Div1),
            2 => Some(Divselect::Div2),
            4 => Some(Divselect::Div4),
            8 => Some(Divselect::Div8),
            16 => Some(Divselect::Div16),
            32 => Some(Divselect::Div32),
            64 => Some(Divselect::Div64),
            128 => Some(Divselect::Div128),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Divselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Divselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Divselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Divselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Divselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Divselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Divselect::Div128
    }
}
#[doc = "Field `DIV` writer - Low-Power Clock Division Factor"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divselect>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<CpudivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "CPU Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpudivSpec;
impl crate::RegisterSpec for CpudivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpudiv::R`](R) reader structure"]
impl crate::Readable for CpudivSpec {}
#[doc = "`write(|w| ..)` method takes [`cpudiv::W`](W) writer structure"]
impl crate::Writable for CpudivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUDIV to value 0x01"]
impl crate::Resettable for CpudivSpec {
    const RESET_VALUE: u8 = 0x01;
}
