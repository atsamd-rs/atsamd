#[doc = "Register `PRESCBUF` reader"]
pub type R = crate::R<PrescbufSpec>;
#[doc = "Register `PRESCBUF` writer"]
pub type W = crate::W<PrescbufSpec>;
#[doc = "Prescaler Buffer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescbufselect {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 8"]
    Div8 = 3,
    #[doc = "4: Divide by 16"]
    Div16 = 4,
    #[doc = "5: Divide by 32"]
    Div32 = 5,
    #[doc = "6: Divide by 64"]
    Div64 = 6,
    #[doc = "7: Divide by 128"]
    Div128 = 7,
    #[doc = "8: Divide by 256"]
    Div256 = 8,
    #[doc = "9: Divide by 512"]
    Div512 = 9,
    #[doc = "10: Divide by 1024"]
    Div1024 = 10,
}
impl From<Prescbufselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescbufselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescbufselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescbufselect {}
#[doc = "Field `PRESCBUF` reader - Prescaler Buffer Value"]
pub type PrescbufR = crate::FieldReader<Prescbufselect>;
impl PrescbufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescbufselect> {
        match self.bits {
            0 => Some(Prescbufselect::Div1),
            1 => Some(Prescbufselect::Div2),
            2 => Some(Prescbufselect::Div4),
            3 => Some(Prescbufselect::Div8),
            4 => Some(Prescbufselect::Div16),
            5 => Some(Prescbufselect::Div32),
            6 => Some(Prescbufselect::Div64),
            7 => Some(Prescbufselect::Div128),
            8 => Some(Prescbufselect::Div256),
            9 => Some(Prescbufselect::Div512),
            10 => Some(Prescbufselect::Div1024),
            _ => None,
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescbufselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescbufselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescbufselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescbufselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescbufselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescbufselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescbufselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescbufselect::Div128
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescbufselect::Div256
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Prescbufselect::Div512
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Prescbufselect::Div1024
    }
}
#[doc = "Field `PRESCBUF` writer - Prescaler Buffer Value"]
pub type PrescbufW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prescbufselect>;
impl<'a, REG> PrescbufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div256)
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescbufselect::Div1024)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&self) -> PrescbufR {
        PrescbufR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&mut self) -> PrescbufW<PrescbufSpec> {
        PrescbufW::new(self, 0)
    }
}
#[doc = "Prescaler Buffer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`prescbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescbufSpec;
impl crate::RegisterSpec for PrescbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prescbuf::R`](R) reader structure"]
impl crate::Readable for PrescbufSpec {}
#[doc = "`write(|w| ..)` method takes [`prescbuf::W`](W) writer structure"]
impl crate::Writable for PrescbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESCBUF to value 0"]
impl crate::Resettable for PrescbufSpec {}
