#[doc = "Register `TAMPCTRL` reader"]
pub type R = crate::R<TampctrlSpec>;
#[doc = "Register `TAMPCTRL` writer"]
pub type W = crate::W<TampctrlSpec>;
#[doc = "Tamper Input 0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In0actselect {
    #[doc = "0: Off (Disabled)"]
    Off = 0,
    #[doc = "1: Wake without timestamp"]
    Wake = 1,
    #[doc = "2: Capture timestamp"]
    Capture = 2,
    #[doc = "3: Compare IN0 to OUT"]
    Actl = 3,
}
impl From<In0actselect> for u8 {
    #[inline(always)]
    fn from(variant: In0actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In0actselect {
    type Ux = u8;
}
impl crate::IsEnum for In0actselect {}
#[doc = "Field `IN0ACT` reader - Tamper Input 0 Action"]
pub type In0actR = crate::FieldReader<In0actselect>;
impl In0actR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In0actselect {
        match self.bits {
            0 => In0actselect::Off,
            1 => In0actselect::Wake,
            2 => In0actselect::Capture,
            3 => In0actselect::Actl,
            _ => unreachable!(),
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == In0actselect::Off
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == In0actselect::Wake
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == In0actselect::Capture
    }
    #[doc = "Compare IN0 to OUT"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == In0actselect::Actl
    }
}
#[doc = "Field `IN0ACT` writer - Tamper Input 0 Action"]
pub type In0actW<'a, REG> = crate::FieldWriter<'a, REG, 2, In0actselect, crate::Safe>;
impl<'a, REG> In0actW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(In0actselect::Off)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(In0actselect::Wake)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(In0actselect::Capture)
    }
    #[doc = "Compare IN0 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut crate::W<REG> {
        self.variant(In0actselect::Actl)
    }
}
#[doc = "Tamper Input 1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In1actselect {
    #[doc = "0: Off (Disabled)"]
    Off = 0,
    #[doc = "1: Wake without timestamp"]
    Wake = 1,
    #[doc = "2: Capture timestamp"]
    Capture = 2,
    #[doc = "3: Compare IN1 to OUT"]
    Actl = 3,
}
impl From<In1actselect> for u8 {
    #[inline(always)]
    fn from(variant: In1actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In1actselect {
    type Ux = u8;
}
impl crate::IsEnum for In1actselect {}
#[doc = "Field `IN1ACT` reader - Tamper Input 1 Action"]
pub type In1actR = crate::FieldReader<In1actselect>;
impl In1actR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In1actselect {
        match self.bits {
            0 => In1actselect::Off,
            1 => In1actselect::Wake,
            2 => In1actselect::Capture,
            3 => In1actselect::Actl,
            _ => unreachable!(),
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == In1actselect::Off
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == In1actselect::Wake
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == In1actselect::Capture
    }
    #[doc = "Compare IN1 to OUT"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == In1actselect::Actl
    }
}
#[doc = "Field `IN1ACT` writer - Tamper Input 1 Action"]
pub type In1actW<'a, REG> = crate::FieldWriter<'a, REG, 2, In1actselect, crate::Safe>;
impl<'a, REG> In1actW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(In1actselect::Off)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(In1actselect::Wake)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(In1actselect::Capture)
    }
    #[doc = "Compare IN1 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut crate::W<REG> {
        self.variant(In1actselect::Actl)
    }
}
#[doc = "Tamper Input 2 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In2actselect {
    #[doc = "0: Off (Disabled)"]
    Off = 0,
    #[doc = "1: Wake without timestamp"]
    Wake = 1,
    #[doc = "2: Capture timestamp"]
    Capture = 2,
    #[doc = "3: Compare IN2 to OUT"]
    Actl = 3,
}
impl From<In2actselect> for u8 {
    #[inline(always)]
    fn from(variant: In2actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In2actselect {
    type Ux = u8;
}
impl crate::IsEnum for In2actselect {}
#[doc = "Field `IN2ACT` reader - Tamper Input 2 Action"]
pub type In2actR = crate::FieldReader<In2actselect>;
impl In2actR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In2actselect {
        match self.bits {
            0 => In2actselect::Off,
            1 => In2actselect::Wake,
            2 => In2actselect::Capture,
            3 => In2actselect::Actl,
            _ => unreachable!(),
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == In2actselect::Off
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == In2actselect::Wake
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == In2actselect::Capture
    }
    #[doc = "Compare IN2 to OUT"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == In2actselect::Actl
    }
}
#[doc = "Field `IN2ACT` writer - Tamper Input 2 Action"]
pub type In2actW<'a, REG> = crate::FieldWriter<'a, REG, 2, In2actselect, crate::Safe>;
impl<'a, REG> In2actW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(In2actselect::Off)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(In2actselect::Wake)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(In2actselect::Capture)
    }
    #[doc = "Compare IN2 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut crate::W<REG> {
        self.variant(In2actselect::Actl)
    }
}
#[doc = "Tamper Input 3 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In3actselect {
    #[doc = "0: Off (Disabled)"]
    Off = 0,
    #[doc = "1: Wake without timestamp"]
    Wake = 1,
    #[doc = "2: Capture timestamp"]
    Capture = 2,
    #[doc = "3: Compare IN3 to OUT"]
    Actl = 3,
}
impl From<In3actselect> for u8 {
    #[inline(always)]
    fn from(variant: In3actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In3actselect {
    type Ux = u8;
}
impl crate::IsEnum for In3actselect {}
#[doc = "Field `IN3ACT` reader - Tamper Input 3 Action"]
pub type In3actR = crate::FieldReader<In3actselect>;
impl In3actR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In3actselect {
        match self.bits {
            0 => In3actselect::Off,
            1 => In3actselect::Wake,
            2 => In3actselect::Capture,
            3 => In3actselect::Actl,
            _ => unreachable!(),
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == In3actselect::Off
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == In3actselect::Wake
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == In3actselect::Capture
    }
    #[doc = "Compare IN3 to OUT"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == In3actselect::Actl
    }
}
#[doc = "Field `IN3ACT` writer - Tamper Input 3 Action"]
pub type In3actW<'a, REG> = crate::FieldWriter<'a, REG, 2, In3actselect, crate::Safe>;
impl<'a, REG> In3actW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(In3actselect::Off)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(In3actselect::Wake)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(In3actselect::Capture)
    }
    #[doc = "Compare IN3 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut crate::W<REG> {
        self.variant(In3actselect::Actl)
    }
}
#[doc = "Tamper Input 4 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum In4actselect {
    #[doc = "0: Off (Disabled)"]
    Off = 0,
    #[doc = "1: Wake without timestamp"]
    Wake = 1,
    #[doc = "2: Capture timestamp"]
    Capture = 2,
    #[doc = "3: Compare IN4 to OUT"]
    Actl = 3,
}
impl From<In4actselect> for u8 {
    #[inline(always)]
    fn from(variant: In4actselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for In4actselect {
    type Ux = u8;
}
impl crate::IsEnum for In4actselect {}
#[doc = "Field `IN4ACT` reader - Tamper Input 4 Action"]
pub type In4actR = crate::FieldReader<In4actselect>;
impl In4actR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In4actselect {
        match self.bits {
            0 => In4actselect::Off,
            1 => In4actselect::Wake,
            2 => In4actselect::Capture,
            3 => In4actselect::Actl,
            _ => unreachable!(),
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == In4actselect::Off
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == In4actselect::Wake
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == In4actselect::Capture
    }
    #[doc = "Compare IN4 to OUT"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == In4actselect::Actl
    }
}
#[doc = "Field `IN4ACT` writer - Tamper Input 4 Action"]
pub type In4actW<'a, REG> = crate::FieldWriter<'a, REG, 2, In4actselect, crate::Safe>;
impl<'a, REG> In4actW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(In4actselect::Off)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(In4actselect::Wake)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(In4actselect::Capture)
    }
    #[doc = "Compare IN4 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut crate::W<REG> {
        self.variant(In4actselect::Actl)
    }
}
#[doc = "Field `TAMLVL0` reader - Tamper Level Select 0"]
pub type Tamlvl0R = crate::BitReader;
#[doc = "Field `TAMLVL0` writer - Tamper Level Select 0"]
pub type Tamlvl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMLVL1` reader - Tamper Level Select 1"]
pub type Tamlvl1R = crate::BitReader;
#[doc = "Field `TAMLVL1` writer - Tamper Level Select 1"]
pub type Tamlvl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMLVL2` reader - Tamper Level Select 2"]
pub type Tamlvl2R = crate::BitReader;
#[doc = "Field `TAMLVL2` writer - Tamper Level Select 2"]
pub type Tamlvl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMLVL3` reader - Tamper Level Select 3"]
pub type Tamlvl3R = crate::BitReader;
#[doc = "Field `TAMLVL3` writer - Tamper Level Select 3"]
pub type Tamlvl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMLVL4` reader - Tamper Level Select 4"]
pub type Tamlvl4R = crate::BitReader;
#[doc = "Field `TAMLVL4` writer - Tamper Level Select 4"]
pub type Tamlvl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBNC0` reader - Debouncer Enable 0"]
pub type Debnc0R = crate::BitReader;
#[doc = "Field `DEBNC0` writer - Debouncer Enable 0"]
pub type Debnc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBNC1` reader - Debouncer Enable 1"]
pub type Debnc1R = crate::BitReader;
#[doc = "Field `DEBNC1` writer - Debouncer Enable 1"]
pub type Debnc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBNC2` reader - Debouncer Enable 2"]
pub type Debnc2R = crate::BitReader;
#[doc = "Field `DEBNC2` writer - Debouncer Enable 2"]
pub type Debnc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBNC3` reader - Debouncer Enable 3"]
pub type Debnc3R = crate::BitReader;
#[doc = "Field `DEBNC3` writer - Debouncer Enable 3"]
pub type Debnc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBNC4` reader - Debouncer Enable 4"]
pub type Debnc4R = crate::BitReader;
#[doc = "Field `DEBNC4` writer - Debouncer Enable 4"]
pub type Debnc4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    pub fn in0act(&self) -> In0actR {
        In0actR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    pub fn in1act(&self) -> In1actR {
        In1actR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    pub fn in2act(&self) -> In2actR {
        In2actR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    pub fn in3act(&self) -> In3actR {
        In3actR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    pub fn in4act(&self) -> In4actR {
        In4actR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    pub fn tamlvl0(&self) -> Tamlvl0R {
        Tamlvl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    pub fn tamlvl1(&self) -> Tamlvl1R {
        Tamlvl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    pub fn tamlvl2(&self) -> Tamlvl2R {
        Tamlvl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    pub fn tamlvl3(&self) -> Tamlvl3R {
        Tamlvl3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    pub fn tamlvl4(&self) -> Tamlvl4R {
        Tamlvl4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    pub fn debnc0(&self) -> Debnc0R {
        Debnc0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    pub fn debnc1(&self) -> Debnc1R {
        Debnc1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    pub fn debnc2(&self) -> Debnc2R {
        Debnc2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    pub fn debnc3(&self) -> Debnc3R {
        Debnc3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    pub fn debnc4(&self) -> Debnc4R {
        Debnc4R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    pub fn in0act(&mut self) -> In0actW<TampctrlSpec> {
        In0actW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    pub fn in1act(&mut self) -> In1actW<TampctrlSpec> {
        In1actW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    pub fn in2act(&mut self) -> In2actW<TampctrlSpec> {
        In2actW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    pub fn in3act(&mut self) -> In3actW<TampctrlSpec> {
        In3actW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    pub fn in4act(&mut self) -> In4actW<TampctrlSpec> {
        In4actW::new(self, 8)
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    pub fn tamlvl0(&mut self) -> Tamlvl0W<TampctrlSpec> {
        Tamlvl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    pub fn tamlvl1(&mut self) -> Tamlvl1W<TampctrlSpec> {
        Tamlvl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    pub fn tamlvl2(&mut self) -> Tamlvl2W<TampctrlSpec> {
        Tamlvl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    pub fn tamlvl3(&mut self) -> Tamlvl3W<TampctrlSpec> {
        Tamlvl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    pub fn tamlvl4(&mut self) -> Tamlvl4W<TampctrlSpec> {
        Tamlvl4W::new(self, 20)
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    pub fn debnc0(&mut self) -> Debnc0W<TampctrlSpec> {
        Debnc0W::new(self, 24)
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    pub fn debnc1(&mut self) -> Debnc1W<TampctrlSpec> {
        Debnc1W::new(self, 25)
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    pub fn debnc2(&mut self) -> Debnc2W<TampctrlSpec> {
        Debnc2W::new(self, 26)
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    pub fn debnc3(&mut self) -> Debnc3W<TampctrlSpec> {
        Debnc3W::new(self, 27)
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    pub fn debnc4(&mut self) -> Debnc4W<TampctrlSpec> {
        Debnc4W::new(self, 28)
    }
}
#[doc = "Tamper Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tampctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampctrlSpec;
impl crate::RegisterSpec for TampctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampctrl::R`](R) reader structure"]
impl crate::Readable for TampctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tampctrl::W`](W) writer structure"]
impl crate::Writable for TampctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAMPCTRL to value 0"]
impl crate::Resettable for TampctrlSpec {}
